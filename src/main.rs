use std::env;
use std::env::Args;
use std::process;
use std::fs;

use image4numworks::{NumImageFormat, file_to_num_image, save_numworks_image};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments !");
        process::exit(1);
    }

    let config = parse_args(&args);

    let data = file_to_num_image(&config.path, config.mode).unwrap_or_else(|e | {
        eprintln!("Error while convesrion  :");
        eprintln!("{e}");
        process::exit(1);
    });

    save_numworks_image(data, &config.output).unwrap_or_else(|e| {
        eprintln!("Error while saving image :");
        eprintln!("{e}");
        process::exit(1)
    });

    println!("Process ended successfuly !")
}

struct Config {
    path: String,
    mode: NumImageFormat,
    output: String,
}

fn parse_args(args: &[String]) -> Config {
    if args.len() == 2 {
        Config {
            path: args[1].clone(),
            output : format!("{}.nat", args[1]),
            mode: NumImageFormat::Simple,
        }
    }
    else if args.len() == 3 {
        let mode: NumImageFormat;
        if args[2] == "0".to_string() {
            mode = NumImageFormat::Simple;
        } 
        else {
            println!("Unsupported mode, use Simple mode.");
            mode = NumImageFormat::Simple;
        }
        Config {
            path: args[1].clone(),
            output : format!("{}.nat", args[1]),
            mode: mode
        }
    }
    else {
        let mode: NumImageFormat;
        if args[2] == "0".to_string() {
            mode = NumImageFormat::Simple;
        } 
        else {
            println!("Unsupported mode, use Simple mode.");
            mode = NumImageFormat::Simple;
        }
        Config { 
            path: args[1].clone(), 
            output: args[3].clone(), 
            mode: mode,
        }
    }
}
