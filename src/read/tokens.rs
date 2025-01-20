// Available reserved
const BREAK: &str = "break";
const CASE: &str = "case";
const CATCH: &str = "catch";
const CLASS: &str = "class";
const CONST: &str = "const";
const CONTINUE: &str = "continue";
const DEBUGGER: &str = "debugger";
const DEFAULT: &str = "default";
const DELETE: &str = "delete";
const DO: &str = "do";
const ELSE: &str = "else";
const EXPORT: &str = "export";
const EXTENDS: &str = "extends";
const FALSE: &str = "false";
const FINALLY: &str = "finally";
const FOR: &str = "for";
const FUNCTION: &str = "function";
const IF: &str = "if";
const IMPORT: &str = "import";
const IN: &str = "in";
const INSTANCEOF: &str = "instanceof";
const NEW: &str = "new";
const NULL: &str = "null";
const RETURN: &str = "return";
const SUPER: &str = "super";
const SWITCH: &str = "switch";
const THIS: &str = "this";
const THROW: &str = "throw";
const TRUE: &str = "true";
const TRY: &str = "try";
const TYPEOF: &str = "typeof";
const VAR: &str = "var";
const VOID: &str = "void";
const WHILE: &str = "whie";
const WITH: &str = "with";

// Only strict mode
const STRICT_LET: &str = "let";
const STRICT_STATIC: &str = "static";
const STRICT_YIELD: &str = "yield";
const STRICT_IMPLEMENTS: &str = "implements";
const STRICT_INTERFACE: &str = "interface";
const STRICT_PACKAGE: &str = "package";
const STRICT_PRIVATE: &str = "private";
const STRICT_PROTECTED: &str = "protected";
const STRICT_PUBLIC: &str = "public";

// Only in modules or async function bodies
const MODULE_ASYNC: &str = "await";

// Future reseved
const FUTURE_ENUM: &str = "enum";

// Future reserved by ECMAScript 1 to 3
const FUTURE_OLD_ABSTRACT: &str = "abstract";
const FUTURE_OLD_BOOLEAN: &str = "boolean";
const FUTURE_OLD_BYTE: &str = "byte";
const FUTURE_OLD_CHAR: &str = "char";
const FUTURE_OLD_DOUBLE: &str = "double";
const FUTURE_OLD_FINAL: &str = "final";
const FUTURE_OLD_FLOAT: &str = "float";
const FUTURE_OLD_GOTO: &str = "goto";
const FUTURE_OLD_INT: &str = "int";
const FUTURE_OLD_LONG: &str = "long";
const FUTURE_OLD_NATIVE: &str = "native";
const FUTURE_OLD_SHORT: &str = "short";
const FUTURE_OLD_SYNCHRONIZED: &str = "synchronized";
const FUTURE_OLD_THROWS: &str = "throws";
const FUTURE_OLD_TRANSIENT: &str = "transient";
const FUTURE_OLD_VOLATILE: &str = "volatile";

// Identifiers with special meanings
const SPECIAL_STRICT_ARGUMENTS: &str = "arguments"; // only works in strict
const SPECIAL_AS: &str = "as";
const SPECIAL_STRICT_EVAL: &str = "eval"; // only works in strict
const SPECIAL_FROM: &str = "from";
const SPECIAL_GET: &str = "get";
const SPECIAL_OF: &str = "of";
const SPECIAL_SET: &str = "set";

// String literals (UTF-8 Hex)
const LITERAL_BACKSLASH: u8 = 0x5C; // \
const LITERAL_CARRIAGE_RETURN: u8 = 0x0D; // CR
const LITERAL_LINE_FEED: u8 = 0x0A; // LF
const LITERAL_SINGLE_QUOTE: u8 = 0x27; // '
const LITERAL_DOUBLE_QUOTE: u8 = 0x22; // "
const LITERAL_BACK_TICK: u8 = 0x60; // `
const LITERAL_PLUS: u8 = 0x2B; // +
const LITERAL_MINUS: u8 = 0x2D; // -
const LITERAL_EQUAL: u8 = 0x3D; // =
const LITERAL_ASTERISK: u8 = 0x2A; // *
const LITERAL_SLASH: u8 = 0x2F; // /
const LITERAL_GREATER_THAN: u8 = 0x3E; // >
const LITERAL_LESS_THAN: u8 = 0x3C; // <
const LITERAL_AMPERSAND: u8 = 0x26; // &
const LITERAL_SPACE: u8 = 0x20; //
const LITERAL_TAB: u8 = 0x09; //
const LITERAL_COMMA: u8 = 0x2C; // ,
const LITERAL_PERIOD: u8 = 0x2E; // .
const LITERAL_SEMICOLON: u8 = 0x3B; // ;
const LITERAL_COLON: u8 = 0x3A; // :
const LITERAL_EXCLAMATION_MARK: u8 = 0x21; // !
const LITERAL_QUESTION_MARK: u8 = 0x3F; // ?
const LITERAL_PIPE: u8 = 0x7C; // |
const LITERAL_CARET: u8 = 0x5E; // ^
const LITERAL_OPEN_BRACKET_LEFT: u8 = 0x28; // (
const LITERAL_OPEN_BRACKET_RIGHT: u8 = 0x29; // )
const LITERAL_SQUARE_BRACKET_LEFT: u8 = 0x5B; // [
const LITERAL_SQUARE_BRACKET_RIGHT: u8 = 0x5D; // ]
const LITERAL_CURLY_BRACKET_LEFT: u8 = 0x7B; // {
const LITERAL_CURLY_BRACKET_RIGHT: u8 = 0x7D; // }
const LITERAL_TILDE: u8 = 0x7E; // ~
const LITERAL_DOLLAR: u8 = 0x24; // $
const LITERAL_AT: u8 = 0x40; // @
const LITERAL_UNDERSCORE: u8 = 0x5F; // _
const LITERAL_PERCENT: u8 = 0x25; // %
