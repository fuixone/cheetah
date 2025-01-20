use crate::error::errors::Errors;

struct PossibleToken {

}

pub struct Token {
    
}

impl Token {
    pub fn tokenize(buffer: Vec<u8>) -> Result<Vec<Token>, Errors> {

        buffer.iter().for_each(|character| {

        });

        Ok(Vec::new())
    }

    // fn token_match(character: char) -> PossibleToken {
        
    // }
}

enum ValueType {
    Str(&'static str),
    U8(u8)
}

trait Value {
    fn get_value(token: &Token) -> ValueType;
}

// impl<T> Value for T {
//     fn get_value(token: &Token) -> ValueType {
        
//     }
// }

#[allow(non_camel_case_types)]
enum Identifier {
    BREAK,
    CASE,
    CATCH,
    CLASS,
    CONST,
    CONTINUE,
    DEBUGGER,
    DEFAULT,
    DELETE,
    DO,
    ELSE,
    EXPORT,
    EXTENDS,
    FALSE,
    FINALLY,
    FOR,
    FUNCTION,
    IF,
    IMPORT,
    IN,
    INSTANCEOF,
    NEW,
    NULL,
    RETURN,
    SUPER,
    SWITCH,
    THIS,
    THROW,
    TRUE,
    TRY,
    TYPEOF,
    VAR,
    VOID,
    WHILE,
    WITH,
    STRICT_LET,
    STRICT_STATIC,
    STRICT_YIELD,
    STRICT_IMPLEMENTS,
    STRICT_INTERFACE,
    STRICT_PACKAGE,
    STRICT_PRIVATE,
    STRICT_PROTECTED,
    STRICT_PUBLIC,
    MODULE_ASYNC,
    FUTURE_ENUM,
    FUTURE_OLD_ABSTRACT,
    FUTURE_OLD_BOOLEAN,
    FUTURE_OLD_BYTE,
    FUTURE_OLD_CHAR,
    FUTURE_OLD_DOUBLE,
    FUTURE_OLD_FINAL,
    FUTURE_OLD_FLOAT,
    FUTURE_OLD_GOTO,
    FUTURE_OLD_INT,
    FUTURE_OLD_LONG,
    FUTURE_OLD_NATIVE,
    FUTURE_OLD_SHORT,
    FUTURE_OLD_SYNCHRONIZED,
    FUTURE_OLD_THROWS,
    FUTURE_OLD_TRANSIENT,
    FUTURE_OLD_VOLATILE,
    SPECIAL_STRICT_ARGUMENTS,
    SPECIAL_AS,
    SPECIAL_STRICT_EVAL,
    SPECIAL_FROM,
    SPECIAL_GET,
    SPECIAL_OF,
    SPECIAL_SET
}

#[allow(non_camel_case_types)]
enum Operator {
    PLUS,
    MINUS,
    EQUAL,
    ASTERISK,
    SLASH,
    GREATER_THAN,
    LESS_THAN,
    AMPERSAND,
    PIPE,
    CARET,
    EXCLAMATION_MARK,
    QUESTION_MARK,
    PERCENT
}

#[allow(non_camel_case_types)]
enum Literal {
    BACKSLASH,
    CARRIAGE_RETURN,
    LINE_FEED,
    SINGLE_QUOTE,
    DOUBLE_QUOTE,
    BACK_TICK,
    PLUS,
    MINUS,
    EQUAL,
    ASTERISK,
    SLASH,
    GREATER_THAN,
    LESS_THAN,
    AMPERSAND,
    SPACE,
    TAB,
    COMMA,
    PERIOD,
    SEMICOLON,
    COLON,
    EXCLAMATION_MARK,
    QUESTION_MARK,
    PIPE,
    CARET,
    OPEN_BRACKET_LEFT,
    OPEN_BRACKET_RIGHT,
    SQUARE_BRACKET_LEFT,
    SQUARE_BRACKET_RIGHT,
    CURLY_BRACKET_LEFT,
    CURLY_BRACKET_RIGHT,
    TILDE,
    DOLLAR,
    AT,
    UNDERSCORE,
    PERCENT
}

#[allow(non_camel_case_types)]
enum Punctuation {
    COMMA,
    PERIOD,
    SEMICOLON,
    COLON,
    OPEN_BRACKET_LEFT,
    OPEN_BRACKET_RIGHT,
    SQUARE_BRACKET_LEFT,
    SQUARE_BRACKET_RIGHT,
    CURLY_BRACKET_LEFT,
    CURLY_BRACKET_RIGHT,
    UNDERSCORE,
    DOLLAR,
    AT,
    BACK_TICK
}
