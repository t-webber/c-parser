//! Walks a function call, declaration or definition, updating state and
//! creating symbols and basic blocks.

use crate::errors::api::{IntoError as _, Located};
use crate::lineariser::basic_block::{BasicBlocks, Id};
use crate::lineariser::state::LState;
use crate::lineariser::symbol::{Type, Value};
use crate::parser::api::{Ast, BracedBlock, FunctionCall, VariableName, VariableValue};

impl FunctionCall {
    /// Pushes some content into the [`BasicBlocks`].
    pub fn push_in(self, bbs: &mut BasicBlocks, state: &mut LState) -> Option<Id> {
        let Self { mut arguments, function_body, variable } = self;

        match variable.into_value() {
            VariableValue::AttributeVariable(attr) => {
                let loc = attr.location();
                if let Some((name, ret)) = attr.into_single_variable() {
                    declare_function(
                        name,
                        arguments,
                        ret.into_iter().map(Located::drop_location).collect(),
                        function_body,
                        state,
                    );
                } else {
                    state.push_error(
                        loc.fail("Found illegal comma in function declaration".to_owned()),
                    );
                }
                None
            }
            VariableValue::VariableName(loc, VariableName::UserDefined(name))
                if function_body.is_some() =>
            {
                state.push_error(loc.fail(format!("Missing return type for function {name}")));
                declare_function(loc.wrap(name), arguments, vec![], function_body, state);
                None
            }
            VariableValue::VariableName(loc, VariableName::Keyword(kwd))
                if function_body.is_some() =>
            {
                state.push_error(loc.fail(format!(
                    "Attempt to declare function with an invalid name, `{kwd}` is a keyword"
                )));
                None
            }
            VariableValue::VariableName(varloc, VariableName::UserDefined(name)) => {
                if let Some(func) = state.find_function(&name) {
                    let ty = func.ret.clone();
                    let fid = func.id;
                    let mut args = vec![];
                    let mut has_errors = false;
                    for arg in arguments {
                        let argloc = arg.location();
                        match arg.push_in(bbs, state) {
                            Some(Id::Found(id)) => args.push(id),
                            Some(Id::NotFound) => has_errors = true,
                            None => {
                                state.stat_not_expr(argloc);
                                has_errors = true;
                            }
                        }
                    }
                    if has_errors {
                        Some(Id::NotFound)
                    } else {
                        Some(state.push_element(Value::Call(fid, args), ty).into())
                    }
                } else {
                    state.push_error(varloc.fail(format!("Call of undeclared function {name}")));
                    None
                }
            }
            VariableValue::VariableName(loc, VariableName::Keyword(kwd)) => {
                if arguments.len() > 1 {
                    state.push_error(loc.fail(format!(
                        "Too many arguments in call to `{kwd}`: expected 1, got {}",
                        arguments.len()
                    )));
                    return None;
                }
                let Some(_) = arguments.pop() else {
                    state.push_error(
                        loc.fail(format!("Missing argument in call to `{kwd}`: expected 1, got 0")),
                    );
                    return None;
                };
                todo!()
            }
        }
    }
}

/// Declares a function with the given signature.
fn declare_function(
    name: Located<String>,
    arguments: Vec<Ast>,
    ret: Type,
    body: Option<BracedBlock>,
    state: &mut LState,
) {
    let mut args = vec![];
    for arg in arguments {
        if let Ast::Variable(arg_var) = arg {
            match arg_var.into_value() {
                VariableValue::AttributeVariable(arg_attr) => {
                    let loc = arg_attr.location();
                    if let Some((arg_name, arg_ty)) = arg_attr.into_single_variable() {
                        args.push((
                            arg_name,
                            arg_ty.into_iter().map(Located::drop_location).collect(),
                        ));
                    } else {
                        state.push_error(loc.fail("Missing argument name".to_owned()));
                        args.push((loc.wrap(String::new()), vec![]));
                    }
                }
                VariableValue::VariableName(loc, arg_name) => {
                    state.push_error(loc.fail("Missing argument type".to_owned()));
                    match arg_name {
                        VariableName::Keyword(_) => {
                            state.push_error(
                                loc.fail("Invalid argument name, shadows keyword.".to_owned()),
                            );
                            args.push((loc.wrap(String::new()), vec![]));
                        }
                        VariableName::UserDefined(vname) => args.push((loc.wrap(vname), vec![])),
                    }
                }
            }
        } else {
            let loc = arg.location();
            state.push_error(loc.fail("Expected argument declaration".to_owned()));
            args.push((loc.wrap(String::new()), vec![]));
        }
    }
    state.push_function(name, args, ret, body);
}
