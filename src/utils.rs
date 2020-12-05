use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub fn load_input(filename: &str) -> Vec<String> {
    let path = Path::new(filename);

    let file: File = match File::open(&path) {
        Err(why) => panic!("couldn't open {} because {}", filename, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(&file).lines();
    return reader.map(|line| line.unwrap()).collect();
}