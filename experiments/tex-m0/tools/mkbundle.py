"""Build the directory bundle and the pack sets from the recorded files (work/rec-*).

A pack set is a layer: a file belongs to the first pack (in the order below) whose sample
documents opened it. Run: python3 -I tools/mkbundle.py   (from experiments/tex-m0)
"""
import hashlib, os, shutil, subprocess, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
W = os.path.join(ROOT, "work")
PACKS = [
    ("Core", ["a_article", "e_fontspec", "l_unimath"]),
    ("Mathematics", ["g_math", "k_algx"]),
    ("Bibliography", ["c_natbib", "f_biblatex", "j_babel"]),
    ("Figures", ["b_tikz", "h_figs", "i_standalone"]),
    ("Slides", ["d_beamer"]),
    ("Fonts", ["m_fonts"]),
]
bundle = os.path.join(W, "bundle")
packs = os.path.join(W, "packs")
for d in (bundle, packs):
    shutil.rmtree(d, ignore_errors=True)
    os.makedirs(d)
seen = {}
for pack, samples in PACKS:
    pdir = os.path.join(packs, pack)
    os.makedirs(pdir)
    for s in samples:
        rd = os.path.join(W, "rec-" + s)
        for f in sorted(os.listdir(rd)):
            if f in seen:
                if open(os.path.join(rd, f), "rb").read() != open(os.path.join(bundle, f), "rb").read():
                    print("DIFFERENT CONTENT for", f, file=sys.stderr)
                continue
            seen[f] = pack
            shutil.copy(os.path.join(rd, f), os.path.join(pdir, f))
            shutil.copy(os.path.join(rd, f), os.path.join(bundle, f))
# the digest file Tectonic wants (any 64 hex characters that change with the content)
h = hashlib.sha256()
for f in sorted(os.listdir(bundle)):
    h.update(f.encode() + b"\0" + hashlib.sha256(open(os.path.join(bundle, f), "rb").read()).digest())
open(os.path.join(bundle, "SHA256SUM"), "w").write(h.hexdigest() + "\n")

def du(d):
    return sum(os.path.getsize(os.path.join(d, f)) for f in os.listdir(d))

def comp(d):
    tar = subprocess.Popen(["tar", "-C", d, "--sort=name", "-cf", "-", "."], stdout=subprocess.PIPE)
    data = tar.stdout.read()
    z = subprocess.run(["zstd", "-19", "-q", "-c"], input=data, capture_output=True).stdout
    x = subprocess.run(["xz", "-9e", "-c"], input=data, capture_output=True).stdout
    return len(z), len(x)

print("| Pack | files | raw MB | zstd -19 MB | xz -9e MB |")
print("|---|---|---|---|---|")
tot = [0, 0, 0, 0]
for pack, _ in PACKS:
    d = os.path.join(packs, pack)
    n, raw = len(os.listdir(d)), du(d)
    z, x = comp(d)
    for i, v in enumerate((n, raw, z, x)):
        tot[i] += v
    print(f"| {pack} | {n} | {raw/1e6:.2f} | {z/1e6:.2f} | {x/1e6:.2f} |")
print(f"| all (sum) | {tot[0]} | {tot[1]/1e6:.2f} | {tot[2]/1e6:.2f} | {tot[3]/1e6:.2f} |")
z, x = comp(bundle)
print(f"whole directory bundle in one archive: raw {du(bundle)/1e6:.2f} MB, zstd {z/1e6:.2f} MB, xz {x/1e6:.2f} MB")
# per-file membership list for the licence scan
with open(os.path.join(W, "membership.tsv"), "w") as m:
    for f, p in sorted(seen.items()):
        m.write(f"{p}\t{f}\t{os.path.getsize(os.path.join(bundle, f))}\n")
