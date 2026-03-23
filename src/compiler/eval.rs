use crate::compiler::ast::Expr;

pub struct Variable {
    name: String,
    value: f64,
}

impl Variable {
    pub fn new(name: String, value: f64) -> Self {
        Self { name, value }
    }
}

pub fn eval<'src>(expr: &'src Expr<'src>, vars: &mut Vec<Variable>) -> Result<f64, String> {
    match expr {
        Expr::Num(x) => Ok(*x),
        Expr::Neg(a) => Ok(-eval(a, vars)?),
        Expr::Add(a, b) => Ok(eval(a, vars)? + eval(b, vars)?),
        Expr::Sub(a, b) => Ok(eval(a, vars)? - eval(b, vars)?),
        Expr::Mul(a, b) => Ok(eval(a, vars)? * eval(b, vars)?),
        Expr::Div(a, b) => Ok(eval(a, vars)? / eval(b, vars)?),
        Expr::Var(name) => {
            // Seek the variable stack for the most recent variable with that name
            if let Some(var) = vars.iter().rev().find(|var| var.name == *name) {
                Ok(var.value)
            } else {
                Err(format!("Cannot find variable `{name}` in scope"))
            }
        }
        Expr::Let { name, rhs } => {
            let rhs = eval(rhs, vars)?;
            vars.push(Variable {
                name: name.to_string(),
                value: rhs,
            });

            Ok(rhs)
        }
        // Expr::Call(_, exprs) => todo!(),
        // Expr::Fn {
        //     name,
        //     args,
        //     body,
        //     then,
        // } => todo!(),
        // _ => todo!()
        _ => Ok(200.0) // mf http code
    }
}
