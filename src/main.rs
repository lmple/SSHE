use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::env;
use std::path::Path;
use std::io::Write;
use std::fmt::Write as format;

fn printlines(start : usize, end : usize, v : &Vec<Vec<u8>>){
    for l in start..end+1 {
        let line = &v[l];

        let mut str_bytes = String::new();

        for byte in line {
            write!(str_bytes, "{:02x} ", byte).ok();
        }

        let conv_str = String::from_utf8_lossy(line);

        println!("{:02}: {}   {}", l, str_bytes, conv_str);
    }

    //print an empty lines after
    println!("");
}

fn show_commands() -> String {
    let mut help = String::new();
    help.push_str("u : go to previous line\n");
    help.push_str("d : go to next line\n");
    help.push_str("h : print help\n");
    help.push_str("g : go to line n\n");
    help.push_str("i : informations\n"); //todo
    help.push_str("m : modify\n"); //todo
    help.push_str("s : search\n"); //todo
    help.push_str("e : exit the program");

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

fn flush(){
    //flush to avoid problem with priting
    match std::io::stdout().flush() {
        Ok(_) => (),
        Err(error) => println!("{}", error),
    }
}

fn goto(start_line : &mut usize, max_len : usize) -> String {
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

    let mut start_line = 0;

    let number_lines_printed = 10;

    let stdin = std::io::stdin();

    //to print results
    let mut result = String::new();

    loop {
        print!("\x1B[2J\x1B[1;1H");

        //print first N lines
        printlines(start_line, start_line+number_lines_printed-1, &bytes_lines);

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

        let max_len = bytes_lines.len();

        //get clean command and match with good command
        let cleaned_command = user_command.trim();
        match cleaned_command {
            "u" => up(&mut start_line),
            "d" => down(&mut start_line, max_len),
            "h" => result = show_commands(),
            "g" => result = goto(&mut start_line, max_len),
            "i" => result.push_str("Not implemented yet"),
            "m" => result.push_str("Not implemented yet"),
            "s" => result.push_str("Not implemented yet"),
            "e" => break,
            _ => result.push_str("Unknown command")
        }
        println!("");
    }

    println!("Bye");
}
