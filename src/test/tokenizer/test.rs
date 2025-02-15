use std::collections::HashMap;

use crate::read::tokenizer::{self, Identifier, Literal, TokenTrait, ValueType};
use crate::utils::test_utils;
use crate::{assert_eq_with_cleanup, assert_with_cleanup};

// Constants for Identifiers
const BREAK: Identifier = Identifier::BREAK;
const CASE: Identifier = Identifier::CASE;
const CATCH: Identifier = Identifier::CATCH;
const CLASS: Identifier = Identifier::CLASS;
const CONST: Identifier = Identifier::CONST;
const CONTINUE: Identifier = Identifier::CONTINUE;
const DEBUGGER: Identifier = Identifier::DEBUGGER;
const DEFAULT: Identifier = Identifier::DEFAULT;
const DELETE: Identifier = Identifier::DELETE;
const DO: Identifier = Identifier::DO;
const ELSE: Identifier = Identifier::ELSE;
const EXPORT: Identifier = Identifier::EXPORT;
const EXTENDS: Identifier = Identifier::EXTENDS;
const FALSE: Identifier = Identifier::FALSE;
const FINALLY: Identifier = Identifier::FINALLY;
const FOR: Identifier = Identifier::FOR;
const FUNCTION: Identifier = Identifier::FUNCTION;
const IF: Identifier = Identifier::IF;
const IMPORT: Identifier = Identifier::IMPORT;
const IN: Identifier = Identifier::IN;
const INSTANCEOF: Identifier = Identifier::INSTANCEOF;
const NEW: Identifier = Identifier::NEW;
const NULL: Identifier = Identifier::NULL;
const RETURN: Identifier = Identifier::RETURN;
const SUPER: Identifier = Identifier::SUPER;
const SWITCH: Identifier = Identifier::SWITCH;
const THIS: Identifier = Identifier::THIS;
const THROW: Identifier = Identifier::THROW;
const TRUE: Identifier = Identifier::TRUE;
const TRY: Identifier = Identifier::TRY;
const TYPEOF: Identifier = Identifier::TYPEOF;
const VAR: Identifier = Identifier::VAR;
const VOID: Identifier = Identifier::VOID;
const WHILE: Identifier = Identifier::WHILE;
const WITH: Identifier = Identifier::WITH;
const STRICT_LET: Identifier = Identifier::STRICT_LET;
const STRICT_STATIC: Identifier = Identifier::STRICT_STATIC;
const STRICT_YIELD: Identifier = Identifier::STRICT_YIELD;
const STRICT_IMPLEMENTS: Identifier = Identifier::STRICT_IMPLEMENTS;
const STRICT_INTERFACE: Identifier = Identifier::STRICT_INTERFACE;
const STRICT_PACKAGE: Identifier = Identifier::STRICT_PACKAGE;
const STRICT_PRIVATE: Identifier = Identifier::STRICT_PRIVATE;
const STRICT_PROTECTED: Identifier = Identifier::STRICT_PROTECTED;
const STRICT_PUBLIC: Identifier = Identifier::STRICT_PUBLIC;
const MODULE_ASYNC: Identifier = Identifier::MODULE_ASYNC;
const FUTURE_ENUM: Identifier = Identifier::FUTURE_ENUM;
const FUTURE_OLD_ABSTRACT: Identifier = Identifier::FUTURE_OLD_ABSTRACT;
const FUTURE_OLD_BOOLEAN: Identifier = Identifier::FUTURE_OLD_BOOLEAN;
const FUTURE_OLD_BYTE: Identifier = Identifier::FUTURE_OLD_BYTE;
const FUTURE_OLD_CHAR: Identifier = Identifier::FUTURE_OLD_CHAR;
const FUTURE_OLD_DOUBLE: Identifier = Identifier::FUTURE_OLD_DOUBLE;
const FUTURE_OLD_FINAL: Identifier = Identifier::FUTURE_OLD_FINAL;
const FUTURE_OLD_FLOAT: Identifier = Identifier::FUTURE_OLD_FLOAT;
const FUTURE_OLD_GOTO: Identifier = Identifier::FUTURE_OLD_GOTO;
const FUTURE_OLD_INT: Identifier = Identifier::FUTURE_OLD_INT;
const FUTURE_OLD_LONG: Identifier = Identifier::FUTURE_OLD_LONG;
const FUTURE_OLD_NATIVE: Identifier = Identifier::FUTURE_OLD_NATIVE;
const FUTURE_OLD_SHORT: Identifier = Identifier::FUTURE_OLD_SHORT;
const FUTURE_OLD_SYNCHRONIZED: Identifier = Identifier::FUTURE_OLD_SYNCHRONIZED;
const FUTURE_OLD_THROWS: Identifier = Identifier::FUTURE_OLD_THROWS;
const FUTURE_OLD_TRANSIENT: Identifier = Identifier::FUTURE_OLD_TRANSIENT;
const FUTURE_OLD_VOLATILE: Identifier = Identifier::FUTURE_OLD_VOLATILE;
const SPECIAL_STRICT_ARGUMENTS: Identifier = Identifier::SPECIAL_STRICT_ARGUMENTS;
const SPECIAL_AS: Identifier = Identifier::SPECIAL_AS;
const SPECIAL_STRICT_EVAL: Identifier = Identifier::SPECIAL_STRICT_EVAL;
const SPECIAL_FROM: Identifier = Identifier::SPECIAL_FROM;
const SPECIAL_GET: Identifier = Identifier::SPECIAL_GET;
const SPECIAL_OF: Identifier = Identifier::SPECIAL_OF;
const SPECIAL_SET: Identifier = Identifier::SPECIAL_SET;

