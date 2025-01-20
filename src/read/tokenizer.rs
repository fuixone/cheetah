use std::collections::HashMap;

use serde::de::value;
use serde_json::Value;

use crate::error::errors::Errors;

#[derive(Copy, Clone)]
pub enum Token {
    Identifier(Identifier),
    Operator(Operator),
    Literal(Literal),
    Punctuation(Punctuation),
}

pub trait TokenTrait {
    fn tokenize(buffer: Vec<u8>) -> Result<Vec<Token>, Errors>;
    fn set_possible_tokens_from_char(
        current_chars: &mut Vec<CharType>,
        current_possible_tokens: &mut Vec<Token>,
        ch: char,
    );
    fn get_next_possible_char_in_token_based_on_index(index: u8, token: &Token)
        -> Option<CharType>;

    fn to_string(self) -> String;
}

impl TokenTrait for Token {
    fn tokenize(buffer: Vec<u8>) -> Result<Vec<Token>, Errors> {
        // Convert the buffer to a UTF-8 string slice
        let decoded_str =
            std::str::from_utf8(&buffer).map_err(|_| Errors::TokenError("Invalid UTF-8"))?;

        let mut state = TokenState {
            current_chars: Vec::new(),
            current_possible_tokens: Vec::new(),
        };

        // Iterate over each character in the decoded string
        for ch in decoded_str.chars() {
            // Call the trait method
            Self::set_possible_tokens_from_char(
                &mut state.current_chars,
                &mut state.current_possible_tokens,
                ch,
            );
        }

        // Return the final result (possibly a collection of tokens)
        Ok(state.current_possible_tokens) // Return actual result
    }

    fn set_possible_tokens_from_char(
        current_chars: &mut Vec<CharType>,
        current_possible_tokens: &mut Vec<Token>,
        character: char,
    ) {
        // HashMap to hold token names and the possible next char
        let mut token_and_next_char: HashMap<String, CharType> = HashMap::new();

        // Use retain to keep tokens that still have a valid next character
        current_possible_tokens.retain(|token| {
            match Self::get_next_possible_char_in_token_based_on_index(
                current_chars.len().try_into().unwrap(),
                token,
            ) {
                Some(next_char) => {
                    // Insert the token's string representation and the next possible character
                    token_and_next_char.insert(token.to_string(), next_char);
                    true // Keep this token in the Vec
                }
                None => false, // Remove this token from the Vec
            }
        });
    }

    fn get_next_possible_char_in_token_based_on_index(
        index: u8,
        token: &Token,
    ) -> Option<CharType> {
        match token.get_value() {
            ValueType::Str(string_value) => {
                if string_value.len() == index as usize {
                    // Check if the index is at the end of the string
                    None
                } else {
                    let next_char = string_value.chars().nth(index as usize); // Get the character at the next index
                    let return_char = next_char.map(|character| CharType::Char(character));
                    return_char
                }
            }
            ValueType::U8(unsigned8) => {
                if index > 1 {
                    None
                } else {
                    Some(CharType::U8(unsigned8)) // Return the unsigned 8-bit value wrapped in Some
                }
            }
        }
    }

    fn to_string(self) -> String {
        match self.get_value() {
            ValueType::Str(v) => v.to_string(),
            ValueType::U8(v) => v.to_string(),
        }
    }
}

struct TokenState {
    current_chars: Vec<CharType>, // changing based on iterations - value is the char
    current_possible_tokens: Vec<Token>, // possible tokens with that order of chars
}

trait SharedToken {
    fn get_value(&self) -> ValueType;
}

impl SharedToken for Token {
    fn get_value(&self) -> ValueType {
        match self {
            Token::Identifier(identifier) => identifier.get_value(),
            Token::Operator(operator) => operator.get_value(),
            Token::Literal(literal) => literal.get_value(),
            Token::Punctuation(punctuation) => punctuation.get_value(),
        }
    }
}

enum ValueType<'a> {
    Str(&'a str),
    U8(u8),
}

pub enum CharType {
    Char(char),
    U8(u8),
}

#[derive(PartialEq, Copy, Clone)]
#[allow(non_camel_case_types, unused)]
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
    SPECIAL_SET,
}

#[derive(PartialEq, Copy, Clone)]
#[allow(non_camel_case_types, unused)]
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
    PERCENT,
}

#[derive(PartialEq, Copy, Clone)]
#[allow(non_camel_case_types, unused)]
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
    PERCENT,
}

#[derive(PartialEq, Copy, Clone)]
#[allow(non_camel_case_types, unused)]
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
    BACK_TICK,
}

