use std::collections::HashMap;

use crate::Token;

pub struct LexResult {
    symbols: HashMap<String, u32>,
    tokens: Vec<Token>,
}
