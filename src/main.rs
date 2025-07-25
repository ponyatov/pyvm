#![allow(unused_variables)]

use memmap2::Mmap;
use std::fs::File;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let argc = argv.len();
    arg(0, &argv[0]);
    for (argc, argv) in argv.iter().enumerate().skip(1) {
        arg(argc, argv);
        let file = File::open(argv).unwrap();
        let mmap = unsafe { Mmap::map(&file).unwrap() };
    }
}

fn arg(argc: usize, argv: &str) {
    eprintln!("argv[{argc}] = {argv:?}");
}