impl SharedToken for Identifier {
    fn get_value(&self) -> ValueType {
        match self {
            Identifier::BREAK => ValueType::Str("break"),
            Identifier::CASE => ValueType::Str("case"),
            Identifier::CATCH => ValueType::Str("catch"),
            Identifier::CLASS => ValueType::Str("class"),
            Identifier::CONST => ValueType::Str("const"),
            Identifier::CONTINUE => ValueType::Str("continue"),
            Identifier::DEBUGGER => ValueType::Str("debugger"),
            Identifier::DEFAULT => ValueType::Str("default"),
            Identifier::DELETE => ValueType::Str("delete"),
            Identifier::DO => ValueType::Str("do"),
            Identifier::ELSE => ValueType::Str("else"),
            Identifier::EXPORT => ValueType::Str("export"),
            Identifier::EXTENDS => ValueType::Str("extends"),
            Identifier::FALSE => ValueType::Str("false"),
            Identifier::FINALLY => ValueType::Str("finally"),
            Identifier::FOR => ValueType::Str("for"),
            Identifier::FUNCTION => ValueType::Str("function"),
            Identifier::IF => ValueType::Str("if"),
            Identifier::IMPORT => ValueType::Str("import"),
            Identifier::IN => ValueType::Str("in"),
            Identifier::INSTANCEOF => ValueType::Str("instanceof"),
            Identifier::NEW => ValueType::Str("new"),
            Identifier::NULL => ValueType::Str("null"),
            Identifier::RETURN => ValueType::Str("return"),
            Identifier::SUPER => ValueType::Str("super"),
            Identifier::SWITCH => ValueType::Str("switch"),
            Identifier::THIS => ValueType::Str("this"),
            Identifier::THROW => ValueType::Str("throw"),
            Identifier::TRUE => ValueType::Str("true"),
            Identifier::TRY => ValueType::Str("try"),
            Identifier::TYPEOF => ValueType::Str("typeof"),
            Identifier::VAR => ValueType::Str("var"),
            Identifier::VOID => ValueType::Str("void"),
            Identifier::WHILE => ValueType::Str("while"),
            Identifier::WITH => ValueType::Str("with"),
            Identifier::STRICT_LET => ValueType::Str("let"),
            Identifier::STRICT_STATIC => ValueType::Str("static"),
            Identifier::STRICT_YIELD => ValueType::Str("yield"),
            Identifier::STRICT_IMPLEMENTS => ValueType::Str("implements"),
            Identifier::STRICT_INTERFACE => ValueType::Str("interface"),
            Identifier::STRICT_PACKAGE => ValueType::Str("package"),
            Identifier::STRICT_PRIVATE => ValueType::Str("private"),
            Identifier::STRICT_PROTECTED => ValueType::Str("protected"),
            Identifier::STRICT_PUBLIC => ValueType::Str("public"),
            Identifier::MODULE_ASYNC => ValueType::Str("async"),
            Identifier::FUTURE_ENUM => ValueType::Str("enum"),
            Identifier::FUTURE_OLD_ABSTRACT => ValueType::Str("abstract"),
            Identifier::FUTURE_OLD_BOOLEAN => ValueType::Str("boolean"),
            Identifier::FUTURE_OLD_BYTE => ValueType::Str("byte"),
            Identifier::FUTURE_OLD_CHAR => ValueType::Str("char"),
            Identifier::FUTURE_OLD_DOUBLE => ValueType::Str("double"),
            Identifier::FUTURE_OLD_FINAL => ValueType::Str("final"),
            Identifier::FUTURE_OLD_FLOAT => ValueType::Str("float"),
            Identifier::FUTURE_OLD_GOTO => ValueType::Str("goto"),
            Identifier::FUTURE_OLD_INT => ValueType::Str("int"),
            Identifier::FUTURE_OLD_LONG => ValueType::Str("long"),
            Identifier::FUTURE_OLD_NATIVE => ValueType::Str("native"),
            Identifier::FUTURE_OLD_SHORT => ValueType::Str("short"),
            Identifier::FUTURE_OLD_SYNCHRONIZED => ValueType::Str("synchronized"),
            Identifier::FUTURE_OLD_THROWS => ValueType::Str("throws"),
            Identifier::FUTURE_OLD_TRANSIENT => ValueType::Str("transient"),
            Identifier::FUTURE_OLD_VOLATILE => ValueType::Str("volatile"),
            Identifier::SPECIAL_STRICT_ARGUMENTS => ValueType::Str("arguments"),
            Identifier::SPECIAL_AS => ValueType::Str("as"),
            Identifier::SPECIAL_STRICT_EVAL => ValueType::Str("eval"),
            Identifier::SPECIAL_FROM => ValueType::Str("from"),
            Identifier::SPECIAL_GET => ValueType::Str("get"),
            Identifier::SPECIAL_OF => ValueType::Str("of"),
            Identifier::SPECIAL_SET => ValueType::Str("set"),
        }
    }
}

