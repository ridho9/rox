#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 31
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 35
#define ALIAS_COUNT 0
#define TOKEN_COUNT 24
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 5
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  sym_ident = 1,
  anon_sym_let = 2,
  anon_sym_EQ = 3,
  anon_sym_SEMI = 4,
  anon_sym_print = 5,
  anon_sym_BANG = 6,
  anon_sym_DASH = 7,
  anon_sym_STAR = 8,
  anon_sym_SLASH = 9,
  anon_sym_PLUS = 10,
  anon_sym_LT_EQ = 11,
  anon_sym_LT = 12,
  anon_sym_GT_EQ = 13,
  anon_sym_GT = 14,
  anon_sym_EQ_EQ = 15,
  anon_sym_BANG_EQ = 16,
  anon_sym_true = 17,
  anon_sym_false = 18,
  sym_nil = 19,
  sym_number = 20,
  sym_string = 21,
  anon_sym_LPAREN = 22,
  anon_sym_RPAREN = 23,
  sym_program = 24,
  sym_stmt = 25,
  sym_let_stmt = 26,
  sym_print_stmt = 27,
  sym_expr = 28,
  sym_unary_expr = 29,
  sym_binary_expr = 30,
  sym_primary = 31,
  sym_bool = 32,
  sym__group = 33,
  aux_sym_program_repeat1 = 34,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_ident] = "ident",
  [anon_sym_let] = "let",
  [anon_sym_EQ] = "=",
  [anon_sym_SEMI] = ";",
  [anon_sym_print] = "print",
  [anon_sym_BANG] = "!",
  [anon_sym_DASH] = "-",
  [anon_sym_STAR] = "*",
  [anon_sym_SLASH] = "/",
  [anon_sym_PLUS] = "+",
  [anon_sym_LT_EQ] = "<=",
  [anon_sym_LT] = "<",
  [anon_sym_GT_EQ] = ">=",
  [anon_sym_GT] = ">",
  [anon_sym_EQ_EQ] = "==",
  [anon_sym_BANG_EQ] = "!=",
  [anon_sym_true] = "true",
  [anon_sym_false] = "false",
  [sym_nil] = "nil",
  [sym_number] = "number",
  [sym_string] = "string",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [sym_program] = "program",
  [sym_stmt] = "stmt",
  [sym_let_stmt] = "let_stmt",
  [sym_print_stmt] = "print_stmt",
  [sym_expr] = "expr",
  [sym_unary_expr] = "unary_expr",
  [sym_binary_expr] = "binary_expr",
  [sym_primary] = "primary",
  [sym_bool] = "bool",
  [sym__group] = "_group",
  [aux_sym_program_repeat1] = "program_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_ident] = sym_ident,
  [anon_sym_let] = anon_sym_let,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_print] = anon_sym_print,
  [anon_sym_BANG] = anon_sym_BANG,
  [anon_sym_DASH] = anon_sym_DASH,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_LT_EQ] = anon_sym_LT_EQ,
  [anon_sym_LT] = anon_sym_LT,
  [anon_sym_GT_EQ] = anon_sym_GT_EQ,
  [anon_sym_GT] = anon_sym_GT,
  [anon_sym_EQ_EQ] = anon_sym_EQ_EQ,
  [anon_sym_BANG_EQ] = anon_sym_BANG_EQ,
  [anon_sym_true] = anon_sym_true,
  [anon_sym_false] = anon_sym_false,
  [sym_nil] = sym_nil,
  [sym_number] = sym_number,
  [sym_string] = sym_string,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [sym_program] = sym_program,
  [sym_stmt] = sym_stmt,
  [sym_let_stmt] = sym_let_stmt,
  [sym_print_stmt] = sym_print_stmt,
  [sym_expr] = sym_expr,
  [sym_unary_expr] = sym_unary_expr,
  [sym_binary_expr] = sym_binary_expr,
  [sym_primary] = sym_primary,
  [sym_bool] = sym_bool,
  [sym__group] = sym__group,
  [aux_sym_program_repeat1] = aux_sym_program_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_ident] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_let] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_print] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BANG] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BANG_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_true] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_false] = {
    .visible = true,
    .named = false,
  },
  [sym_nil] = {
    .visible = true,
    .named = true,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [sym_program] = {
    .visible = true,
    .named = true,
  },
  [sym_stmt] = {
    .visible = true,
    .named = true,
  },
  [sym_let_stmt] = {
    .visible = true,
    .named = true,
  },
  [sym_print_stmt] = {
    .visible = true,
    .named = true,
  },
  [sym_expr] = {
    .visible = true,
    .named = true,
  },
  [sym_unary_expr] = {
    .visible = true,
    .named = true,
  },
  [sym_binary_expr] = {
    .visible = true,
    .named = true,
  },
  [sym_primary] = {
    .visible = true,
    .named = true,
  },
  [sym_bool] = {
    .visible = true,
    .named = true,
  },
  [sym__group] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_program_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(8);
      if (lookahead == '!') ADVANCE(13);
      if (lookahead == '"') ADVANCE(3);
      if (lookahead == '(') ADVANCE(28);
      if (lookahead == ')') ADVANCE(29);
      if (lookahead == '*') ADVANCE(15);
      if (lookahead == '+') ADVANCE(17);
      if (lookahead == '-') ADVANCE(14);
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == ';') ADVANCE(11);
      if (lookahead == '<') ADVANCE(19);
      if (lookahead == '=') ADVANCE(10);
      if (lookahead == '>') ADVANCE(21);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(24);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(26);
      END_STATE();
    case 1:
      if (lookahead == '!') ADVANCE(12);
      if (lookahead == '"') ADVANCE(3);
      if (lookahead == '(') ADVANCE(28);
      if (lookahead == '-') ADVANCE(14);
      if (lookahead == ';') ADVANCE(11);
      if (lookahead == '=') ADVANCE(9);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(1)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(24);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(26);
      END_STATE();
    case 2:
      if (lookahead == '!') ADVANCE(4);
      if (lookahead == ')') ADVANCE(29);
      if (lookahead == '*') ADVANCE(15);
      if (lookahead == '+') ADVANCE(17);
      if (lookahead == '-') ADVANCE(14);
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == ';') ADVANCE(11);
      if (lookahead == '<') ADVANCE(19);
      if (lookahead == '=') ADVANCE(5);
      if (lookahead == '>') ADVANCE(21);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(2)
      END_STATE();
    case 3:
      if (lookahead == '"') ADVANCE(27);
      if (lookahead == '\\') ADVANCE(7);
      if (lookahead != 0) ADVANCE(3);
      END_STATE();
    case 4:
      if (lookahead == '=') ADVANCE(23);
      END_STATE();
    case 5:
      if (lookahead == '=') ADVANCE(22);
      END_STATE();
    case 6:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(25);
      END_STATE();
    case 7:
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(3);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_EQ);
      if (lookahead == '=') ADVANCE(22);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_BANG);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_BANG);
      if (lookahead == '=') ADVANCE(23);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_SLASH);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_LT_EQ);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '=') ADVANCE(18);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_GT_EQ);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead == '=') ADVANCE(20);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_EQ_EQ);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_BANG_EQ);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(sym_number);
      if (lookahead == '.') ADVANCE(6);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(24);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(25);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(sym_ident);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(26);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(sym_string);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    default:
      return false;
  }
}

