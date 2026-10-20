#!/bin/bash
# Offline compile of every sample from the directory bundle: cold (empty format cache), then warm.
# The network is cut with `unshare -rn` (new network namespace, no interface up).
cd "$(dirname "$0")"
BIN=${BIN:-target/release/tex-m0}
rm -rf work/off; mkdir -p work/off
echo "sample run result"
for s in a_article b_tikz c_natbib d_beamer e_fontspec f_biblatex g_math h_figs i_standalone j_babel k_algx l_unimath m_fonts; do
  rm -rf work/off/$s; mkdir -p work/off/$s
  for run in cold warm; do
    r=$(unshare -rn $BIN work/bundle samples/$s.tex work/off/$s/out --format-cache work/off/$s/fmt --synctex 2>/dev/null | grep ^RESULT | cut -c1-300)
    echo "$s $run $r"
  done
done
