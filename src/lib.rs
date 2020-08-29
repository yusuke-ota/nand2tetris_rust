pub mod tools;
use crate::tools::{CommandType, CompType, DestType, JumpType, Symbol};
use std::collections::HashMap;
use std::fs::File;

#[derive(Clone, Debug)]
pub struct Parser {
    buffer: Vec<String>,
    command: Option<String>,
}

pub trait IParser {
    fn new(file: File) -> Self;
    fn has_more_commands(&self) -> bool;
    fn advance(&mut self);
    fn command_type(&self) -> Result<CommandType, &'static str>;
    fn symbol(&self) -> Result<Symbol, &'static str>;
    fn dest(&self) -> Result<DestType, &'static str>;
    fn comp(&self) -> Result<CompType, &'static str>;
    fn jump(&self) -> Result<JumpType, &'static str>;
}

#[derive(Debug)]
pub struct SymbolTable {
    dictionary: HashMap<String, i32>,
}

pub trait ISymbolTable {
    fn new() -> Self;
    fn add_entry(&mut self, symbol: String, address: i32);
    fn contains(&self, symbol: &str) -> bool;
    fn get_address(&self, symbol: &str) -> Result<i32, &'static str>;
}
