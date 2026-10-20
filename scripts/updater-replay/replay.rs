//! Replays an OLD release's updater (the `mahay-update` crate of its tag) against the live latest
//! release, as an installed app of that version would: discover the release (GitHub's API, then the
//! `releases/latest` redirect fallback), verify the signed checksums and download one APK.
//! Run by scripts/check-updater-replay.sh. Exits 1 on the first failure.
use mahay_update::*;

fn check(what: &str, ok: bool) -> bool {
    println!("  {} {what}", if ok { "ok  " } else { "FAIL" });
    ok
}

fn main() {
    let installed = std::env::args().nth(1).expect("installed version");
    let app = std::env::args().nth(2).unwrap_or_else(|| "tabilao".into());
    let mut all = true;
    // 1. Through GitHub's API.
    match latest_release() {
        Ok(r) => {
            all &= check(&format!("API: latest is {}, newer than {installed}", r.version), is_newer(&r.version, &installed));
            for abi in ["arm64-v8a", "x86_64"] {
                let name = asset_name(&app, &r.version, Target::Android(abi));
                all &= check(&format!("API: {name} is listed"), r.asset(&name).is_some());
            }
            all &= check("API: SHA256SUMS.txt and its signature are listed", r.asset(SUMS).is_some() && r.asset(SUMS_SIG).is_some());
            let dir = std::env::temp_dir().join(format!("updater-replay-{installed}"));
            match download(&r, &app, &installed, Target::Android("arm64-v8a"), &dir, &|_| {}) {
                Ok(d) => all &= check(&format!("API: downloaded and verified {}", d.path.file_name().and_then(|n| n.to_str()).unwrap_or("")), true),
                Err(e) => all &= check(&format!("API: download failed: {e:?}"), false),
            }
        }
        Err(e) => all &= check(&format!("API: latest_release failed: {e:?}"), false),
    }
    // 2. Without the API (rate limits on shared mobile networks): the names built from the redirect.
    let agent: ureq::Agent = ureq::Agent::config_builder().max_redirects(0).build().into();
    match agent.get("https://github.com/raantss18/mahay/releases/latest").call() {
        Ok(resp) => {
            let loc = resp.headers().get("location").and_then(|v| v.to_str().ok()).unwrap_or("").to_string();
            match tag_from_location(&loc) {
                Some(version) => {
                    let r = Release::from_version(&version, &APPS);
                    let dir = std::env::temp_dir().join(format!("updater-replay-fallback-{installed}"));
                    match download(&r, &app, &installed, Target::Android("arm64-v8a"), &dir, &|_| {}) {
                        Ok(_) => all &= check(&format!("redirect: {version} downloaded and verified"), true),
                        Err(e) => all &= check(&format!("redirect: download failed: {e:?}"), false),
                    }
                }
                None => all &= check(&format!("redirect: location not understood: {loc}"), false),
            }
        }
        Err(e) => all &= check(&format!("redirect: request failed: {e}"), false),
    }
    std::process::exit(if all { 0 } else { 1 });
}
