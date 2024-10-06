use std::{env, fs, io::Write, process::Command};
use fikra:: parse_config;
use fikra_parser::node::AstArena;
mod fikra_entities;
mod fikra_tokenization;
mod fikra_parser;
mod fikra_generator;
mod fikra_errors;

use fikra_tokenization::tokenization::tokens;  
//use fikra_tokenization::to_asm::fikra_tokens_to_asm;

use fikra_parser::parser::Parser;
use fikra_generator::generator::Generator;


fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}


fn run() -> Result<(), String> {

    let args: Vec<String> = env::args().collect();

    let  file_path= parse_config(&args);

    println!("{file_path}");

    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Error reading file {}: {}", file_path, e))?;

    let tokens = tokens::tokenize(contents);

    let ast_arena = AstArena::new();

    let mut token_parser = Parser::new(&tokens,&ast_arena);

    let node_return = token_parser.parse_prog()
    .map_err(|e| format!("Parsing error: {}", e))?;

    let output_generator = Generator::new(node_return);
    let output_tree = output_generator.generate_program()
        .map_err(|e| format!("Generation error: {}", e))?;

    write_to_file("output.asm", &output_tree)?;
    println!("Generated the following asm string: \n {}", output_tree);

    run_command("nasm", &["-felf64", "output.asm"])?;
    run_command("ld", &["-o", "output", "output.o"])?;



    Ok(())
}

fn write_to_file(filename: &str, content: &str) -> Result<(), String> {
    let mut file = fs::File::create(filename)
        .map_err(|e| format!("Error creating file {}: {}", filename, e))?;

    file.write_all(content.as_bytes())
        .map_err(|e| format!("Error writing to file {}: {}", filename, e))?;

    println!("Successfully wrote ASM to file");
    Ok(())
}

fn run_command(command: &str, args: &[&str]) -> Result<(), String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute {}: {}", command, e))?;

    if !output.status.success() {
        return Err(format!("{} error: {}", command, String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}