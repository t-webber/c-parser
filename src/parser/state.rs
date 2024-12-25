#[derive(PartialEq, Eq, Debug)]
pub enum Block {
    Parenthesis,
    Brace,
    Bracket,
}

#[derive(Default, Debug)]
pub struct ParsingState {
    pub wanting_colon: bool,
    pub opened_blocks: Vec<Block>,
}
