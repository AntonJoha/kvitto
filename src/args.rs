use std::env;


fn print_help() {
    println!("Usage: kvitto");
    println!("Options:");
    println!("\t-h\n\t\tPrint this help message");
    println!("\t-i\n\t\tIp address of the printer");
    println!("\t-p\n\t\tSpecify the printer port");
    println!("\t-t\n\t\tSpecify the text to print");
    println!("\t-f\n\t\tSpecify the file to print");
}


pub struct Args {
    pub ip: String,
    pub port: u32,
    pub file: String,
    pub text: String,
}


pub fn get_args() -> Args {

    let args = env::args().collect::<Vec<String>>();

    let mut ip = String::new();
    let mut port: u32 = 9100;
    let mut file = String::new();
    let mut text = String::new();


    for (i, arg) in args.iter().enumerate() {
        if arg == "-i" {
            ip = args[i + 1].clone();
        } else if arg == "-p" {
            port = args[i + 1].parse::<u32>().unwrap();
        } else if arg == "-f" {
            file = args[i + 1].clone();
        } else if arg == "-t" {
            text = args[i + 1].clone(); 
        }
        else if arg == "-h" {
            print_help();
            std::process::exit(0);
        }
    }

    Args {
        ip,
        port,
        file,
        text,
    }
}
