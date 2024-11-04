use std::env;

fn print_help() {
    println!("Usage: kvitto");
    println!("Options:");
    println!("\t-h\n\t\tPrint this help message");
    println!("\t-a\n\t\tIp address of the printer");
    println!("\t-p\n\t\tSpecify the printer port");
    println!("\t-t\n\t\tSpecify the text to print");
    println!("\t-f\n\t\tSpecify the file to print");
    println!("\t-i\n\t\tSpecify the image to print");
}

pub struct Args {
    pub ip: String,
    pub port: u32,
    pub file: String,
    pub text: String,
    pub img: bool,
    pub outfile: String,
}

pub fn get_args() -> Args {
    let args = env::args().collect::<Vec<String>>();

    let mut ip = String::new();
    let mut port: u32 = 9100;
    let mut file = String::new();
    let mut text = String::new();
    let mut img = false;
    let mut outfile = String::new();
    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "-a" => ip = args[i + 1].clone(),
            "-p" => port = args[i + 1].parse::<u32>().unwrap(),
            "-f" => file = args[i + 1].clone(),
            "-t" => text = args[i + 1].clone(),
            "-i" => {
                img = true;
                file = args[i + 1].clone();
            }
            "-h" => {
                print_help();
                std::process::exit(0);
            }
            "-o" => outfile = args[i + 1].clone(),
            _ => (),
        };
    }

    Args {
        ip,
        port,
        file,
        text,
        img,
        outfile,
    }
}
