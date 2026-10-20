//! M0 spike: compile one TeX file with the Tectonic engine from a bundle.
//!
//! tex-m0 <bundle> <input.tex> <outdir> [--format-cache DIR] [--synctex] [--record DIR]
//!   <bundle>  a directory (offline directory bundle) or the word `online`
//!   --record  with `online`: copy every file the engine asks the bundle for into DIR
//!
//! Prints one `RESULT` line: success, wall time, peak resident memory and output size.
#[cfg(feature = "online")]
use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;

#[cfg(feature = "online")]
use tectonic::config::PersistentConfig;
use tectonic::driver::{OutputFormat, PassSetting, ProcessingSessionBuilder};
#[cfg(feature = "online")]
use tectonic::io::{InputHandle, IoProvider, OpenResult};
use tectonic::status::termcolor::TermcolorStatusBackend;
use tectonic::status::ChatterLevel;
use tectonic_bundles::{dir::DirBundle, Bundle};

#[cfg(feature = "online")]
/// Wraps a bundle and saves a copy of every file the engine opens.
#[cfg(feature = "online")]
struct Recorder {
    inner: Box<dyn Bundle>,
    dir: PathBuf,
}

#[cfg(feature = "online")]
impl IoProvider for Recorder {
    fn input_open_name(
        &mut self,
        name: &str,
        status: &mut dyn tectonic::status::StatusBackend,
    ) -> OpenResult<InputHandle> {
        match self.inner.input_open_name(name, status) {
            OpenResult::Ok(mut h) => {
                let mut buf = Vec::new();
                if h.read_to_end(&mut buf).is_err() {
                    return OpenResult::NotAvailable;
                }
                let safe = name.replace('/', "__");
                let _ = std::fs::write(self.dir.join(&safe), &buf);
                OpenResult::Ok(InputHandle::new_read_only(
                    name,
                    std::io::Cursor::new(buf),
                    tectonic::io::InputOrigin::Other,
                ))
            }
            other => other,
        }
    }
}

#[cfg(feature = "online")]
impl Bundle for Recorder {
    fn get_digest(&mut self) -> tectonic::Result<tectonic::digest::DigestData> {
        self.inner.get_digest()
    }
    fn all_files(&self) -> Vec<String> {
        self.inner.all_files()
    }
}

fn hwm_kb() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VmHWM:"))
                .and_then(|l| l.split_whitespace().nth(1).and_then(|v| v.parse().ok()))
        })
        .unwrap_or(0)
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.len() < 3 {
        eprintln!("usage: tex-m0 <bundle|online> <input.tex> <outdir> [--format-cache DIR] [--synctex] [--record DIR]");
        std::process::exit(2);
    }
    let (bundle_arg, input, outdir) = (&a[0], PathBuf::from(&a[1]), PathBuf::from(&a[2]));
    let mut fmt_cache = outdir.join("fmt");
    let mut synctex = false;
    let mut record: Option<PathBuf> = None;
    let mut i = 3;
    while i < a.len() {
        match a[i].as_str() {
            "--synctex" => synctex = true,
            "--format-cache" => {
                i += 1;
                fmt_cache = PathBuf::from(&a[i]);
            }
            "--record" => {
                i += 1;
                record = Some(PathBuf::from(&a[i]));
            }
            x => panic!("unknown argument {x}"),
        }
        i += 1;
    }
    std::fs::create_dir_all(&outdir).unwrap();
    std::fs::create_dir_all(&fmt_cache).unwrap();

    let mut status = TermcolorStatusBackend::new(ChatterLevel::Minimal);
    let t0 = Instant::now();
    let res = run(bundle_arg, &input, &outdir, &fmt_cache, synctex, record, &mut status);
    let ms = t0.elapsed().as_millis();
    let stem = input.file_stem().unwrap().to_string_lossy().to_string();
    let pdf = std::fs::metadata(outdir.join(format!("{stem}.pdf"))).map(|m| m.len()).unwrap_or(0);
    let stx = std::fs::metadata(outdir.join(format!("{stem}.synctex.gz"))).map(|m| m.len()).unwrap_or(0);
    match res {
        Ok(()) => println!("RESULT ok=1 ms={ms} hwm_kb={} pdf_bytes={pdf} synctex_bytes={stx}", hwm_kb()),
        Err(e) => println!("RESULT ok=0 ms={ms} hwm_kb={} pdf_bytes={pdf} synctex_bytes={stx} error={e:#}", hwm_kb()),
    }
}

fn run(
    bundle_arg: &str,
    input: &PathBuf,
    outdir: &PathBuf,
    fmt_cache: &PathBuf,
    synctex: bool,
    record: Option<PathBuf>,
    status: &mut TermcolorStatusBackend,
) -> tectonic::Result<()> {
    #[cfg(not(feature = "online"))]
    let _ = &record;
    #[cfg(feature = "online")]
    let online: Option<Box<dyn Bundle>> = if bundle_arg == "online" {
        let cfg = PersistentConfig::open(false)?;
        let inner = cfg.default_bundle(false)?;
        Some(match record {
            Some(dir) => {
                std::fs::create_dir_all(&dir)?;
                Box::new(Recorder { inner, dir })
            }
            None => inner,
        })
    } else {
        None
    };
    #[cfg(not(feature = "online"))]
    let online: Option<Box<dyn Bundle>> = None;
    let mut b: Box<dyn Bundle> = online.unwrap_or_else(|| Box::new(DirBundle::new(bundle_arg)));
    let _ = b.get_digest(); // fail early when the bundle has no SHA256SUM
    let mut sb = ProcessingSessionBuilder::new_with_security(tectonic_bridge_core::SecuritySettings::new(
        tectonic_bridge_core::SecurityStance::DisableInsecures,
    ));
    let root = input.parent().map(|p| p.to_path_buf()).unwrap_or_default();
    sb.primary_input_path(input)
        .filesystem_root(if root.as_os_str().is_empty() { PathBuf::from(".") } else { root })
        .tex_input_name(&input.file_name().unwrap().to_string_lossy())
        .format_name("latex")
        .format_cache_path(fmt_cache)
        .output_dir(outdir)
        .output_format(OutputFormat::Pdf)
        .pass(PassSetting::Default)
        .keep_logs(true)
        .keep_intermediates(true)
        .synctex(synctex)
        .bundle(b);
    let mut sess = sb.create(status)?;
    sess.run(status)
}
