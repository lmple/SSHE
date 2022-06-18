use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use core::fmt::Write;
use std::env;
use std::path::Path;

fn printlines(start : usize, end : usize, v : &Vec<Vec<u8>>){
    for l in start..end+1 {
        let line = &v[l];

        let mut str_line = String::new();

        for byte in line {
            write!(str_line, "{:02x} ", byte).ok();
        }

        println!("{}: {}", l, str_line);
    }

    //print an empty lines after
    println!("");
}

fn show_commands(){
    println!("u -> go to previous line");
    println!("d -> go to next line");
    println!("exit -> exit the program");

    //print an empty lines after
    println!("");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Please enter one filename");
        return;
    }

    let file_path = &args[1];

    if !Path::new(file_path).is_file() {
        println!("Please use a valid binary file");
        return;
    }

    let buffer = BufReader::new(File::open(file_path).unwrap());

    let mut count_per_line : usize = 0;
    let max_per_line : usize = 12;

    //all lines
    let mut bytes_lines : Vec<Vec<u8>> = Vec::new();

    let bytes = buffer.bytes();

    //a line of N bytes
    let mut bytes_line : Vec<u8> = Vec::new();

    //create array with bytes from the file
    for byte_or_error in bytes {
        let byte = byte_or_error.unwrap();
        bytes_line.push(byte);
        count_per_line+=1;

        if count_per_line % max_per_line == 0 {
            bytes_lines.push(bytes_line.clone());
            bytes_line.clear();
        }
    }


    let start_line = 0;


    let stdin = std::io::stdin();

    loop {
        //print first ten lines
        printlines(start_line, 9, &bytes_lines);

        print!(">> ");

        //flush to avoid problem with priting

        // recreated everytime to avoid stdin problems
        let mut user_command : String = String::new();

        //get user input
        stdin.read_line(&mut user_command).ok();

        //get clean command and match with good command
        let cleaned_command = user_command.trim();
        match cleaned_command {
            "u" => println!("up"),
            "d" => println!("down"),
            "h" => show_commands(),
            "exit" => break,
            _ => println!("Unknown command")
        }

        println!("Bye");
    }
}