static bool ts_lex_keywords(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (lookahead == 'f') ADVANCE(1);
      if (lookahead == 'l') ADVANCE(2);
      if (lookahead == 'n') ADVANCE(3);
      if (lookahead == 'p') ADVANCE(4);
      if (lookahead == 't') ADVANCE(5);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0)
      END_STATE();
    case 1:
      if (lookahead == 'a') ADVANCE(6);
      END_STATE();
    case 2:
      if (lookahead == 'e') ADVANCE(7);
      END_STATE();
    case 3:
      if (lookahead == 'i') ADVANCE(8);
      END_STATE();
    case 4:
      if (lookahead == 'r') ADVANCE(9);
      END_STATE();
    case 5:
      if (lookahead == 'r') ADVANCE(10);
      END_STATE();
    case 6:
      if (lookahead == 'l') ADVANCE(11);
      END_STATE();
    case 7:
      if (lookahead == 't') ADVANCE(12);
      END_STATE();
    case 8:
      if (lookahead == 'l') ADVANCE(13);
      END_STATE();
    case 9:
      if (lookahead == 'i') ADVANCE(14);
      END_STATE();
    case 10:
      if (lookahead == 'u') ADVANCE(15);
      END_STATE();
    case 11:
      if (lookahead == 's') ADVANCE(16);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_let);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(sym_nil);
      END_STATE();
    case 14:
      if (lookahead == 'n') ADVANCE(17);
      END_STATE();
    case 15:
      if (lookahead == 'e') ADVANCE(18);
      END_STATE();
    case 16:
      if (lookahead == 'e') ADVANCE(19);
      END_STATE();
    case 17:
      if (lookahead == 't') ADVANCE(20);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_print);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 1},
  [3] = {.lex_state = 1},
  [4] = {.lex_state = 1},
  [5] = {.lex_state = 1},
  [6] = {.lex_state = 1},
  [7] = {.lex_state = 1},
  [8] = {.lex_state = 1},
  [9] = {.lex_state = 1},
  [10] = {.lex_state = 2},
  [11] = {.lex_state = 2},
  [12] = {.lex_state = 2},
  [13] = {.lex_state = 2},
  [14] = {.lex_state = 2},
  [15] = {.lex_state = 2},
  [16] = {.lex_state = 2},
  [17] = {.lex_state = 2},
  [18] = {.lex_state = 2},
  [19] = {.lex_state = 2},
  [20] = {.lex_state = 2},
  [21] = {.lex_state = 2},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 0},
  [27] = {.lex_state = 0},
  [28] = {.lex_state = 1},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_ident] = ACTIONS(1),
    [anon_sym_let] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_print] = ACTIONS(1),
    [anon_sym_BANG] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_LT_EQ] = ACTIONS(1),
    [anon_sym_LT] = ACTIONS(1),
    [anon_sym_GT_EQ] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [anon_sym_EQ_EQ] = ACTIONS(1),
    [anon_sym_BANG_EQ] = ACTIONS(1),
    [anon_sym_true] = ACTIONS(1),
    [anon_sym_false] = ACTIONS(1),
    [sym_nil] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
    [sym_string] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
  },
  [1] = {
    [sym_program] = STATE(30),
    [sym_stmt] = STATE(23),
    [sym_let_stmt] = STATE(26),
    [sym_print_stmt] = STATE(26),
    [aux_sym_program_repeat1] = STATE(23),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_let] = ACTIONS(5),
    [anon_sym_print] = ACTIONS(7),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 8,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    STATE(20), 1,
      sym_expr,
    ACTIONS(9), 2,
      sym_nil,
      sym_ident,
    ACTIONS(11), 2,
      anon_sym_BANG,
      anon_sym_DASH,
    ACTIONS(13), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(15), 2,
      sym_number,
      sym_string,
    STATE(11), 2,
      sym_bool,
      sym__group,
    STATE(12), 3,
      sym_unary_expr,
      sym_binary_expr,
      sym_primary,
  [32] = 8,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    STATE(13), 1,
      sym_expr,
    ACTIONS(9), 2,
      sym_nil,
      sym_ident,
    ACTIONS(11), 2,
      anon_sym_BANG,
      anon_sym_DASH,
    ACTIONS(13), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(15), 2,
      sym_number,
      sym_string,
    STATE(11), 2,
      sym_bool,
      sym__group,
    STATE(12), 3,
      sym_unary_expr,
      sym_binary_expr,
      sym_primary,
  [64] = 8,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    STATE(19), 1,
      sym_expr,
    ACTIONS(9), 2,
      sym_nil,
      sym_ident,
    ACTIONS(11), 2,
      anon_sym_BANG,
      anon_sym_DASH,
    ACTIONS(13), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(15), 2,
      sym_number,
      sym_string,
    STATE(11), 2,
      sym_bool,
      sym__group,
    STATE(12), 3,
      sym_unary_expr,
      sym_binary_expr,
      sym_primary,
  [96] = 8,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    STATE(18), 1,
      sym_expr,
    ACTIONS(9), 2,
      sym_nil,
      sym_ident,
    ACTIONS(11), 2,
      anon_sym_BANG,
      anon_sym_DASH,
    ACTIONS(13), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(15), 2,
      sym_number,
      sym_string,
    STATE(11), 2,
      sym_bool,
      sym__group,
    STATE(12), 3,
      sym_unary_expr,
      sym_binary_expr,
      sym_primary,
  [128] = 8,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    STATE(10), 1,
      sym_expr,
    ACTIONS(9), 2,
      sym_nil,
      sym_ident,
    ACTIONS(11), 2,
      anon_sym_BANG,
      anon_sym_DASH,
    ACTIONS(13), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(15), 2,
      sym_number,
      sym_string,
    STATE(11), 2,
      sym_bool,
      sym__group,
    STATE(12), 3,
      sym_unary_expr,
      sym_binary_expr,
      sym_primary,
  [160] = 8,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    STATE(17), 1,
      sym_expr,
    ACTIONS(9), 2,
      sym_nil,
      sym_ident,
    ACTIONS(11), 2,
      anon_sym_BANG,
      anon_sym_DASH,
    ACTIONS(13), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(15), 2,
      sym_number,
      sym_string,
    STATE(11), 2,
      sym_bool,
      sym__group,
    STATE(12), 3,
      sym_unary_expr,
      sym_binary_expr,
      sym_primary,
  [192] = 8,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    STATE(21), 1,
      sym_expr,
    ACTIONS(9), 2,
      sym_nil,
      sym_ident,
    ACTIONS(11), 2,
      anon_sym_BANG,
      anon_sym_DASH,
    ACTIONS(13), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(15), 2,
      sym_number,
      sym_string,
    STATE(11), 2,
      sym_bool,
      sym__group,
    STATE(12), 3,
      sym_unary_expr,
      sym_binary_expr,
      sym_primary,
  [224] = 8,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    STATE(15), 1,
      sym_expr,
    ACTIONS(9), 2,
      sym_nil,
      sym_ident,
    ACTIONS(11), 2,
      anon_sym_BANG,
      anon_sym_DASH,
    ACTIONS(13), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(15), 2,
      sym_number,
      sym_string,
    STATE(11), 2,
      sym_bool,
      sym__group,
    STATE(12), 3,
      sym_unary_expr,
      sym_binary_expr,
      sym_primary,
  [256] = 2,
    ACTIONS(21), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(19), 10,
      anon_sym_SEMI,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_SLASH,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_RPAREN,
  [273] = 2,
    ACTIONS(25), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(23), 10,
      anon_sym_SEMI,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_SLASH,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_RPAREN,
  [290] = 2,
    ACTIONS(29), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(27), 10,
      anon_sym_SEMI,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_SLASH,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_RPAREN,
  [307] = 5,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(33), 2,
      anon_sym_STAR,
      anon_sym_SLASH,
    ACTIONS(35), 2,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
    ACTIONS(37), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(19), 4,
      anon_sym_SEMI,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_RPAREN,
  [330] = 2,
    ACTIONS(41), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(39), 10,
      anon_sym_SEMI,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_SLASH,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_RPAREN,
  [347] = 2,
    ACTIONS(45), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(43), 10,
      anon_sym_SEMI,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_SLASH,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_RPAREN,
  [364] = 2,
    ACTIONS(49), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(47), 10,
      anon_sym_SEMI,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_SLASH,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_RPAREN,
  [381] = 3,
    ACTIONS(21), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(33), 2,
      anon_sym_STAR,
      anon_sym_SLASH,
    ACTIONS(19), 8,
      anon_sym_SEMI,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_RPAREN,
  [400] = 4,
    ACTIONS(21), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(33), 2,
      anon_sym_STAR,
      anon_sym_SLASH,
    ACTIONS(19), 6,
      anon_sym_SEMI,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_RPAREN,
  [421] = 6,
    ACTIONS(51), 1,
      anon_sym_SEMI,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(33), 2,
      anon_sym_STAR,
      anon_sym_SLASH,
    ACTIONS(35), 2,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
    ACTIONS(37), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(53), 2,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
  [445] = 6,
    ACTIONS(55), 1,
      anon_sym_RPAREN,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(33), 2,
      anon_sym_STAR,
      anon_sym_SLASH,
    ACTIONS(35), 2,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
    ACTIONS(37), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(53), 2,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
  [469] = 6,
    ACTIONS(57), 1,
      anon_sym_SEMI,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(33), 2,
      anon_sym_STAR,
      anon_sym_SLASH,
    ACTIONS(35), 2,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
    ACTIONS(37), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(53), 2,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
  [493] = 5,
    ACTIONS(59), 1,
      ts_builtin_sym_end,
    ACTIONS(61), 1,
      anon_sym_let,
    ACTIONS(64), 1,
      anon_sym_print,
    STATE(22), 2,
      sym_stmt,
      aux_sym_program_repeat1,
    STATE(26), 2,
      sym_let_stmt,
      sym_print_stmt,
  [511] = 5,
    ACTIONS(5), 1,
      anon_sym_let,
    ACTIONS(7), 1,
      anon_sym_print,
    ACTIONS(67), 1,
      ts_builtin_sym_end,
    STATE(22), 2,
      sym_stmt,
      aux_sym_program_repeat1,
    STATE(26), 2,
      sym_let_stmt,
      sym_print_stmt,
  [529] = 1,
    ACTIONS(69), 3,
      ts_builtin_sym_end,
      anon_sym_let,
      anon_sym_print,
  [535] = 1,
    ACTIONS(71), 3,
      ts_builtin_sym_end,
      anon_sym_let,
      anon_sym_print,
  [541] = 1,
    ACTIONS(73), 3,
      ts_builtin_sym_end,
      anon_sym_let,
      anon_sym_print,
  [547] = 1,
    ACTIONS(75), 3,
      ts_builtin_sym_end,
      anon_sym_let,
      anon_sym_print,
  [553] = 2,
    ACTIONS(77), 1,
      anon_sym_EQ,
    ACTIONS(79), 1,
      anon_sym_SEMI,
  [560] = 1,
    ACTIONS(81), 1,
      sym_ident,
  [564] = 1,
    ACTIONS(83), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 32,
  [SMALL_STATE(4)] = 64,
  [SMALL_STATE(5)] = 96,
  [SMALL_STATE(6)] = 128,
  [SMALL_STATE(7)] = 160,
  [SMALL_STATE(8)] = 192,
  [SMALL_STATE(9)] = 224,
  [SMALL_STATE(10)] = 256,
  [SMALL_STATE(11)] = 273,
  [SMALL_STATE(12)] = 290,
  [SMALL_STATE(13)] = 307,
  [SMALL_STATE(14)] = 330,
  [SMALL_STATE(15)] = 347,
  [SMALL_STATE(16)] = 364,
  [SMALL_STATE(17)] = 381,
  [SMALL_STATE(18)] = 400,
  [SMALL_STATE(19)] = 421,
  [SMALL_STATE(20)] = 445,
  [SMALL_STATE(21)] = 469,
  [SMALL_STATE(22)] = 493,
  [SMALL_STATE(23)] = 511,
  [SMALL_STATE(24)] = 529,
  [SMALL_STATE(25)] = 535,
  [SMALL_STATE(26)] = 541,
  [SMALL_STATE(27)] = 547,
  [SMALL_STATE(28)] = 553,
  [SMALL_STATE(29)] = 560,
  [SMALL_STATE(30)] = 564,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_program, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(14),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_expr, 3),
  [21] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_expr, 3),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_primary, 1),
  [25] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_primary, 1),
  [27] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expr, 1),
  [29] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expr, 1),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [37] = {.entry = {.count = 1, .reusable = false}}, SHIFT(5),
  [39] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bool, 1),
  [41] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_bool, 1),
  [43] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_expr, 2),
  [45] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_expr, 2),
  [47] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__group, 3),
  [49] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__group, 3),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [59] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2),
  [61] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2), SHIFT_REPEAT(29),
  [64] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2), SHIFT_REPEAT(4),
  [67] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_program, 1),
  [69] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_print_stmt, 3),
  [71] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_let_stmt, 3),
  [73] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_stmt, 1),
  [75] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_let_stmt, 5),
  [77] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [79] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [81] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [83] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_rox(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .keyword_lex_fn = ts_lex_keywords,
    .keyword_capture_token = sym_ident,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
