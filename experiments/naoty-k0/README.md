# naoty-k0: Naoty K0 spike (throwaway)

Question: can Naoty (Mahay's notebook, `docs/NEW_APPS.md`, D26) be a **native Jupyter client in
Rust** with a **Python kernel**, on Linux **and** Android? Results and decision: `docs/NEW_APPS.md`
(App 16) and `docs/DECISIONS.md` (D26). This directory is its own Cargo workspace, not part of the
suite; nothing here ships. Everything big is git-ignored (`.venv/`, `target/`, `build/`, `work/`).

```
client/   Rust Jupyter client (jupyter-zmq-client, jupyter-protocol, nbformat, jupyter-websocket-client)
          spawn <argv…>   start a kernel ({conn} = connection file), run 5 cells, print every message
          connect <json>  same, against a kernel that already runs
          ws <url> <tok>  same, through a `jupyter server` (token) with jupyter-websocket-client
          nbfmt <ipynb>   .ipynb read → write round trip, compared as JSON
kernel/   naoty-kernel: ~300 lines of Rust, embeds CPython (pyo3) and speaks the protocol over the
          pure-Rust `zeromq` crate (execute/kernel_info/shutdown, live stdout, last-expression
          result, errors, matplotlib figures as image/png); kernel.py is the Python side
stage-python.sh, stage-site.sh, build-kernel-android.sh, measure-sizes.sh, run-env.sh   see below
```

Needs: Rust (rustup toolchain with `x86_64-linux-android` and `aarch64-linux-android`), NDK
28.2.13676358 in `~/Android/Sdk`, `uv`, `zstd`, `adb`, a running x86_64 emulator.
`ANDROID_HOME` in the environment may point to a stale path: the scripts set
`ANDROID_HOME=ANDROID_SDK_ROOT=$HOME/Android/Sdk` themselves.

## 1–2. Linux

```bash
cd experiments/naoty-k0
uv venv --python 3.13 .venv
VIRTUAL_ENV=$PWD/.venv uv pip install ipykernel matplotlib numpy sympy jupyter_server pip
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=$PWD/target PYO3_PYTHON=$PWD/.venv/bin/python
cargo build --release -j 4
source ./run-env.sh        # LD_LIBRARY_PATH/PYTHONHOME for the embedding kernel (uv's libpython)
# 1. Rust client ↔ stock ipykernel
./target/release/naoty-k0-client spawn $PWD/.venv/bin/python -m ipykernel_launcher -f '{conn}'
# 2. Rust client ↔ our Rust kernel (pyo3 + zeromq)
./target/release/naoty-k0-client spawn ./target/release/naoty-kernel -f '{conn}'
# nbformat round trip
./target/release/naoty-k0-client nbfmt testdata/executed.ipynb     # made by testdata/mknb.py + nbconvert --execute
```

## 5. Remote kernel (jupyter server, token)

```bash
export JUPYTER_CONFIG_DIR=$PWD/work/jconf JUPYTER_DATA_DIR=$PWD/work/jdata JUPYTER_RUNTIME_DIR=$PWD/work/jrun
.venv/bin/python -m jupyter server --no-browser --ServerApp.token=k0token --ServerApp.port=18899 &
./target/release/naoty-k0-client ws http://127.0.0.1:18899 k0token
```

## 3. CPython 3.13 for Android (the official `Android/android.py`)

```bash
mkdir -p build && git clone --depth 1 --branch v3.13.16 https://github.com/python/cpython build/cpython
cd build/cpython/Android
sed -i 's/^ndk_version=.*/ndk_version=28.2.13676358/' android-env.sh   # it asks for 27.3 by default
sed -i 's/str(os.cpu_count())/"4"/' android.py                           # shared machine: -j 4
cd .. && export ANDROID_HOME=$HOME/Android/Sdk ANDROID_SDK_ROOT=$HOME/Android/Sdk
A=Android/android.py; $A configure-build; $A make-build
for h in x86_64-linux-android aarch64-linux-android; do $A configure-host $h; $A make-host $h; done
```

Notes: `android.py build HOST` alone fails (it wants `configure-build` first). Do not call
`android.py ci` (it deletes other NDK versions in the SDK). The script downloads bzip2, libffi,
openssl, sqlite and xz built for Android from `github.com/beeware/cpython-android-source-deps`.
Result: `build/cpython/cross-build/<host>/prefix`. Not built: `_curses`, `_dbm`, `_gdbm`,
`_multiprocessing`, `_posixshmem`, `_tkinter`, `_uuid`, `grp`, `readline`.

Scientific stack: **no numpy/matplotlib/Pillow wheel for Android exists on PyPI** (checked for
3.13 and 3.14). The Chaquopy index `https://chaquo.com/pypi-13.1` has `cp313` wheels
(`android_24_x86_64`, `android_24_arm64_v8a`) of numpy 1.26.2, matplotlib 3.8.4, Pillow 11.0.0,
kiwisolver, contourpy; they need the shared libraries of `chaquopy-openblas`,
`chaquopy-libgfortran`, `chaquopy-freetype`, `chaquopy-libjpeg` (same index) and the NDK's
`libc++_shared.so`. `sympy`, `mpmath` and the other dependencies are pure Python (PyPI).
`pyzmq` **does** have Android wheels on PyPI (27.2.0, cp313 and cp314, x86_64 and arm64).

```bash
# wheels (done by hand with pip download --platform android_24_<abi> --python-version 3.13 \
#   --implementation cp --abi cp313 --only-binary=:all: --no-deps --extra-index-url https://chaquo.com/pypi-13.1)
./stage-python.sh x86_64-linux-android work/pyroot-x86_64     # stripped runtime, tests removed
./stage-site.sh x86_64 work/site-x86_64                       # site-packages + work/site-x86_64-libs
./build-kernel-android.sh x86_64                              # kernel + client (also: aarch64)
A="adb -s emulator-5554"                                      # everything under /data/local/tmp/naoty-k0
$A push work/pyroot-x86_64 /data/local/tmp/naoty-k0/py; $A push work/site-x86_64 …/site; $A push work/site-x86_64-libs …/libs
$A push target/x86_64-linux-android/release/naoty-kernel target/x86_64-linux-android/release/naoty-k0-client …/
# env on the device (work/envrun.sh): PYTHONHOME=py LD_LIBRARY_PATH=py/lib:libs PYTHONPATH=site MPLBACKEND=Agg
$A shell '. /data/local/tmp/naoty-k0/envrun.sh; ./naoty-k0-client spawn ./naoty-kernel -f "{conn}"'
```

Precompile bytecode on the host before pushing (it cuts the first `import` time by a factor of
2 to 5): `python -m compileall -q --invalidation-mode unchecked-hash <tree>` with any 3.13.
Stock ipykernel on Android: `pip install --target ipk --no-deps` the pure-Python stack
(ipykernel, **ipython<9** because matplotlib 3.8 imports `backend2gui`, jupyter_client,
tornado without its `.so`, nest-asyncio2…) plus the **pyzmq android wheel**, run
`python -m ipykernel_launcher -f {conn}` with `PYTHONPATH=ipk:site` and no `MPLBACKEND`.

## 4. Sizes

`./measure-sizes.sh x86_64|arm64` prints raw and `zstd -19` sizes per component (bytecode
included, tests/docs removed, `.so` stripped).

Cleanup when done: `rm -rf target build work .venv` (all git-ignored).