impl SharedToken for Operator {
    fn get_value(&self) -> ValueType {
        match self {
            Operator::PLUS => ValueType::U8(0x2B),
            Operator::MINUS => ValueType::U8(0x2D),
            Operator::EQUAL => ValueType::U8(0x3D),
            Operator::ASTERISK => ValueType::U8(0x2A),
            Operator::SLASH => ValueType::U8(0x2F),
            Operator::GREATER_THAN => ValueType::U8(0x3E),
            Operator::LESS_THAN => ValueType::U8(0x3C),
            Operator::AMPERSAND => ValueType::U8(0x26),
            Operator::PIPE => ValueType::U8(0x7C),
            Operator::CARET => ValueType::U8(0x5E),
            Operator::EXCLAMATION_MARK => ValueType::U8(0x21),
            Operator::QUESTION_MARK => ValueType::U8(0x3F),
            Operator::PERCENT => ValueType::U8(0x25),
        }
    }
}

impl SharedToken for Literal {
    fn get_value(&self) -> ValueType {
        match self {
            Literal::BACKSLASH => ValueType::U8(0x5C),
            Literal::CARRIAGE_RETURN => ValueType::U8(0x0D),
            Literal::LINE_FEED => ValueType::U8(0x0A),
            Literal::SINGLE_QUOTE => ValueType::U8(0x27),
            Literal::DOUBLE_QUOTE => ValueType::U8(0x22),
            Literal::BACK_TICK => ValueType::U8(0x60),
            Literal::PLUS => ValueType::U8(0x2B),
            Literal::MINUS => ValueType::U8(0x2D),
            Literal::EQUAL => ValueType::U8(0x3D),
            Literal::ASTERISK => ValueType::U8(0x2A),
            Literal::SLASH => ValueType::U8(0x2F),
            Literal::GREATER_THAN => ValueType::U8(0x3E),
            Literal::LESS_THAN => ValueType::U8(0x3C),
            Literal::AMPERSAND => ValueType::U8(0x26),
            Literal::SPACE => ValueType::U8(0x20),
            Literal::TAB => ValueType::U8(0x09),
            Literal::COMMA => ValueType::U8(0x2C),
            Literal::PERIOD => ValueType::U8(0x2E),
            Literal::SEMICOLON => ValueType::U8(0x3B),
            Literal::COLON => ValueType::U8(0x3A),
            Literal::EXCLAMATION_MARK => ValueType::U8(0x21),
            Literal::QUESTION_MARK => ValueType::U8(0x3F),
            Literal::PIPE => ValueType::U8(0x7C),
            Literal::CARET => ValueType::U8(0x5E),
            Literal::OPEN_BRACKET_LEFT => ValueType::U8(0x28),
            Literal::OPEN_BRACKET_RIGHT => ValueType::U8(0x29),
            Literal::SQUARE_BRACKET_LEFT => ValueType::U8(0x5B),
            Literal::SQUARE_BRACKET_RIGHT => ValueType::U8(0x5D),
            Literal::CURLY_BRACKET_LEFT => ValueType::U8(0x7B),
            Literal::CURLY_BRACKET_RIGHT => ValueType::U8(0x7D),
            Literal::TILDE => ValueType::U8(0x7E),
            Literal::DOLLAR => ValueType::U8(0x24),
            Literal::AT => ValueType::U8(0x40),
            Literal::UNDERSCORE => ValueType::U8(0x5F),
            Literal::PERCENT => ValueType::U8(0x25),
        }
    }
}

impl SharedToken for Punctuation {
    fn get_value(&self) -> ValueType {
        match self {
            Punctuation::COMMA => ValueType::U8(0x2C),
            Punctuation::PERIOD => ValueType::U8(0x2E),
            Punctuation::SEMICOLON => ValueType::U8(0x3B),
            Punctuation::COLON => ValueType::U8(0x3A),
            Punctuation::OPEN_BRACKET_LEFT => ValueType::U8(0x28),
            Punctuation::OPEN_BRACKET_RIGHT => ValueType::U8(0x29),
            Punctuation::SQUARE_BRACKET_LEFT => ValueType::U8(0x5B),
            Punctuation::SQUARE_BRACKET_RIGHT => ValueType::U8(0x5D),
            Punctuation::CURLY_BRACKET_LEFT => ValueType::U8(0x7B),
            Punctuation::CURLY_BRACKET_RIGHT => ValueType::U8(0x7D),
            Punctuation::UNDERSCORE => ValueType::U8(0x5F),
            Punctuation::DOLLAR => ValueType::U8(0x24),
            Punctuation::AT => ValueType::U8(0x40),
            Punctuation::BACK_TICK => ValueType::U8(0x60),
        }
    }
}
