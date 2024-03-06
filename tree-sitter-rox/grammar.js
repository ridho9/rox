module.exports = grammar({
  name: "rox",

  rules: {
    // TODO: add the actual grammar rules
    program: ($) => repeat($._primary),
    _primary: ($) => choice($.bool, $.nil, $.number, $.ident),
    bool: ($) => choice("true", "false"),
    nil: ($) => "nil",
    number: ($) => /\d+(.\d+)?/,
    ident: ($) => /[_a-zA-Z]\w*/,
  },
});
