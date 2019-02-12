#![feature(rustc_private)]

extern crate rustc_driver;
extern crate syntax;

fn run_compiler_path(args: &[String], path: String) {
    let mut args = args.to_vec();
    args.push(path.clone());
    let handler = std::thread::spawn(move || {
        eprintln!(
            "pid {} tid {:?}",
            std::process::id(),
            std::thread::current().id()
        );
        eprintln!("path {}", path.clone());
        rustc_driver::run(move || {
            eprintln!(
                "pid {} tid {:?}",
                std::process::id(),
                std::thread::current().id()
            );
            rustc_driver::run_compiler(
                &args,
                Box::new(rustc_driver::RustcDefaultCalls),
                Some(Box::new(syntax::source_map::RealFileLoader)),
                Some(Box::new(std::io::stderr())),
            )
        })
    });
    handler.join().unwrap();
}

fn main() {
    /*
     *  assume two file path are specified as args[-1] and args[-2]
     */
    let mut args: Vec<_> = std::env::args().collect();
    args[0] = "rustc".to_string();
    eprintln!("first run");
    let len = args.len();
    run_compiler_path(&args[..len - 2], args[len - 2].clone());
    eprintln!("second run");
    run_compiler_path(&args[..len - 2], args[len - 1].clone());
}
