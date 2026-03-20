

// use clap::Parser;
// #[derive(Parser)]
// #[command(version, about, long_about = None)]
// struct Cli {
//     /// Input source file to compile
//     #[arg(value_name = "FILE")]
//     src_files: Option<PathBuf>,
// }

// fn main() {
//     let cli = Cli::parse();

//     // let src = match cli.src_files {
//     //     Some(path) => fs::read_to_string(path).expect("Failed to read file"),
//     //     None => {
//     //         // Read from stdin until EOF
//     //         // fs::read_to_string("/dev/stdin")
//     //         //     .expect("Failed to read from stdin")
//     //         println!("--- SysteML REPL ---")

//     //         let stdin = io::stdin();
//     //         for line in stdin.lock().lines() {}
//     //     }
//     // };

//     // println!("Source:\n{}", src);

//     let src = "let add = fn x y = x + y in
// let mul = fn x y = x * y in
// let x = mul (add 5 42) 2 in
// add x 3.5
// ";

//     let tokens = lexer()
//         .parse(src)
//         .into_result()
//         .unwrap_or_else(|errs| parse_failure(&errs[0], src));

//     let expr = parser()
//         .parse(tokens[..].split_spanned((0..src.len()).into()))
//         .into_result()
//         .unwrap_or_else(|errs| parse_failure(&errs[0], src));

//     let mut solver = Solver {
//         src,
//         vars: Vec::new(),
//     };
//     let program_ty = solver.check(&expr, &mut Vec::new());
//     println!("Result type: {:?}", solver.solve(program_ty));

//     let mut vm = Vm::default();
//     println!("Result: {:?}", vm.eval(&expr));
// }
