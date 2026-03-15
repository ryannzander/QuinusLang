"""Pygments lexer for QuinusLang (.q files)."""

from pygments.lexer import RegexLexer, bygroups, include
from pygments.token import (
    Comment,
    Keyword,
    Name,
    Number,
    Operator,
    String,
    Text,
)

__all__ = ["QuinusLangLexer"]


class QuinusLangLexer(RegexLexer):
    """Lexer for QuinusLang source code."""

    name = "QuinusLang"
    aliases = ["quinuslang", "quinus", "q"]
    filenames = ["*.q"]

    tokens = {
        "root": [
            (r"\s+", Text),
            (r"//.*$", Comment.Single),
            (r'"(?:[^"\\]|\\.)*"', String.Double),
            (r"`(?:[^`\\]|\\.)*`", String.Backtick),
            (r"-?\b[0-9][0-9_]*(\.([0-9][0-9_]*))?([eE][+-]?[0-9][0-9_]*)?\b", Number),
            (r"\b(true|false)\b", Keyword.Constant),
            (
                r"\b(craft|send|make|shift|check|otherwise|loopwhile|foreach|stop|skip|in|for)\b",
                Keyword,
            ),
            (
                r"\b(eternal|anchor|form|state|fusion|realm|link|mark|reach|hazard|machine|"
                r"cblock|bring|open|import|class|extends|init|new|this|super|impl|implements|"
                r"try|catch|alias|extern|defer|choose|move|pub|priv)\b",
                Keyword.Declaration,
            ),
            (
                r"\b(int|float|bool|str|void|u8|u16|u32|u64|i8|i16|i32|i64|usize|f32|f64)\b",
                Keyword.Type,
            ),
            (
                r"\b(print|write|writeln|read|len|strlen|panic|assert)\b",
                Name.Builtin,
            ),
            (r"\b([a-zA-Z_][a-zA-Z0-9_]*)\s*(?=\()", Name.Function),
            (r"\b([a-zA-Z_][a-zA-Z0-9_]*)\b", Name),
            (r"\.\.|->|=>|==|!=|<=|>=|&&|\|\||[+\-*/%=<>!&|.,:;\[\]{}()]", Operator),
        ]
    }
