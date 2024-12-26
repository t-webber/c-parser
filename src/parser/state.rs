#[derive(PartialEq, Eq, Debug)]
pub enum BlockState {
    Brace,
    Bracket,
    Parenthesis,
}

#[derive(Default, Debug)]
pub struct ParsingState {
    pub opened_blocks: Vec<BlockState>,
}
