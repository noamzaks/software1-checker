use std::error::Error;
use std::path::Path;
use std::process::exit;
use std::{env, process};

use loading::Loading;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Test {
    arguments: Vec<String>,
    input: String,
    output: String,
}

#[derive(Deserialize, Debug)]
struct Assignment {
    filename: String,
    tests: Vec<Test>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "Usage: {} <directory> <username>\nWhere directory contains hw*-username.",
            args[0]
        );
        exit(-1);
    }

    let mut total_passing = 0;
    let mut total_failing = 0;
    let loading = Loading::default();
    loading.text("Fetching latest tests");

    let tests = reqwest::blocking::get("https://gist.githubusercontent.com/noamzaks/38e9723f7f131ce2e2363957afa8324e/raw/tests.json")?.text()?;
    let tests: Vec<Vec<Assignment>> = serde_json::from_str(&tests).unwrap();
    loading.success("Fetched latest tests");
    loading.end();

    let directory = &args[1];
    let root = Path::new(directory);
    env::set_current_dir(root).unwrap();
    let user = &args[2];

    for i in 1..13 {
        let filename = format!("hw{}-{}", i, user);
        let path = Path::new(&filename);
        if path.is_dir() {
            env::set_current_dir(path.join("src")).unwrap();
            for assignment in &tests[i - 1] {
                let loading = Loading::default();
                loading.text(format!("Checking {}", assignment.filename));

                for test in &assignment.tests {
                    let output = process::Command::new("java")
                        .arg(&assignment.filename)
                        .args(&test.arguments)
                        .output();

                    match output {
                        Ok(output) => {
                            let actual_output = String::from_utf8(output.stdout).unwrap();
                            if actual_output == test.output {
                                loading.success("Test passed");
                                loading.end();
                                total_passing += 1;
                            } else {
                                loading.fail("Test failed");
                                loading.end();
                                println!(
                                    "Expected:\n----------------\n{}\n----------------\nGot:\n----------------\n{}\n----------------",
                                    test.output, actual_output
                                );
                                total_failing += 1;
                            }
                        }
                        Err(error) => {
                            println!("Test failed: {}", error);
                            loading.fail(format!("{} Incorrect", assignment.filename));
                            loading.end();
                            total_failing += 1;
                        }
                    }
                }
            }
            env::set_current_dir(root).unwrap();
        }
    }

    if total_failing == 0 {
        println!("All {} tests passed!", total_passing);
    } else {
        println!(
            "Results: {} failed, {} succeeded",
            total_failing, total_passing
        );
    }

    Ok(())
}
