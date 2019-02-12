#![feature(rustc_private)]

extern crate rustc_driver;
extern crate syntax;

fn run_compiler(args: Vec<String>) {
    rustc_driver::run(move || {
        rustc_driver::run_compiler(
            &args,
            Box::new(rustc_driver::RustcDefaultCalls),
            Some(Box::new(syntax::source_map::RealFileLoader)),
            Some(Box::new(std::io::stderr())),
        )
    });
}

fn main() {
    let mut args: Vec<_> = std::env::args().collect();
    args[0] = "rustc".to_string();
    eprintln!("first run");
    run_compiler(args.clone());
    eprintln!("second run");
    run_compiler(args.clone());
}
