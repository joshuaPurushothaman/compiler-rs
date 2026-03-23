#[derive(Debug)]
pub enum Expr<'src> {
    Num(f64),
    Var(&'src str),

    Neg(Box<Expr<'src>>),
    Add(Box<Expr<'src>>, Box<Expr<'src>>),
    Sub(Box<Expr<'src>>, Box<Expr<'src>>),
    Mul(Box<Expr<'src>>, Box<Expr<'src>>),
    Div(Box<Expr<'src>>, Box<Expr<'src>>),

    Call(&'src str, Vec<Expr<'src>>),
    Let {
        name: &'src str,
        rhs: Box<Expr<'src>>,
    },
    Fn {
        name: &'src str,
        args: Vec<&'src str>,
        body: Box<Expr<'src>>,
        then: Box<Expr<'src>>,
    },
}
