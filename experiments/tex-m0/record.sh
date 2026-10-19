#!/bin/bash
# Compile each sample once with the default ONLINE bundle, recording every file the engine opens.
set -u
cd "$(dirname "$0")"
BIN=${BIN:-target/release/tex-m0}
for t in a_article b_tikz c_natbib d_beamer e_fontspec f_biblatex; do
  rm -rf work/rec-$t work/out-rec-$t; mkdir -p work/rec-$t
  echo "== $t"
  $BIN online samples/$t.tex work/out-rec-$t --record work/rec-$t 2>&1 | tail -15
  echo "files: $(ls work/rec-$t | wc -l)"
done
