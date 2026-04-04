/**
 * Prism.js language definition for Q++
 * https://prismjs.com/extending.html
 */
(function () {
  if (typeof Prism === 'undefined') return;

  Prism.languages.qpp = {
    comment: {
      pattern: /\/\/.*/,
      greedy: true,
    },
    string: [
      {
        pattern: /"(?:[^"\\]|\\.)*"/,
        greedy: true,
      },
      {
        pattern: /`(?:[^`\\]|\\.)*`/,
        greedy: true,
        alias: 'template-string',
      },
    ],
    keyword: [
      /\b(?:craft|send|make|shift|check|otherwise|loopwhile|foreach|stop|skip|in|for|with|defer|choose)\b/,
      /\b(?:eternal|anchor|form|state|fusion|realm|link|mark|reach|hazard|machine|bring|open|import|class|extends|init|new|this|super|impl|implements|try|catch|alias|extern|move|pub|priv|cblock)\b/,
    ],
    'type': /\b(?:int|float|bool|str|void|u8|u16|u32|u64|i8|i16|i32|i64|usize|f32|f64)\b/,
    builtin: /\b(?:print|write|writeln|read|len|strlen|panic|assert)\b/,
    boolean: /\b(?:true|false)\b/,
    number: /-?\b\d+(?:\.\d+)?(?:e[+-]?\d+)?\b/i,
    function: /\b[a-zA-Z_][a-zA-Z0-9_]*\s*(?=\()/,
    operator: /\.\.|->|=>|==|!=|<=|>=|&&|\|\||[+\-*/%=<>!&|.,:;\[\]{}()]/,
    punctuation: /[.,:;()[\]{}]/,
  };

})();