// Constants for Literals
const BACKSLASH: Literal = Literal::BACKSLASH;
const CARRIAGE_RETURN: Literal = Literal::CARRIAGE_RETURN;
const LINE_FEED: Literal = Literal::LINE_FEED;
const SINGLE_QUOTE: Literal = Literal::SINGLE_QUOTE;
const DOUBLE_QUOTE: Literal = Literal::DOUBLE_QUOTE;
const BACK_TICK: Literal = Literal::BACK_TICK;
const PLUS: Literal = Literal::PLUS;
const MINUS: Literal = Literal::MINUS;
const EQUAL: Literal = Literal::EQUAL;
const ASTERISK: Literal = Literal::ASTERISK;
const SLASH: Literal = Literal::SLASH;
const GREATER_THAN: Literal = Literal::GREATER_THAN;
const LESS_THAN: Literal = Literal::LESS_THAN;
const AMPERSAND: Literal = Literal::AMPERSAND;
const SPACE: Literal = Literal::SPACE;
const TAB: Literal = Literal::TAB;
const COMMA: Literal = Literal::COMMA;
const PERIOD: Literal = Literal::PERIOD;
const SEMICOLON: Literal = Literal::SEMICOLON;
const COLON: Literal = Literal::COLON;
const EXCLAMATION_MARK: Literal = Literal::EXCLAMATION_MARK;
const QUESTION_MARK: Literal = Literal::QUESTION_MARK;
const PIPE: Literal = Literal::PIPE;
const CARET: Literal = Literal::CARET;
const OPEN_BRACKET_LEFT: Literal = Literal::OPEN_BRACKET_LEFT;
const OPEN_BRACKET_RIGHT: Literal = Literal::OPEN_BRACKET_RIGHT;
const SQUARE_BRACKET_LEFT: Literal = Literal::SQUARE_BRACKET_LEFT;
const SQUARE_BRACKET_RIGHT: Literal = Literal::SQUARE_BRACKET_RIGHT;
const CURLY_BRACKET_LEFT: Literal = Literal::CURLY_BRACKET_LEFT;
const CURLY_BRACKET_RIGHT: Literal = Literal::CURLY_BRACKET_RIGHT;
const TILDE: Literal = Literal::TILDE;
const DOLLAR: Literal = Literal::DOLLAR;
const AT: Literal = Literal::AT;
const UNDERSCORE: Literal = Literal::UNDERSCORE;
const PERCENT: Literal = Literal::PERCENT;

#[cfg(test)]
mod tests {
    use crate::{
        assert_with_cleanup,
        read::tokenizer::{
            self, CharType, Identifier, Literal, SharedToken, Token, TokenTrait, ValueType,
        },
        utils::test_utils,
    };

    use crate::test::tokenizer::test;

