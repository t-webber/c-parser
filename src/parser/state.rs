#[derive(PartialEq, Eq)]
pub enum Block {
    Parenthesis,
    Brace,
    Bracket,
}

#[derive(Default)]
pub struct ParsingState {
    pub wanting_colon: bool,
    pub opened_blocks: Vec<Block>,
}
