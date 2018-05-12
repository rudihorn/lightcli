#![no_std]

pub extern crate heapless;
pub extern crate embedded_hal as hal;
pub extern crate nb;

#[cfg(test)]
mod tests;

mod tokenizer;
mod lexer;

use tokenizer::{Tokenizer};
use lexer::Lexer;
use hal::serial::Read;


pub enum StringType {
    Command,
    Key,
    Value
}

pub struct LightCli<SLEN> where SLEN: heapless::ArrayLength<u8> {
    tokenizer: Tokenizer<SLEN>,
    lexer: Lexer<SLEN>
}

impl<SLEN : heapless::ArrayLength<u8>> LightCli<SLEN> {
    pub fn new() -> Self {
        LightCli {
            tokenizer: Tokenizer::new(),
            lexer: Lexer::new(),
        }
    }

    pub fn parse_data<CB>(&mut self, callback: CB) -> nb::Result<(), tokenizer::Error> 
        where CB: FnMut(&str, &str, &str) -> () {
        self.lexer.parse_data(&mut self.tokenizer, callback)
    }

    pub fn fill<E>(&mut self, ser: &mut Read<u8, Error=E>) -> nb::Result<(), E> {
        self.tokenizer.fill(ser)
    }
}