    #[test]
    fn test_final() {
        let content = r#"const"#;

        let file_path = test_utils::create_temp_js_file(content);

        let buffer: Vec<u8> = file_path.clone().into_bytes();

        let result = tokenizer::Token::tokenize(buffer);

        // Print the result for debugging
        match &result {
            Ok(tokens) => println!("File tokens succesfully processed: {:?}\n", tokens),
            Err(e) => {
                eprintln!("Failed to process file tokens: {:?}\n", e);
                // cleanup
                test_utils::remove_config_from_path(&file_path);
            }
        }

        assert_with_cleanup!(
            result.is_ok(),
            "File tokens should be processed successfully\n",
            &file_path
        );

        test_utils::remove_config_from_path(&file_path);
    }

    #[test]
    fn test_get_value() {
        assert_eq!(test::BREAK.get_value(), ValueType::Str("break"));
        assert_eq!(test::CASE.get_value(), ValueType::Str("case"));
        assert_eq!(test::CATCH.get_value(), ValueType::Str("catch"));
        assert_eq!(test::CLASS.get_value(), ValueType::Str("class"));
        assert_eq!(test::CONST.get_value(), ValueType::Str("const"));
        assert_eq!(test::CONTINUE.get_value(), ValueType::Str("continue"));
        assert_eq!(test::DEBUGGER.get_value(), ValueType::Str("debugger"));
        assert_eq!(test::DEFAULT.get_value(), ValueType::Str("default"));
        assert_eq!(test::DELETE.get_value(), ValueType::Str("delete"));
        assert_eq!(test::DO.get_value(), ValueType::Str("do"));
        assert_eq!(test::ELSE.get_value(), ValueType::Str("else"));
        assert_eq!(test::EXPORT.get_value(), ValueType::Str("export"));
        assert_eq!(test::EXTENDS.get_value(), ValueType::Str("extends"));
        assert_eq!(test::FALSE.get_value(), ValueType::Str("false"));
        assert_eq!(test::FINALLY.get_value(), ValueType::Str("finally"));
        assert_eq!(test::FOR.get_value(), ValueType::Str("for"));
        assert_eq!(test::FUNCTION.get_value(), ValueType::Str("function"));
        assert_eq!(test::IF.get_value(), ValueType::Str("if"));
        assert_eq!(test::IMPORT.get_value(), ValueType::Str("import"));
        assert_eq!(test::IN.get_value(), ValueType::Str("in"));
        assert_eq!(test::INSTANCEOF.get_value(), ValueType::Str("instanceof"));
        assert_eq!(test::NEW.get_value(), ValueType::Str("new"));
        assert_eq!(test::NULL.get_value(), ValueType::Str("null"));
        assert_eq!(test::RETURN.get_value(), ValueType::Str("return"));
        assert_eq!(test::SUPER.get_value(), ValueType::Str("super"));
        assert_eq!(test::SWITCH.get_value(), ValueType::Str("switch"));
        assert_eq!(test::THIS.get_value(), ValueType::Str("this"));
        assert_eq!(test::THROW.get_value(), ValueType::Str("throw"));
        assert_eq!(test::TRUE.get_value(), ValueType::Str("true"));
        assert_eq!(test::TRY.get_value(), ValueType::Str("try"));
        assert_eq!(test::TYPEOF.get_value(), ValueType::Str("typeof"));
        assert_eq!(test::VAR.get_value(), ValueType::Str("var"));
        assert_eq!(test::VOID.get_value(), ValueType::Str("void"));
        assert_eq!(test::WHILE.get_value(), ValueType::Str("while"));
        assert_eq!(test::WITH.get_value(), ValueType::Str("with"));
        assert_eq!(test::STRICT_LET.get_value(), ValueType::Str("let"));
        assert_eq!(test::STRICT_STATIC.get_value(), ValueType::Str("static"));
        assert_eq!(test::STRICT_YIELD.get_value(), ValueType::Str("yield"));
        assert_eq!(
            test::STRICT_IMPLEMENTS.get_value(),
            ValueType::Str("implements")
        );
        assert_eq!(
            test::STRICT_INTERFACE.get_value(),
            ValueType::Str("interface")
        );
        assert_eq!(test::STRICT_PACKAGE.get_value(), ValueType::Str("package"));
        assert_eq!(test::STRICT_PRIVATE.get_value(), ValueType::Str("private"));
        assert_eq!(
            test::STRICT_PROTECTED.get_value(),
            ValueType::Str("protected")
        );
        assert_eq!(test::STRICT_PUBLIC.get_value(), ValueType::Str("public"));
        assert_eq!(test::MODULE_ASYNC.get_value(), ValueType::Str("async"));
        assert_eq!(test::FUTURE_ENUM.get_value(), ValueType::Str("enum"));
        assert_eq!(
            test::FUTURE_OLD_ABSTRACT.get_value(),
            ValueType::Str("abstract")
        );
        assert_eq!(
            test::FUTURE_OLD_BOOLEAN.get_value(),
            ValueType::Str("boolean")
        );
        assert_eq!(test::FUTURE_OLD_BYTE.get_value(), ValueType::Str("byte"));
        assert_eq!(test::FUTURE_OLD_CHAR.get_value(), ValueType::Str("char"));
        assert_eq!(
            test::FUTURE_OLD_DOUBLE.get_value(),
            ValueType::Str("double")
        );
        assert_eq!(test::FUTURE_OLD_FINAL.get_value(), ValueType::Str("final"));
        assert_eq!(test::FUTURE_OLD_FLOAT.get_value(), ValueType::Str("float"));
        assert_eq!(test::FUTURE_OLD_GOTO.get_value(), ValueType::Str("goto"));
        assert_eq!(test::FUTURE_OLD_INT.get_value(), ValueType::Str("int"));
        assert_eq!(test::FUTURE_OLD_LONG.get_value(), ValueType::Str("long"));
        assert_eq!(
            test::FUTURE_OLD_NATIVE.get_value(),
            ValueType::Str("native")
        );
        assert_eq!(test::FUTURE_OLD_SHORT.get_value(), ValueType::Str("short"));
        assert_eq!(
            test::FUTURE_OLD_SYNCHRONIZED.get_value(),
            ValueType::Str("synchronized")
        );
        assert_eq!(
            test::FUTURE_OLD_THROWS.get_value(),
            ValueType::Str("throws")
        );
        assert_eq!(
            test::FUTURE_OLD_TRANSIENT.get_value(),
            ValueType::Str("transient")
        );
        assert_eq!(
            test::FUTURE_OLD_VOLATILE.get_value(),
            ValueType::Str("volatile")
        );
        assert_eq!(
            test::SPECIAL_STRICT_ARGUMENTS.get_value(),
            ValueType::Str("arguments")
        );
        assert_eq!(test::SPECIAL_AS.get_value(), ValueType::Str("as"));
        assert_eq!(
            test::SPECIAL_STRICT_EVAL.get_value(),
            ValueType::Str("eval")
        );
        assert_eq!(test::SPECIAL_FROM.get_value(), ValueType::Str("from"));
        assert_eq!(test::SPECIAL_GET.get_value(), ValueType::Str("get"));
        assert_eq!(test::SPECIAL_OF.get_value(), ValueType::Str("of"));
        assert_eq!(test::SPECIAL_SET.get_value(), ValueType::Str("set"));

        assert_eq!(test::BACKSLASH.get_value(), ValueType::U8(0x5c));
        assert_eq!(test::CARRIAGE_RETURN.get_value(), ValueType::U8(0x0D));
        assert_eq!(test::LINE_FEED.get_value(), ValueType::U8(0x0A));
        assert_eq!(test::SINGLE_QUOTE.get_value(), ValueType::U8(0x27));
        assert_eq!(test::DOUBLE_QUOTE.get_value(), ValueType::U8(0x22));
        assert_eq!(test::BACK_TICK.get_value(), ValueType::U8(0x60));
        assert_eq!(test::PLUS.get_value(), ValueType::U8(0x2B));
        assert_eq!(test::MINUS.get_value(), ValueType::U8(0x2D));
        assert_eq!(test::EQUAL.get_value(), ValueType::U8(0x3D));
        assert_eq!(test::ASTERISK.get_value(), ValueType::U8(0x2A));
        assert_eq!(test::SLASH.get_value(), ValueType::U8(0x2F));
        assert_eq!(test::GREATER_THAN.get_value(), ValueType::U8(0x3E));
        assert_eq!(test::LESS_THAN.get_value(), ValueType::U8(0x3C));
        assert_eq!(test::AMPERSAND.get_value(), ValueType::U8(0x26));
        assert_eq!(test::SPACE.get_value(), ValueType::U8(0x20));
        assert_eq!(test::TAB.get_value(), ValueType::U8(0x09));
        assert_eq!(test::COMMA.get_value(), ValueType::U8(0x2C));
        assert_eq!(test::PERIOD.get_value(), ValueType::U8(0x2E));
        assert_eq!(test::SEMICOLON.get_value(), ValueType::U8(0x3B));
        assert_eq!(test::COLON.get_value(), ValueType::U8(0x3A));
        assert_eq!(test::EXCLAMATION_MARK.get_value(), ValueType::U8(0x21));
        assert_eq!(test::QUESTION_MARK.get_value(), ValueType::U8(0x3F));
        assert_eq!(test::PIPE.get_value(), ValueType::U8(0x7C));
        assert_eq!(test::CARET.get_value(), ValueType::U8(0x5E));
        assert_eq!(test::OPEN_BRACKET_LEFT.get_value(), ValueType::U8(0x28));
        assert_eq!(test::OPEN_BRACKET_RIGHT.get_value(), ValueType::U8(0x29));
        assert_eq!(test::SQUARE_BRACKET_LEFT.get_value(), ValueType::U8(0x5B));
        assert_eq!(test::SQUARE_BRACKET_RIGHT.get_value(), ValueType::U8(0x5D));
        assert_eq!(test::CURLY_BRACKET_LEFT.get_value(), ValueType::U8(0x7B));
        assert_eq!(test::CURLY_BRACKET_RIGHT.get_value(), ValueType::U8(0x7D));
        assert_eq!(test::TILDE.get_value(), ValueType::U8(0x7E));
        assert_eq!(test::DOLLAR.get_value(), ValueType::U8(0x24));
        assert_eq!(test::AT.get_value(), ValueType::U8(0x40));
        assert_eq!(test::UNDERSCORE.get_value(), ValueType::U8(0x5F));
        assert_eq!(test::PERCENT.get_value(), ValueType::U8(0x25));
    }

