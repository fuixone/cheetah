use std::fs::File;
use std::fs;
use std::io::Error;

use super::tokenizer::Token;

fn read_file(file_path: &str) {
    let data = fs::read(file_path).unwrap();
    
    let tokens = Token::tokenize(data);
}