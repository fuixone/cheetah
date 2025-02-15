use std::fs;
use std::fs::File;
use std::io::Error;

use super::tokenizer::{Token, TokenTrait};

fn read_file(file_path: &str) {
    let data: &[u8] = fs::read(file_path).unwrap().as_ref();

    let tokens = Token::tokenize(data);
}
