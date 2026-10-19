#!/bin/bash
# Compile the samples ON the emulator/device from the directory bundle: cold (no format cache), then warm.
# Usage: ./run-android.sh x86_64 [adb-serial] [samples...]   (files pushed beforehand to /data/local/tmp/texm0)
cd "$(dirname "$0")"
ABI=${1:-x86_64}; SER=${2:-emulator-5554}; shift 2 2>/dev/null
SAMPLES=${*:-a_article b_tikz c_natbib d_beamer e_fontspec f_biblatex g_math h_figs i_standalone j_babel k_algx l_unimath m_fonts}
A="adb -s $SER shell"
for s in $SAMPLES; do
  $A "cd /data/local/tmp/texm0 && rm -rf off/$s && mkdir -p off/$s"
  for run in cold warm; do
    r=$($A "cd /data/local/tmp/texm0 && ./tex-m0-$ABI bundle samples/$s.tex off/$s/out --format-cache off/$s/fmt --synctex 2>/dev/null | grep ^RESULT" | tr -d '\r' | cut -c1-300)
    echo "$s $run $r"
  done
done
