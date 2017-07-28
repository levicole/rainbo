extern crate rainbo;
use std::io::{self, Read, Write};
use rainbo::colors::{HSV, RGB};

fn main() {

    let mut buffer = String::new();
    let mut stdout = io::stdout();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => {
            let mut i = 0;
            for ch in buffer.chars() {
                let color = HSV::new(i % 360, 1.0, 1.0).unwrap();
                let rgb   = color.into_rgb();
                write!(
                    stdout,
                    "\x1b[38;2;{};{};{}m{}\x1b[0m",
                    rgb.r,
                    rgb.g,
                    rgb.b,
                    ch
                ).unwrap();
                i += 2;
            }
            println!("");
        },
        Err(_) => {
            panic!("Couldn't read from stdin");
        }
    }

}

