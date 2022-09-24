use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::env;
use std::path::Path;
use std::io::Write;
use std::fmt::Write as format;

// print the bytes of a binary file
fn print_lines(start : usize, end : usize, v : &Vec<Vec<u8>>, number_bytes_per_line : usize){
    for l in start..end+1 {
        let line = &v[l];

        let mut str_bytes = String::new();

        for byte in line {
            write!(str_bytes, "{:02x} ", byte).ok();
        }

        let conv_str = String::from_utf8_lossy(line);

        let b = l*number_bytes_per_line;

        println!("{:02}|{:02}: {}   {}", l, b, str_bytes, conv_str);
    }

    //print an empty lines after
    println!("");
}

// print the available commands
fn show_commands() -> String {
    let mut help = String::new();
    help.push_str("up      : go to previous line\n");
    help.push_str("down    : go to next line\n");
    help.push_str("help    : print help\n");
    help.push_str("line    : go to line n\n");
    help.push_str("go      : go to byte n\n");
    help.push_str("info    : information\n");
    help.push_str("modify  : modify current byte\n"); //todo
    help.push_str("search  : search text\n"); //todo
    help.push_str("write   : write to a new file\n"); //todo
    help.push_str("exit    : exit the program");

    return help;
}


fn up(start_line : &mut usize){
    if *start_line > 0 {
        *start_line -= 1;
    }
}

fn down(start_line : &mut usize, max_len : usize){
    if *start_line < max_len-1 {
        *start_line += 1;
    }
}

fn info(file_path : &str, max_len : usize ) -> String {
    let mut result = String::new();
    result.push_str("File            : ");
    result.push_str(&file_path);
    result.push_str("\n");
    result.push_str("Number of bytes : ");
    result.push_str(&max_len.to_string());

    return result;
}

fn flush(){
    //flush to avoid problem with priting
    match std::io::stdout().flush() {
        Ok(_) => (),
        Err(error) => println!("{}", error),
    }
}

fn goto_line(start_line : &mut usize, max_len : usize) -> String {
    let stdin = std::io::stdin();
    print!("line : ");

    flush();

    let mut line_to_go : String = String::new();

    //the line number
    stdin.read_line(&mut line_to_go).ok();
    let clean_number = line_to_go.trim();

    //empty result if ok, otherwise print error
    let mut result = String::new();

    match clean_number.parse::<usize>() {
        Ok(i) => if i < max_len {*start_line = i} else { result.push_str("Please enter a valid number") },
        Err(..) => result.push_str("Not a number!")
    };

    return result;
}

fn goto_byte(start_line : &mut usize, max_len : usize, number_bytes_per_line : usize) -> String {
    let stdin = std::io::stdin();
    print!("byte : ");

    flush();

    let mut byte_to_go : String = String::new();

    //the line number
    stdin.read_line(&mut byte_to_go).ok();
    let clean_number = byte_to_go.trim();

    //empty result if ok, otherwise print error
    let mut result = String::new();

    match clean_number.parse::<usize>() {
        Ok(i) => if i < max_len {*start_line = i/number_bytes_per_line} else { result.push_str("Please enter a valid number") },
        Err(..) => result.push_str("Not a number!")
    };

    return result;
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
    let max_per_line : usize = 8;

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
    
    let mut start_line = 0;

    let number_lines_printed = 10;

    let stdin = std::io::stdin();

    //to print results
    let mut result = String::new();

    loop {
        print!("\x1B[2J\x1B[1;1H");

        //print first N lines
        print_lines(start_line, start_line+number_lines_printed-1, &bytes_lines, max_per_line);

        if result != "" {
            println!("{}\n", result);
        }

        result = String::new();

        print!(">> ");

        //flush to avoid problem with priting
        flush();

        // recreated everytime to avoid stdin problems
        let mut user_command : String = String::new();

        //get user input
        stdin.read_line(&mut user_command).ok();

        let lines_number = bytes_lines.len();
        let bytes_number = bytes_lines.len()*max_per_line;

        //get clean command and match with good command
        let cleaned_command = user_command.trim();
        
        match cleaned_command {
            "up" => up(&mut start_line),
            "down" => down(&mut start_line, lines_number),
            "help" => result = show_commands(),
            "line" => result = goto_line(&mut start_line, lines_number),
            "go" => result = goto_byte(&mut start_line, bytes_number, max_per_line),
            "info" => result = info(file_path, bytes_number),
            "modify" => result.push_str("Not implemented yet"),
            "search" => result.push_str("Not implemented yet"),
            "write" => result.push_str("Not implemented yet"),
            "exit" => break,
            _ => result.push_str("Unknown command")
        }

        println!("");
    }

    println!("Bye");
}