    #[test]
    fn test_to_string() {
        let token = Token::Identifier(Identifier::CASE);
        assert_eq!(token.to_string(), "case");

        let token_u8 = Token::Literal(Literal::ASTERISK);
        assert_eq!(token_u8.to_string(), "42");
    }

    #[test]
    fn test_get_next_possible_char_in_token_based_on_index() {
        let token_default = Token::Identifier(Identifier::DEFAULT);

        assert_eq!(
            Token::get_next_possible_char_in_token_based_on_index(0, &token_default),
            Some(CharType::Char('d'))
        );
        assert_eq!(
            Token::get_next_possible_char_in_token_based_on_index(1, &token_default),
            Some(CharType::Char('e'))
        );
        assert_eq!(
            Token::get_next_possible_char_in_token_based_on_index(2, &token_default),
            Some(CharType::Char('f'))
        );
        assert_eq!(
            Token::get_next_possible_char_in_token_based_on_index(3, &token_default),
            Some(CharType::Char('a'))
        );
        assert_eq!(
            Token::get_next_possible_char_in_token_based_on_index(4, &token_default),
            Some(CharType::Char('u'))
        );
        assert_eq!(
            Token::get_next_possible_char_in_token_based_on_index(5, &token_default),
            Some(CharType::Char('l'))
        );
        assert_eq!(
            Token::get_next_possible_char_in_token_based_on_index(6, &token_default),
            Some(CharType::Char('t'))
        );
        assert_eq!(
            Token::get_next_possible_char_in_token_based_on_index(7, &token_default),
            None
        );
        assert_eq!(
            Token::get_next_possible_char_in_token_based_on_index(100, &token_default),
            None
        );

        let token_carriage_return = Token::Literal(Literal::CARRIAGE_RETURN);

        assert_eq!(
            Token::get_next_possible_char_in_token_based_on_index(0, &token_carriage_return),
            Some(CharType::U8(0x0D))
        );
        assert_eq!(
            Token::get_next_possible_char_in_token_based_on_index(1, &token_carriage_return),
            None
        );
        assert_eq!(
            Token::get_next_possible_char_in_token_based_on_index(20, &token_carriage_return),
            None
        );
    }

    #[test]
    fn test_set_possible_tokens_from_char() {
        let mut current_chars: Vec<CharType> = Vec::new();
        let mut current_possible_tokens: Vec<Token> = Vec::new();

        Token::set_possible_tokens_from_char(&mut current_chars, &mut current_possible_tokens, 'b');

        assert_eq!(current_possible_tokens.len(), 1);
    }
}
