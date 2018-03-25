extern crate elapsed;

mod fibonacci;
mod prime;
mod utils;
mod p001_to_p010;

use std::collections::HashSet;
use p001_to_p010::*;

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

static SOLUTIONS: [fn(); 5] = [p001, p002, p003, p004, p005];

fn main() {
    let problems = parse_arguments();

    if problems.is_empty() {
        show_help();
        return;
    }

    for p in problems {
        if p > SOLUTIONS.len() {
            println!("Problem {} has not been solved yet! Ignoring.", p);
        }
        else {
            utils::execute(p,SOLUTIONS[p - 1]);
        }
    }
}

fn parse_arguments() -> Vec<usize> {
    let mut args = HashSet::new();

    for arg in std::env::args().skip(1) {
        if arg == "all" {
            args.extend(1..SOLUTIONS.len());
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
