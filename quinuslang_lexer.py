"""Custom Pygments lexer for QuinusLang (.q files)."""

from pygments.lexer import RegexLexer, words, bygroups
from pygments.token import (
    Comment, String, Keyword, Name, Number, Operator, Punctuation, Text, Token,
)


class QuinusLangLexer(RegexLexer):
    name = "QuinusLang"
    aliases = ["q", "quinuslang", "quinus"]
    filenames = ["*.q"]

    tokens = {
        "root": [
            # Comments
            (r"//.*$", Comment.Single),

            # Strings
            (r'"(?:[^"\\]|\\.)*"', String.Double),
            (r"`(?:[^`\\]|\\.)*`", String.Backtick),

            # Keywords — control flow
            (words((
                "check", "otherwise", "loopwhile", "foreach", "for", "in",
                "stop", "skip", "choose", "with", "defer",
            ), prefix=r"\b", suffix=r"\b"), Keyword),

            # Keywords — declarations
            (words((
                "craft", "send", "make", "shift", "eternal", "anchor",
                "form", "state", "fusion", "realm", "bring",
                "hazard", "machine", "extern", "alias", "move",
                "pub", "priv", "cblock", "open", "import",
                "class", "extends", "init", "new", "this", "super",
                "impl", "implements", "try", "catch", "link", "mark", "reach",
            ), prefix=r"\b", suffix=r"\b"), Keyword.Declaration),

            # Types
            (words((
                "int", "float", "bool", "str", "void",
                "u8", "u16", "u32", "u64",
                "i8", "i16", "i32", "i64",
                "usize", "f32", "f64",
            ), prefix=r"\b", suffix=r"\b"), Keyword.Type),

            # Builtins
            (words((
                "print", "write", "writeln", "read", "len",
                "strlen", "panic", "assert",
            ), prefix=r"\b", suffix=r"\b"), Name.Builtin),

            # Booleans
            (words(("true", "false"), prefix=r"\b", suffix=r"\b"), Keyword.Constant),

            # Numbers
            (r"0x[0-9a-fA-F_]+", Number.Hex),
            (r"0b[01_]+", Number.Bin),
            (r"0o[0-7_]+", Number.Oct),
            (r"\d[\d_]*\.\d[\d_]*(?:[eE][+-]?\d+)?", Number.Float),
            (r"\d[\d_]*(?:[eE][+-]?\d+)?", Number.Integer),

            # Function calls
            (r"([a-zA-Z_]\w*)(\s*)(\()", bygroups(Name.Function, Text, Punctuation)),

            # Identifiers
            (r"[a-zA-Z_]\w*", Name),

            # Operators
            (r"->|=>|==|!=|<=|>=|&&|\|\||\.\.|\.\.\.", Operator),
            (r"[+\-*/%=<>!&|^~]", Operator),

            # Punctuation
            (r"[{}\[\]();,.:@#]", Punctuation),

            # Whitespace
            (r"\s+", Text),
        ],
    }
