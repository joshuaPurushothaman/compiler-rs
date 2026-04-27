use clap::Parser as ClapParser;
use std::{fs, path::PathBuf};

mod compiler;
use compiler::{eval::eval, parser::parser};

use std::vec;

use chumsky::Parser;

use simply_colored::*;

// #[allow(unused_imports)]
// use ariadne::{Color, Label, Report, ReportKind, sources};

#[derive(ClapParser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input source file to compile
    #[arg(value_name = "FILE")]
    // src_file: Option<PathBuf>, -- this would be to have REPL functionality
    src_file: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let src = fs::read_to_string(cli.src_file).unwrap();

    let mut vars = vec![];
    let mut funcs = vec![];
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

    print_info!("Input: ", src, BLUE, CYAN);

    let parsed = p.parse(&src);

    match parsed.into_result() {
        Ok(ast) => {
            print_info!("Parses to:\n", ast, YELLOW, WHITE);

            match eval(&ast, &mut vars, &mut funcs) {
                Ok(output) => print_info!("Evaluates to: ", output, BLUE, GREEN),
                Err(eval_err) => print_info!("Evaluation error:\n", eval_err, YELLOW, RED),
            }
        }
        Err(parse_errs) => parse_errs
            .into_iter()
            .for_each(|e| print_info!("Parse error: \n", e, YELLOW, RED)),
    }
}
