#![allow(dead_code)]
#![allow(unused_variables)]

extern crate elapsed;
extern crate fnv;

mod fibonacci;
mod prime;
mod calc;
mod utils;
mod iterator_adapters;
mod matrix;
mod p001_to_p010;
mod p011_to_p020;

use std::collections::HashSet;
use p001_to_p010::*;
use p011_to_p020::*;

fn show_help() {
    println!("Welcome to my Project Euler solutions!");
    println!("To run, pass one or more numbers on the command line, a range, or 'all'. For example:");
    println!();
    println!("    $ projecteuler 2              // Runs problem 2");
    println!("    $ projecteuler 2 3 4 8        // Runs problems 2, 3, 4 and 8");
    println!("    $ projecteuler 2..10          // Runs problems 2 to 10, inclusive");
    println!("    $ projecteuler all            // Runs all problems");
    println!();
}

fn pnop() -> Option<u64> { None }

static SOLUTIONS: [fn() -> Option<u64>; 14] =
    [
        p001, p002, p003, p004, p005, p006, p007, p008, p009, p010,
        p011, p012, p013, p014
    ];

fn main() {
    let problems = parse_arguments();

    if problems.is_empty() {
        show_help();
        return;
    }

    println!("Problem     Time             Answer");
    println!("==========  ===============  ===============");

    for p in problems {
        let f = if p > SOLUTIONS.len() { pnop } else { SOLUTIONS[p - 1] };

        if f == pnop {
            println!("Problem {} has not been solved yet! Ignoring.", p);
        } else {
            utils::execute(p, f);
        }
    }
}

fn parse_arguments() -> Vec<usize> {
    let mut args = HashSet::new();

    for arg in std::env::args().skip(1) {
        if arg == "all" {
            args.extend(1..SOLUTIONS.len() + 1);
        }
        else if let Ok(n) = arg.parse::<usize>() {
            args.insert(n);
        } else if arg.contains("..") {
            let items = arg.split("..").collect::<Vec<&str>>();
            if items.len() == 2 {
                if let Ok(start) = items[0].parse::<usize>() {
                    if let Ok(end) = items[1].parse::<usize>() {
                        if end > start {
                            // inclusive range makes more sense from a human usability perspective
                            args.extend(start..end + 1);
                        }
                    }
                }
            }
        }
    }

    let mut v = args.into_iter().collect::<Vec<_>>();
    v.sort();
    v
}
