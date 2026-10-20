import ast, base64, io, json, os, sys, traceback, warnings

os.environ.setdefault("MPLBACKEND", "Agg")
warnings.filterwarnings("ignore", message=".*non-interactive.*")
NS = {"__name__": "__main__"}


def _bundle(v):
    d = {"text/plain": repr(v)}
    for attr, mime in (("_repr_html_", "text/html"), ("_repr_latex_", "text/latex"),
                       ("_repr_svg_", "image/svg+xml")):
        f = getattr(v, attr, None)
        if f:
            try:
                r = f()
            except Exception:
                r = None
            if r:
                d[mime] = r
    return d


def _figures(emit):
    plt = sys.modules.get("matplotlib.pyplot")
    if plt is None:
        return
    for n in plt.get_fignums():
        buf = io.BytesIO()
        plt.figure(n).savefig(buf, format="png", bbox_inches="tight")
        emit("display", json.dumps({"data": {"image/png": base64.b64encode(buf.getvalue()).decode(),
                                              "text/plain": "<Figure>"}, "metadata": {}}))
    plt.close("all")


def run(code, emit):
    try:
        tree = ast.parse(code, "<cell>")
        last = ast.Expression(tree.body.pop().value) if tree.body and isinstance(tree.body[-1], ast.Expr) else None
        exec(compile(tree, "<cell>", "exec"), NS)
        if last is not None:
            v = eval(compile(last, "<cell>", "eval"), NS)
            if v is not None:
                NS["_"] = v
                emit("result", json.dumps({"data": _bundle(v), "metadata": {}}))
        _figures(emit)
        return True
    except BaseException as e:
        tb = e.__traceback__
        if tb is not None and tb.tb_next is not None and not isinstance(e, SyntaxError):
            tb = tb.tb_next  # hide this helper's frame
        lines = "".join(traceback.format_exception(type(e), e, tb)).splitlines()
        emit("error", json.dumps({"ename": type(e).__name__, "evalue": str(e), "traceback": lines}))
        return False
