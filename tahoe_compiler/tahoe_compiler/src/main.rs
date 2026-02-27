fn main() {
    println!("Hello, world!");
}
// Tahoe Compiler v0.1
// Rust-based secure compiler

use std::env;
use std::fs;

#[derive(Debug)]
struct Token {
    value: String,
    }

    fn lexer(source: &str) -> Vec<Token> {
        source
                .lines()
                        .filter(|l| !l.trim().is_empty())
                                .map(|line| {
                                            let line = line.trim();
                                                        if !line.ends_with("</") {
                                                                        panic!("Syntax error: every line must end with '</' → {}", line);
                                                                                    }
                                                                                                Token {
                                                                                                                value: line.to_string(),
                                                                                                                            }
                                                                                                                                    })
                                                                                                                                            .collect()
                                                                                                                                            }

                                                                                                                                            fn parser(tokens: Vec<Token>) -> Vec<String> {
                                                                                                                                                tokens.into_iter().map(|t| t.value).collect()
                                                                                                                                                }

                                                                                                                                                fn security_model(ast: &[String]) {
                                                                                                                                                    for node in ast {
                                                                                                                                                            if node.contains("kernel") {
                                                                                                                                                                        panic!("Security violation: kernel access denied");
                                                                                                                                                                                }
                                                                                                                                                                                    }
                                                                                                                                                                                    }

                                                                                                                                                                                    fn emit_ir(ast: Vec<String>) {
                                                                                                                                                                                        println!("=== Tahoe Compilation Successful ===");
                                                                                                                                                                                            for node in ast {
                                                                                                                                                                                                    println!("IR::{}", node);
                                                                                                                                                                                                        }
                                                                                                                                                                                                        }

                                                                                                                                                                                                        fn main() {
                                                                                                                                                                                                            let args: Vec<String> = env::args().collect();

                                                                                                                                                                                                                if args.len() < 2 {
                                                                                                                                                                                                                        println!("Usage: tahoe_compiler <file.tahoe>");
                                                                                                                                                                                                                                return;
                                                                                                                                                                                                                                    }

                                                                                                                                                                                                                                        let source =
                                                                                                                                                                                                                                                fs::read_to_string(&args[1]).expect("Failed to read Tahoe source file");

                                                                                                                                                                                                                                                    let tokens = lexer(&source);
                                                                                                                                                                                                                                                        let ast = parser(tokens);
                                                                                                                                                                                                                                                            security_model(&ast);
                                                                                                                                                                                                                                                                emit_ir(ast);
                                                                                                                                                                                                                                                                }