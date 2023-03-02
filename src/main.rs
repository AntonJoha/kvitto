use esc_pos_lib::printer;
mod args;

fn print(p: printer::Printer, ip: String, port: u32) {

    match p.print(ip, port) {
        Ok(_) => println!("Printed"),
        Err(e) => println!("Didn't print message {e}"),
    };
}

fn print_file(p: &mut printer::Printer, file: &String) {
    let file = std::fs::read_to_string(file).unwrap();
    p.add_str(file.as_str());
}


fn main() {
    
    let args = args::get_args();

    let mut p = printer::Printer::new();
    
    let ip: String = match args.ip.as_str() {
        "" => "192.168.0.157".to_string(),
        _ => args.ip,
    };
    let port = match args.port {
        0 => 9100,
        _ => args.port,
    };
    match args.file.as_str() {
        "" => (),
        _ => {
            print_file(&mut p, &args.file);
            p.cut();
            print(p, ip, port);
            return;
        }
    };
    match args.text.as_str() {
        "" => (),
        _ => {
            p.add_str(args.text.as_str());
            p.cut();
            print(p, ip, port);
            return;
        }
    };
    
}
