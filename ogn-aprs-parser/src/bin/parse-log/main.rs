use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use ogn_aprs_parser::model::ogn_status_message::OGNStatusMessage;

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    println!("{}", filename);
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[2];

    let lines = read_lines(filename.to_string());
    for line in lines {
        let unwrapped = line.unwrap();
        let str = unwrapped.as_str();
        let parsed = OGNStatusMessage::from_str(str, None);
        if let Ok(p) = parsed {
            println!(
                "[{},{},{}],",
                p.position.longitude, p.position.latitude, p.position.altitude
            );
        }
    }
}
