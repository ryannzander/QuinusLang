from setuptools import setup

setup(
    name="qpp-lexer",
    version="0.1.0",
    py_modules=["qpp_lexer"],
    entry_points={
        "pygments.lexers": [
            "qpp = qpp_lexer:QppLexer",
        ],
    },
)
