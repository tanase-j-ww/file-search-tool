use std::env;
use walkdir::WalkDir;

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <directory> <pattern>", args[0]);
        std::process::exit(1);
    }
    let directory = &args[1];
    let pattern = &args[2];
    
    println!("directory: {} pattern: {}", directory, pattern);

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }
}
