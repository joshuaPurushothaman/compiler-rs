mod compiler;
use compiler::{
    eval::{Variable, eval},
    parser::parser,
};

use std::vec;
use std::io::{self, BufRead, Write};

use chumsky::Parser;

use simply_colored::*;

// #[allow(unused_imports)]
// use ariadne::{Color, Label, Report, ReportKind, sources};

fn main() {
    println!("Enter expressions to be evaluated!");
    print!(">>> ");
    std::io::stdout().flush().unwrap();

    let mut vars = vec![
        Variable::new("x".to_string(), 3.0),
        Variable::new("sixseven".to_string(), 67.0),
    ];

    iter_lines_from_stdin()
        .map(|line| process_line(line, &mut vars)) // To take in a file, just cat src.syml | cargo run
        .for_each(drop) // see justfile for more
}

fn iter_lines_from_stdin() -> impl Iterator<Item = String> {
    let stdin = io::stdin();
    let lock = stdin.lock();

    // Panic if any line is not valid UTF-8
    lock.lines().map(|l| l.unwrap())
}

fn process_line(line: String, vars: &mut Vec<Variable>) {
    let p = parser();

    // given a header and body, make them pretty! lol
    macro_rules! print_info {
        ($header:expr, $body:expr, $header_color:expr, $body_color:expr) => {
            println!(
                "{RESET}{BOLD}{0}{1}{NO_BOLD}{2}{3:#?}{RESET}",
                $header_color, $header, $body_color, $body
            )
        };
    }

    print_info!("Input: ", line, BLUE, CYAN);

    let parsed = p.parse(&line);

    match parsed.into_result() {
        Ok(ast) => {
            print_info!("Parses to:\n", ast, YELLOW, WHITE);

            match eval(&ast, vars) {
                Ok(output) => print_info!("Evaluates to: ", output, BLUE, GREEN),
                Err(eval_err) => print_info!("Evaluation error:\n", eval_err, YELLOW, RED),
            }
        }
        Err(parse_errs) => parse_errs
            .into_iter()
            .for_each(|e| print_info!("Parse error: \n", e, YELLOW, RED)),
    }

    print!(">>> "); // TODO: move this silly thing somewhere besides the repl lol
    std::io::stdout().flush().unwrap();
}
