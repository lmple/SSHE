use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use core::fmt::Write;
use std::env;
use std::path::Path;

fn printlines(start : usize, end : usize, v : Vec<Vec<u8>>){
    for l in start..end+1 {
        let line = &v[l];

        let mut str_line = String::new();

        for byte in line {
            write!(str_line, "{:02x} ", byte);
        }


        println!("{}: {}", l, str_line);
    }
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

    let mut bytes_lines : Vec<Vec<u8>> = Vec::new();

    let bytes = buffer.bytes();

    let mut bytes_line : Vec<u8> = Vec::new();

    for byte_or_error in bytes {
        let byte = byte_or_error.unwrap();
        bytes_line.push(byte);
        count_per_line+=1;

        if count_per_line % max_per_line == 0 {
            bytes_lines.push(bytes_line.clone());
            bytes_line.clear();
        }
    }

    printlines(0, 3, bytes_lines);
}
