"""Markdown table from results/linux-offline.txt and results/android-x86_64-emulator.txt.
Run: python3 -I tools/table.py   (from experiments/tex-m0)"""
import re
def load(p):
    d = {}
    for l in open(p):
        m = re.match(r'(\w+) (cold|warm) RESULT ok=(\d) ms=(\d+) hwm_kb=(\d+) pdf_bytes=(\d+) synctex_bytes=(\d+)', l)
        if m:
            d[(m[1], m[2])] = tuple(int(x) for x in m.groups()[2:])
    return d
L = load('results/linux-offline.txt'); A = load('results/android-x86_64-emulator.txt')
names = {'a_article': '(a) article: ams*, mathtools, graphicx, hyperref', 'b_tikz': '(b) TikZ + pgfplots',
 'c_natbib': '(c) BibTeX + natbib', 'd_beamer': '(d) beamer', 'e_fontspec': '(e) fontspec, Latin Modern',
 'f_biblatex': '(f) biblatex (backend=biber)', 'g_math': 'extra: physics, siunitx, tcolorbox, listings, algorithm2e',
 'h_figs': 'extra: subcaption, float, wrapfig, caption', 'i_standalone': 'extra: standalone class + tikz',
 'j_babel': 'extra: babel (French) + csquotes', 'k_algx': 'extra: algorithmicx',
 'l_unimath': 'extra: unicode-math + Latin Modern Math', 'm_fonts': 'extra: TeX Gyre fonts'}
print('| Document | OK | Linux cold / warm (s) | Linux peak RAM cold / warm (MB) | Android x86_64 cold / warm (s) | Android peak RAM cold / warm (MB) | PDF (KB) | SyncTeX (KB) |')
print('|---|---|---|---|---|---|---|---|')
for k, n in names.items():
    lc, lw, ac, aw = L[(k, 'cold')], L[(k, 'warm')], A[(k, 'cold')], A[(k, 'warm')]
    if not lc[0]:
        print(f'| {n} | NO | – | – | – | – | – | – |'); continue
    print(f"| {n} | yes | {lc[1]/1000:.1f} / {lw[1]/1000:.1f} | {lc[2]//1024} / {lw[2]//1024} | {ac[1]/1000:.1f} / {aw[1]/1000:.1f} | {ac[2]//1024} / {aw[2]//1024} | {lc[3]/1000:.0f} | {lc[4]/1000:.1f} |")
