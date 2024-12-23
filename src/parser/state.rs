#[derive(Default)]
pub struct ParsingState {
    pub ternary: usize,
    pub wanting_colon: bool,
    pub closing_bracket: bool,
}
