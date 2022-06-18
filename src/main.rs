use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use core::fmt::Write;


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
    let buffer = BufReader::new(File::open("/Users/loiclievre/sources/perso/hexa/blue.gb").unwrap());

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