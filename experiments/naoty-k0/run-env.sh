# source me: environment to run the embedding kernel on Linux against the uv Python 3.13
B=$(dirname $(dirname $(readlink -f "$(dirname "${BASH_SOURCE[0]}")/.venv/bin/python")))
export LD_LIBRARY_PATH=$B/lib PYTHONHOME=$B NAOTY_PYPATH=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/.venv/lib/python3.13/site-packages
