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
    println!("\t-o\n\t\tSpecify the output file");
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
    let mut port: u32 = 0;
    let mut file = String::new();
    let mut text = String::new();
    let mut img = false;
    let mut outfile = String::new();
    for (i, arg) in args.iter().enumerate() {
        let next = args.get(i + 1).map(|s| s.as_str());
        match arg.as_str() {
            "-a" => {
                if let Some(val) = next {
                    ip = val.to_string();
                }
            }
            "-p" => {
                if let Some(val) = next {
                    match val.parse::<u32>() {
                        Ok(p) => port = p,
                        Err(_) => {
                            println!("Invalid port: {}", val);
                            std::process::exit(1);
                        }
                    }
                }
            }
            "-f" => {
                if let Some(val) = next {
                    file = val.to_string();
                }
            }
            "-t" => {
                if let Some(val) = next {
                    text = val.to_string();
                }
            }
            "-i" => {
                if let Some(val) = next {
                    img = true;
                    file = val.to_string();
                }
            }
            "-h" => {
                print_help();
                std::process::exit(0);
            }
            "-o" => {
                if let Some(val) = next {
                    outfile = val.to_string();
                }
            }
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
