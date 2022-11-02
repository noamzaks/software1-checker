use std::path::Path;
use std::process::exit;
use std::{env, process};

struct Assignment {
    filename: &'static str,
    tests: Vec<Test>,
}

struct Test {
    arguments: Vec<&'static str>,
    input: &'static str,
    output: &'static str,
}

fn main() {
    let tests = [
        Vec::from([Assignment {
            filename: "",
            tests: Vec::from([]),
        }]),
        Vec::from([
            Assignment {
                filename: "Assignment02Q01.java",
                tests: Vec::from([
                    Test {
                        arguments: Vec::from(["Before", "A", "E", "none"]),
                        input: "",
                        output: "A\nn\n",
                    },
                ]),
            },
            Assignment {
                filename: "Assignment02Q02.java",
                tests: Vec::from([
                    Test {
                        arguments: Vec::from(["4"]),
                        input: "",
                        output: "2.8952380952380956 3.141592653589793\n",
                    },
                    Test {
                        arguments: Vec::from(["100"]),
                        input: "",
                        output: "3.1315929035585537 3.141592653589793\n",
                    },
                ]),
            },
            Assignment {
                filename: "Assignment02Q03.java",
                tests: Vec::from([
                    Test {
                        arguments: Vec::from(["10"]),
                        input: "",
                        output: "The first 10 Fibonacci numbers are:\n1 1 2 3 5 8 13 21 34 55\nThe number of odd numbers is: 7\n",
                    },
                    Test {
                        arguments: Vec::from(["20"]),
                        input: "",
                        output: "The first 20 Fibonacci numbers are:\n1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 1597 2584 4181 6765\nThe number of odd numbers is: 14\n",
                    },
                ]),
            },
            Assignment {
                filename: "Assignment02Q04.java",
                tests: Vec::from([
                    Test {
                        arguments: Vec::from([]),
                        input: "",
                        output: "[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71]\n",
                    }
                ]),
            },
            Assignment {
                filename: "Assignment02Q05.java",
                tests: Vec::from([
                    Test {
                        arguments: Vec::from(["3", "1", "2", "3", "4", "5", "6", "7", "8", "9"]),
                        input: "",
                        output: "[1, 2, 3]\n[4, 5, 6]\n[7, 8, 9]\n\n[7, 4, 1]\n[8, 5, 2]\n[9, 6, 3]\n",
                    },
                    Test {
                        arguments: Vec::from(["4", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16"]),
                        input: "",
                        output: "[1, 2, 3, 4]\n[5, 6, 7, 8]\n[9, 10, 11, 12]\n[13, 14, 15, 16]\n\n[13, 9, 5, 1]\n[14, 10, 6, 2]\n[15, 11, 7, 3]\n[16, 12, 8, 4]\n",
                    }
                ]),
            },
        ]),
    ];

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "Usage: {} <directory> <username>\nWhere directory contains hw*-username.",
            args[0]
        );
        exit(-1);
    }

    let directory = &args[1];
    let root = Path::new(directory);
    env::set_current_dir(root).unwrap();
    let user = &args[2];

    for i in 1..13 {
        let filename = format!("hw{}-{}", i, user);
        let path = Path::new(&filename);
        if path.is_dir() {
            println!("Checking Homework #{}", i);
            env::set_current_dir(path.join("src")).unwrap();
            for assignment in &tests[i - 1] {
                println!("Checking {}", assignment.filename);
                for test in &assignment.tests {
                    let output = process::Command::new("java")
                        .arg(assignment.filename)
                        .args(&test.arguments)
                        .output();

                    match output {
                        Ok(output) => {
                            let actual_output = String::from_utf8(output.stdout).unwrap();
                            if actual_output == test.output {
                                println!("Test passed");
                            } else {
                                println!(
                                    "Test failed: Expected:\n----------------\n{}\n----------------\nGot:\n----------------\n{}\n----------------",
                                    test.output, actual_output
                                );
                            }
                        }
                        Err(error) => {
                            println!("Test failed: {}", error);
                        }
                    }
                }
            }
            env::set_current_dir(root).unwrap();
        }
    }
}
