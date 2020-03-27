use std::process::exit;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Write;

pub fn save_file(out: String) {
    let path = Path::new("out.rs");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => {
            eprintln!("couldn't create {}: {}", display, Error::description(&why));
            exit(1);
        },
        Ok(file) => file,
    };

    if let Err(why) = file.write_all(out.as_bytes()) {
        eprintln!("couldn't write out {}: {}", display, Error::description(&why));
        exit(1);
    }
}