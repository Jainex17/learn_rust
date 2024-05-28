use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

pub fn main(){
    let file_path = "for_file_reader.txt";
    
    let readable_file = File::open(file_path).unwrap_or_else(|_|{
        eprintln!("well can't open file");
        std::process::exit(1);
    });
    let mut reader = BufReader::new(readable_file);
    
    let mut file_content = String::new();
    reader.read_to_string(&mut file_content).unwrap_or_else(|_|{
        eprintln!("well can't read file");
        std::process::exit(1);
    });
    
    println!("{file_path}:");
    println!("-----------------------------------------");
    println!("{}", file_content);
    println!("-----------------------------------------");

    let file = File::create("created_with_rust_code.txt").unwrap_or_else(|_|{
        eprintln!("can't create file");
        std::process::exit(1);
    });
    let mut writer = BufWriter::new(file);
    writer.write_all(b"this file is created with rust code hehehe").unwrap_or_else(|_|{
        eprintln!("can't write in file");
        std::process::exit(1);
    });

    println!("Good");
}