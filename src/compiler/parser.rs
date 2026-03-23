use crate::compiler::ast::Expr;

use chumsky::Parser;
use chumsky::{
    // pratt::*,
    prelude::*,
};

#[allow(clippy::let_and_return)] // for some names + future expansion lol
pub fn parser<'src>() -> impl Parser<'src, &'src str, Expr<'src>> {
    let ident = text::ascii::ident().padded();

    let expr = recursive(|expr| {
        let int = text::int(10)
            .map(|s: &str| Expr::Num(s.parse().unwrap()))
            .padded();

        let atom = choice((
            int,
            expr.delimited_by(just('('), just(')')),
            ident.map(Expr::Var),
        ))
        .padded();

        let op = |c| just(c).padded();

        let unary = op('-')
            .repeated()
            .foldr(atom, |_op, rhs| Expr::Neg(Box::new(rhs)));

        let product = unary.clone().foldl(
            choice((
                op('*').to(Expr::Mul as fn(_, _) -> _),
                op('/').to(Expr::Div as fn(_, _) -> _),
            ))
            .then(unary)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        let sum = product.clone().foldl(
            choice((
                op('+').to(Expr::Add as fn(_, _) -> _),
                op('-').to(Expr::Sub as fn(_, _) -> _),
            ))
            .then(product)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        sum
    });

    let decl = text::ascii::keyword("let")
        .ignore_then(ident)
        .then_ignore(just('='))
        .then(expr.clone())
        .map(|(name, rhs)| Expr::Let {
            name,
            rhs: Box::new(rhs),
        });

    let function = text::ascii::keyword("fn")
        .ignore_then(ident)
        .then(ident.repeated().collect::<Vec<_>>())
        .then_ignore(just('='))
        .then(expr.clone())
        .map(|((name, args), body)| Expr::Fn {
            name,
            args,
            body: Box::new(body),
        });

    let valid_toplevels = choice((function, decl, expr));

    valid_toplevels.padded()
}
