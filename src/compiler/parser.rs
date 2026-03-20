use crate::compiler::ast::Expr;

use chumsky::Parser;
use chumsky::{
    // input::{Input as _, MappedInput},
    // pratt::*,
    prelude::*,
};

pub fn parser<'src>() -> impl Parser<'src, &'src str, Expr<'src>> {
    let int = text::int(10)
        .map(|s: &str| Expr::Num(s.parse().unwrap()))
        .padded();

    let atom = int;

    let op = |c| just(c).padded();

    let unary = op('-')
        .repeated()
        .foldr(atom, |_op, rhs| Expr::Neg(Box::new(rhs)));

    let product = unary.foldl(
        choice((
            op('*').to(Expr::Mul as fn(_, _) -> _),
            op('/').to(Expr::Div as fn(_, _) -> _),
        ))
        .then(unary)
        .repeated(),
        |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
    );

    let sum = product.foldl(
        choice((
            op('+').to(Expr::Add as fn(_, _) -> _),
            op('-').to(Expr::Sub as fn(_, _) -> _),
        ))
        .then(product)
        .repeated(),
        |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
    );

    sum
}
