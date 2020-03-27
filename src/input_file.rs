use std::process::exit;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Read;

pub fn open_file() -> String {
    let path = Path::new("main.jh");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => {
            eprintln!("couldn't open {}: {}", display, Error::description(&why));
            exit(1);
        },
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            eprintln!("couldn't read {}: {}", display, Error::description(&why));
            exit(1);
        },
        Ok(_) => return s,
    }
}
