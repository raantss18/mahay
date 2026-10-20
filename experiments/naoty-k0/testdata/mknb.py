import nbformat as nbf
nb = nbf.v4.new_notebook()
nb.metadata["kernelspec"] = {"display_name":"Python 3","language":"python","name":"python3"}
nb.metadata["custom_ext"] = {"anything": [1, 2.5, None, {"x": "é"}]}
nb.cells = [
 nbf.v4.new_markdown_cell("# Titre\nUne formule $e^{i\\pi}+1=0$ et du texte ❤ unicode\n"),
 nbf.v4.new_code_cell("print(1+1)\n1+1"),
 nbf.v4.new_code_cell("import matplotlib.pyplot as plt\nplt.plot([1,2,3],[1,4,9])\nplt.show()"),
 nbf.v4.new_code_cell("1/0"),
 nbf.v4.new_code_cell("import sympy as sp\nx=sp.symbols('x')\nsp.latex(sp.integrate(x**2,x))\nsp.Integral(x**2,x)"),
 nbf.v4.new_code_cell("from IPython.display import HTML\nHTML('<b>gras</b><table><tr><td>1</td></tr></table>')"),
 nbf.v4.new_raw_cell("raw \n cell", metadata={"tags":["a","b"]}),
]
nb.cells[0]["metadata"]["collapsed"] = False
nb.cells[0]["attachments"] = {"im.png": {"image/png": "iVBORw0KGgo="}}
nbf.write(nb, "in.ipynb")
