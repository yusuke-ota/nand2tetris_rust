use std::collections::HashMap;

pub struct SymbolTable{
    dictionary: HashMap<String, i32>
}

impl SymbolTable{
    pub fn new() -> Self{
        let mut dictionary: HashMap<String, i32> = HashMap::new();
        const DEFINED_SYMBOL:[(&str, i32);23] =
            [   ("SP", 0),
                ("LCL", 1),
                ("ARG", 2),
                ("THIS", 3),
                ("THAT", 4),
                ("R0", 0),
                ("R1", 1),
                ("R2", 2),
                ("R3", 3),
                ("R4", 4),
                ("R5", 5),
                ("R6", 6),
                ("R7", 7),
                ("R8", 8),
                ("R9", 9),
                ("R10", 10),
                ("R11", 11),
                ("R12", 12),
                ("R13", 13),
                ("R14", 14),
                ("R15", 15),
                ("SCREEN", 16384),
                ("KBD", 24576)
            ];
        DEFINED_SYMBOL.iter().map(|&(symbol, address)| dictionary.insert(symbol.to_string(), address));

        Self{
            dictionary,
        }
    }

    pub fn add_entry(&mut self, symbol: String, address: i32){
        self.dictionary.insert(symbol, address);
    }

    pub fn contains(&self, symbol: &str) -> bool{
        self.dictionary.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> Result<i32, &'static str>{
        match self.dictionary.get(symbol){
            Some(&address) => Ok(address),
            None => Err("Such symbol dose not exist in SymbolTable"),
        }
    }
}