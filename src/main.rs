/*
 * eci-check: Ethereum WebAssembly contract compliance tool
 *
 * Copyright (c) 2018 Jake Lang
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#[macro_use] extern crate log;
extern crate eci;

use std::env;
use std::process;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use eci::checker::EcicChecker;

fn print_help_and_exit() -> ! {
    println!("Usage: eci-conform input.wasm");
    process::exit(-1);
}

fn main() {
    let cli_args: Vec<String> = env::args().collect();

    if cli_args.len() < 2 {
        print_help_and_exit();
    }
    
    let input_path = Path::new(&cli_args[1]);

    debug!("Attempting to open file {}", input_path.display());
    assert!(input_path.exists());

    let mut fp = File::open(input_path).unwrap();
    let mut wasm_input: Vec<u8> = Vec::new();

    fp.read_to_end(&mut wasm_input);

    let mut checker = EcicChecker::default(&wasm_input);
    checker.fire();
    checker.print_report()
}
