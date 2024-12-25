#[derive(PartialEq, Eq, Debug)]
pub enum Block {
    Brace,
    Bracket,
    Parenthesis,
}

#[derive(Default, Debug)]
pub struct ParsingState {
    pub opened_blocks: Vec<Block>,
}
