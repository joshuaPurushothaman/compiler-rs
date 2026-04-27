use crate::compiler::ast::Expr;

pub struct Variable {
    name: String,
    value: f64,
}
#[derive(Clone)]
pub struct Function<'a> {
    name: String,
    args: Vec<String>,
    body: &'a Expr<'a>,
}

pub fn eval<'src>(
    expr: &'src Expr<'src>,
    vars: &mut Vec<Variable>,
    funcs: &mut Vec<Function<'src>>,
) -> Result<f64, String> {
    match expr {
        Expr::Num(x) => Ok(*x),
        Expr::Neg(a) => Ok(-eval(a, vars, funcs)?),
        Expr::Add(a, b) => Ok(eval(a, vars, funcs)? + eval(b, vars, funcs)?),
        Expr::Sub(a, b) => Ok(eval(a, vars, funcs)? - eval(b, vars, funcs)?),
        Expr::Mul(a, b) => Ok(eval(a, vars, funcs)? * eval(b, vars, funcs)?),
        Expr::Div(a, b) => Ok(eval(a, vars, funcs)? / eval(b, vars, funcs)?),
        Expr::Var(name) => {
            // Seek the variable stack for the most recent variable with that name
            if let Some(var) = vars.iter().rev().find(|var| var.name == *name) {
                Ok(var.value)
            } else {
                Err(format!("Cannot find variable `{name}` in scope"))
            }
        }
        Expr::Let { name, rhs, then } => {
            let rhs = eval(rhs, vars, funcs)?;
            vars.push(Variable {
                name: name.to_string(),
                value: rhs,
            });
            let output = eval(then, vars, funcs);
            vars.pop();

            output
        }
        Expr::Fn {
            name,
            args,
            body,
            then,
        } => {
            funcs.push(Function {
                name: (**name).to_string(),
                args: args.iter().map(|arg| arg.to_string()).collect(),
                body,
            });

            let output = eval(then, vars, funcs);
            funcs.pop();
            output
        }
        Expr::Call(name, args) => {
            // Find the last function on the stack with matching name
            let last_fn = funcs.iter().rev().find(|func| func.name == *name);

            if let Some(func) = last_fn {
                let arg_names = func.args.clone();
                let body = func.body;

                if arg_names.len() == args.len() {
                    let mut args = args
                        .iter()
                        .map(|arg| eval(arg, vars, funcs))
                        .zip(arg_names.iter())
                        .map(|(val, name)| {
                            Ok(Variable {
                                name: name.clone(),
                                value: val?,
                            })
                        })
                        .collect::<Result<_, String>>()?;

                    // Push the function's args on to the var stack, eval the fn, then pop them back off
                    let num_of_old_vars = vars.len();
                    vars.append(&mut args);

                    let output = eval(body, vars, funcs);
                    vars.truncate(num_of_old_vars);

                    output
                } else {
                    Err(format!(
                        "Wrong number of arguments for function `{name}`: expected {}, found {}",
                        arg_names.len(),
                        args.len(),
                    ))
                }
            } else {
                Err(format!("Cannot find function `{name}` in scope"))
            }
        }
    }
}
