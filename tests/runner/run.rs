use c_parser::{BracedBlock, Token, display_tokens, lex, linearise, parse};

use crate::runner::{_LINEAR_, _PARSED_, _TOKENS_, C0, CONTENTS, SIDE};

macro_rules! ret_err {
    ($con:expr) => {
        match $con {
            Ok(ok) => ok,
            Err(err) => return err,
        }
    };
}

#[derive(Copy, Clone)]
pub enum TestScope {
    Ast,
    AstNoError,
    Ssa,
}

impl TestScope {
    fn lex(content: &str) -> Result<Vec<Token>, String> {
        eprintln!("{SIDE}{_TOKENS_}{SIDE}{C0}");
        let (tokens, err) = lex(content, "").as_displayed_errors(&[("", content)]);
        eprintln!("\x1b[32m{}{C0}", display_tokens(tokens.as_ref().unwrap()));
        if err.is_empty() {
            Ok(tokens.unwrap())
        } else {
            Err(err)
        }
    }

    fn linearise(tree: BracedBlock, files: &[(&str, &str)]) -> String {
        eprintln!("{SIDE}{_LINEAR_}{SIDE}{C0}");
        let (ssa, err) = linearise(tree).as_displayed_errors(files);
        let ssa_str = ssa.unwrap().display();
        eprintln!("\x1b[32m{ssa_str}{C0}");
        if err.is_empty() { ssa_str } else { err }
    }

    fn parse(self, tokens: Vec<Token>, files: &[(&str, &str)]) -> Result<BracedBlock, String> {
        eprintln!("{SIDE}{_PARSED_}{SIDE}{C0}");
        let (tree, err) = parse(tokens).as_displayed_errors(files);
        eprintln!("\x1b[32m{}{C0}", tree.as_ref().unwrap());
        if !err.is_empty() && !matches!(self, Self::AstNoError) {
            Err(err)
        } else {
            Ok(tree.unwrap())
        }
    }

    pub fn run(self, content: &str) -> String {
        let files = &[("", content)];
        eprintln!("{SIDE}{CONTENTS}{SIDE}{C0}\n{content}");

        let tokens = ret_err!(Self::lex(content));
        let tree = ret_err!(self.parse(tokens, files));

        if !matches!(self, Self::Ssa) {
            return tree.to_string();
        }

        Self::linearise(tree, files)
    }
}
