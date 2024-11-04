use esc_pos_lib::printer;
mod args;
use esc_pos_lib::image;
use esc_pos_lib::qr;

fn print(p: printer::Printer, ip: String, port: u32, outfile: String) {
    if outfile != "" {
        p.print_file(outfile.as_str());
        return;
    }

    match p.print(ip, port) {
        Ok(_) => println!("Printed"),
        Err(e) => println!("Didn't print message {e}"),
    };
}

fn line_split(line: &String) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut word: String = String::new();
    for c in line.chars() {
        if c == '\n' {
            words.push(word);
            word = String::new();
        } else {
            word.push(c);
        }
    }
    words.push(word);
    words
}

fn print_qr(p: &mut printer::Printer, s: String) {
    let qr = qr::Qr::new(s.into_bytes());
    p.add_qr(qr);
}

fn print_file(p: &mut printer::Printer, file: &String) {
    let indata = std::fs::read_to_string(file).unwrap();
    let lines = line_split(&indata);
    for line in lines {
        if line.starts_with("@qr@") {
            let s = line.replace("@qr@", "");
            print_qr(p, s);
        } else if line.starts_with("@img@") {
            let s = line.replace("@img@", "");
            print_image(p, &s);
        } else {
            p.add_str(&line);
            p.add_str("\n");
        }
    }
}

fn print_image(p: &mut printer::Printer, file: &String) {
    let img = match image::image_from_path(file) {
        Ok(img) => img,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    p.add(img.export());
    p.add_str("\n");
}

fn main() {
    let args = args::get_args();

    let mut p = printer::Printer::new();

    let outfile: String = match args.outfile.as_str() {
        "" => String::new(),
        _ => args.outfile,
    };
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
            if args.img {
                print_image(&mut p, &args.file);
            } else {
                print_file(&mut p, &args.file);
            }
            p.cut();
            print(p, ip, port, outfile);
            return;
        }
    };
    match args.text.as_str() {
        "" => (),
        _ => {
            p.add_str(args.text.as_str());
            p.cut();
            print(p, ip, port, outfile);
            return;
        }
    };
}
