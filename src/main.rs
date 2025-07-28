//! executable file integrates VM components
//! - parser
//! - byte-code compiler (in-memory only)
//! - byte-code interpreter

mod config;

use memmap2::Mmap;
use std::fs::File;
use std::io;
use std::io::Write;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let _argc = argv.len();
    arg(0, &argv[0]);
    for (argc, argv) in argv.iter().enumerate().skip(1) {
        arg(argc, argv);
        let file = File::open(argv).unwrap();
        let mmap = unsafe { Mmap::map(&file).unwrap() };
        // eprintln!("{:?}", &mmap[..] as &str);
        io::stdout().write_all(&mmap[..]).unwrap();
    }
}

fn arg(argc: usize, argv: &str) {
    eprintln!("argv[{argc}] = {argv:?}");
}
