use esc_pos_lib::printer;
mod args;

fn main() {
    
    let args = args::get_args();

    let mut p = printer::Printer::new();
    p.add_str(args.text.as_str());
    p.cut();
    match p.print("192.168.0.157".to_string(), 9100) {
        Ok(_) => println!("Printed"),
        Err(e) => println!("Didn't print message {e}"),
    };
}
