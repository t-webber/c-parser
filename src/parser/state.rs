#[derive(Default)]
pub struct ParsingState {
    pub parenthesis: usize,
    pub brackets: usize,
    pub braces: usize,
    pub ternary: usize,
    pub wanting_colon: bool,
}
