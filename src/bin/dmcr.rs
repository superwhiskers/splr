// DIMACS Model Checker in Rust
#![allow(unused_imports)]
use splr::config::Config;
use splr::solver::Solver;
use splr::traits::{SatSolverIF, ValidatorIF};
use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "dmcr", about = "DIMACS-format Model Checker in Rust")]
struct TargetOpts {
    #[structopt(parse(from_os_str))]
    #[structopt(short = "a", long = "assign")]
    assign: Option<std::path::PathBuf>,
    #[structopt(parse(from_os_str))]
    problem: std::path::PathBuf,
}

fn main() {
    let mut from_file = true;
    let mut found = false;
    let mut args = TargetOpts::from_args();
    if !args.problem.exists() {
        println!("{} does not exist.", args.problem.to_str().unwrap(),);
        return;
    }
    let mut config = Config::default();
    config.cnf = args.problem.clone();
    let mut s = Solver::build(config).expect("failed to load");
    if args.assign == None {
        args.assign = Some(PathBuf::from(format!(
            ".ans_{}",
            Path::new(&args.problem)
                .file_name()
                .unwrap()
                .to_string_lossy()
        )));
    }
    if let Some(f) = &args.assign {
        if let Ok(d) = File::open(f.as_path()) {
            if s.inject_assigmnent(&read_assignment(&mut BufReader::new(d)))
                .is_err()
            {
                println!(
                    "{} seems an unsat problem but no proof.",
                    args.problem.to_str().unwrap()
                );
                return;
            }
            found = true;
        }
    }
    if !found {
        if s.inject_assigmnent(&read_assignment(&mut BufReader::new(stdin())))
            .is_err()
        {
            println!(
                "{} seems an unsat problem but no proof.",
                args.problem.to_str().unwrap(),
            );
            return;
        }
        found = true;
        from_file = false;
    }
    if !found {
        println!("There's no assign file.");
        return;
    }
    match s.validate() {
        Some(v) => println!(
            "Invalid assignment for {} due to {:?}.",
            args.problem.to_str().unwrap(),
            v
        ),
        None if from_file => println!(
            "Valid assignment for {} found in {}.",
            &args.problem.to_str().unwrap(),
            &args.assign.unwrap().to_str().unwrap(),
        ),
        None => println!("Valid assignment for {}.", &args.problem.to_str().unwrap()),
    }
}

fn read_assignment(rs: &mut BufRead) -> Vec<i32> {
    let mut buf = String::new();
    loop {
        match rs.read_line(&mut buf) {
            Ok(0) => return Vec::new(),
            Ok(_) => {
                if buf.starts_with('c') {
                    buf.clear();
                    continue;
                }
                let mut v: Vec<i32> = Vec::new();
                for s in buf.split_whitespace() {
                    match s.parse::<i32>() {
                        Ok(0) => break,
                        Ok(x) => v.push(x),
                        Err(e) => panic!("{} by {}", e, s),
                    }
                }
                return v;
            }
            Err(e) => panic!("{}", e),
        }
    }
}