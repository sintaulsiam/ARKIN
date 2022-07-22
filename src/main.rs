use std::io;
use std::io::prelude::*;

fn to_bin(str: &str) -> String {
    let mut result = String::new();

    for i in str.clone().as_bytes() {
        result += &format!("0{:b}", i);
        result += " ";
    }
    return result;
}

fn encoder(str: &str) -> String {
    let mut result = String::new();
    for (_, c) in str.chars().enumerate() {
        if c == '1' {
            result += "OK";
        } else if c == '0' {
            result += "NO";
        } else {
            result += " ";
        }
        result += " ";
    }
    return result;
}

fn bin_to_str(str: &str) -> String {
    match u8::from_str_radix(str, 2) {
        Ok(c) => (c as char).to_string(),
        Err(_) => String::new(),
    }
}

fn decoder(str: &str) -> String {
    let mut result = String::new();

    for i in str.split("  ") {
        let mut bin_str = String::new();

        for j in i.split(" ") {
            if j == "OK" {
                bin_str += "1";
            } else if j == "NO" {
                bin_str += "0"
            } else {
                continue;
            }
        }
        println!("{}", i);

        result += &bin_to_str(&bin_str);
        if i == " " {
            result += " ";
        }
    }
    return result;
}

fn main() {
    loop {
        let mut input_str = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input_str)
            .expect("can't read input");
        let input_str = input_str.trim();

        if input_str.to_lowercase() == "q" {
            break;
        }

        if input_str.to_lowercase().starts_with("-e") {
            let str = &input_str.trim()[3..input_str.len()];
            let result = to_bin(str);
            println!("{}", encoder(&result));
        } else if input_str.to_lowercase().starts_with("-d") {
            let str = &input_str.trim()[3..input_str.len()];
            let result = decoder(&str);
            println!("{}", result);
        }
    }
}
