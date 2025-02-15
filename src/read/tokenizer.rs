use std::collections::HashMap;

use serde::de::value;
use serde_json::Value;

use crate::error::errors::Errors;

#[derive(Copy, Clone, Debug)]
pub(crate) enum Token {
    Identifier(Identifier),
    Literal(Literal),
}

pub(crate) trait TokenTrait {
    fn tokenize(buffer: &[u8]) -> Result<Vec<Token>, Errors>;
    fn set_possible_tokens_from_u8(
        current_bytes: &mut Vec<u8>,
        current_possible_tokens: &mut Vec<Token>,
        byte: u8,
    );
    fn get_next_possible_byte_in_token_based_on_index(index: u8, token: &Token) -> Option<u8>;

    fn to_string(self) -> String;
}

impl TokenTrait for Token {
    fn tokenize(buffer: &[u8]) -> Result<Vec<Token>, Errors> {
        // Convert the buffer to a UTF-8 string slice
        let mut state = TokenState {
            current_bytes: Vec::new(),
            current_possible_tokens: Vec::new(),
        };

        // Iterate over each character in the decoded string
        for byte in buffer {
            // Call the trait method
            Self::set_possible_tokens_from_u8(
                &mut state.current_bytes,
                &mut state.current_possible_tokens,
                byte,
            );
        }

        // Return the final result (possibly a collection of tokens)
        Ok(state.current_possible_tokens) // Return actual result
    }

    fn set_possible_tokens_from_u8(
        current_bytes: &mut Vec<u8>,
        current_possible_tokens: &mut Vec<Token>,
        byte: u8,
    ) {
        if current_possible_tokens.len() == 0 {}

        // Use retain to keep tokens that still have a valid next character
        current_possible_tokens.retain(|token| {
            match Self::get_next_possible_byte_in_token_based_on_index(
                current_bytes.len().try_into().unwrap(),
                token,
            ) {
                Some(next_byte) => {
                    // If next_char and character are the same then we can keep it in our vec, otherwise remove
                    next_byte == byte
                }
                None => false, // Remove this token from the Vec
            }
        });
    }

    /**
     *
     * @args index: u8 - next index to check in token
     * @args token: Option<&Token> - the token to get for
     *
     *
     * @returns Option<u8>
     * @returns u8 if it has another u8
     * @returns None if it doesn't have any more char's
     */
    fn get_next_possible_byte_in_token_based_on_index(index: u8, token: &Token) -> Option<u8> {
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
                if index > 0 {
                    None
                } else {
                    Some(unsigned8) // Return the unsigned 8-bit value wrapped in Some
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
    current_bytes: Vec<u8>, // changing based on iterations - value is the char
    current_possible_tokens: Vec<Token>, // possible tokens with that order of chars
}

pub(crate) trait SharedToken {
    fn get_value(&self) -> &[u8];
    fn byte_at_index(&self) -> u8;
}

impl SharedToken for Token {
    fn get_value(&self) -> &[u8] {
        match self {
            Token::Identifier(identifier) => identifier.get_value(),
            Token::Literal(literal) => literal.get_value(),
        }
    }
    fn byte_at_index(&self, index: u8) -> u8 {
        match self {
            Token::Identifier(identifier) => identifier.byte_at_index(index),
            Token::Literal(literal) => literal.get_value(),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
#[allow(non_camel_case_types, unused)]
pub(crate) enum Identifier {
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

#[derive(PartialEq, Copy, Clone, Debug)]
#[allow(non_camel_case_types, unused)]
pub(crate) enum Literal {
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

impl SharedToken for Identifier {
    fn get_value(&self) -> &[u8] {
        match self {
            Identifier::BREAK => b"break",
            Identifier::CASE => b"case",
            Identifier::CATCH => b"catch",
            Identifier::CLASS => b"class",
            Identifier::CONST => b"const",
            Identifier::CONTINUE => b"continue",
            Identifier::DEBUGGER => b"debugger",
            Identifier::DEFAULT => b"default",
            Identifier::DELETE => b"delete",
            Identifier::DO => b"do",
            Identifier::ELSE => b"else",
            Identifier::EXPORT => b"export",
            Identifier::EXTENDS => b"extends",
            Identifier::FALSE => b"false",
            Identifier::FINALLY => b"finally",
            Identifier::FOR => b"for",
            Identifier::FUNCTION => b"function",
            Identifier::IF => b"if",
            Identifier::IMPORT => b"import",
            Identifier::IN => b"in",
            Identifier::INSTANCEOF => b"instanceof",
            Identifier::NEW => b"new",
            Identifier::NULL => b"null",
            Identifier::RETURN => b"return",
            Identifier::SUPER => b"super",
            Identifier::SWITCH => b"switch",
            Identifier::THIS => b"this",
            Identifier::THROW => b"throw",
            Identifier::TRUE => b"true",
            Identifier::TRY => b"try",
            Identifier::TYPEOF => b"typeof",
            Identifier::VAR => b"var",
            Identifier::VOID => b"void",
            Identifier::WHILE => b"while",
            Identifier::WITH => b"with",
            Identifier::STRICT_LET => b"let",
            Identifier::STRICT_STATIC => b"static",
            Identifier::STRICT_YIELD => b"yield",
            Identifier::STRICT_IMPLEMENTS => b"implements",
            Identifier::STRICT_INTERFACE => b"interface",
            Identifier::STRICT_PACKAGE => b"package",
            Identifier::STRICT_PRIVATE => b"private",
            Identifier::STRICT_PROTECTED => b"protected",
            Identifier::STRICT_PUBLIC => b"public",
            Identifier::MODULE_ASYNC => b"async",
            Identifier::FUTURE_ENUM => b"enum",
            Identifier::FUTURE_OLD_ABSTRACT => b"abstract",
            Identifier::FUTURE_OLD_BOOLEAN => b"boolean",
            Identifier::FUTURE_OLD_BYTE => b"byte",
            Identifier::FUTURE_OLD_CHAR => b"char",
            Identifier::FUTURE_OLD_DOUBLE => b"double",
            Identifier::FUTURE_OLD_FINAL => b"final",
            Identifier::FUTURE_OLD_FLOAT => b"float",
            Identifier::FUTURE_OLD_GOTO => b"goto",
            Identifier::FUTURE_OLD_INT => b"int",
            Identifier::FUTURE_OLD_LONG => b"long",
            Identifier::FUTURE_OLD_NATIVE => b"native",
            Identifier::FUTURE_OLD_SHORT => b"short",
            Identifier::FUTURE_OLD_SYNCHRONIZED => b"synchronized",
            Identifier::FUTURE_OLD_THROWS => b"throws",
            Identifier::FUTURE_OLD_TRANSIENT => b"transient",
            Identifier::FUTURE_OLD_VOLATILE => b"volatile",
            Identifier::SPECIAL_STRICT_ARGUMENTS => b"arguments",
            Identifier::SPECIAL_AS => b"as",
            Identifier::SPECIAL_STRICT_EVAL => b"eval",
            Identifier::SPECIAL_FROM => b"from",
            Identifier::SPECIAL_GET => b"get",
            Identifier::SPECIAL_OF => b"of",
            Identifier::SPECIAL_SET => b"set",
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
