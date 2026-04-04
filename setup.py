from setuptools import setup

setup(
    name="quinuslang-lexer",
    version="0.1.0",
    py_modules=["quinuslang_lexer"],
    entry_points={
        "pygments.lexers": [
            "quinuslang = quinuslang_lexer:QuinusLangLexer",
        ],
    },
)
