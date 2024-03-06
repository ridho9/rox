module.exports = grammar({
  name: "rox",
  word: ($) => $.ident,
  rules: {
    program: ($) => repeat($.stmt),

    stmt: ($) => choice($.print_stmt, $.let_stmt),

    let_stmt: ($) => seq("let", $.ident, optional(seq("=", $.expr)), ";"),
    print_stmt: ($) => seq("print", $.expr, ";"),

    expr: ($) => choice($._primary, $.unary_expr, $.binary_expr),
    unary_expr: ($) => prec(10, choice(seq("!", $.expr), seq("-", $.expr))),
    binary_expr: ($) =>
      choice(
        prec.left(9, seq($.expr, choice("*", "/"), $.expr)),
        prec.left(8, seq($.expr, choice("+", "-"), $.expr)),
        prec.left(7, seq($.expr, choice("<=", "<", ">=", ">"), $.expr)),
        prec.left(6, seq($.expr, choice("==", "!="), $.expr))
      ),

    _primary: ($) =>
      choice($.bool, $.nil, $.number, $.ident, $.string, $._group),
    bool: ($) => choice("true", "false"),
    nil: ($) => "nil",
    number: ($) => /\d+(\.\d+)?/,
    ident: ($) => /[_a-zA-Z]\w*/,
    string: ($) => /"([^"\\]|\\.)*"/,
    _group: ($) => seq("(", $.expr, ")"),
  },
});
