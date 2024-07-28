use bincode::serialize as to_bincode;
use serde_cbor::to_vec as to_cbor;
use serde_json::to_string as to_json;
use serde_derive::{Serialize};
use std::io::prelude::*;
use std::fs::File;
use std::env;

#[derive(Serialize)]
struct City {
    name : String,
    population : usize,
    latitude : f64,
    longtitude : f64,
}
const BYTES_PER_LINE: usize = 16;
const INPUT : &'static [u8] = br#"
fn main() {
    println!("Hello, world!");
}
"#;

fn main() {
    let calabar = City {
        name : String::from("Calabar"),
        population : 470_000,
        latitude: 4.95,
        longtitude : 8.33,
    };
    let as_json  = to_json(&calabar).unwrap();
    let as_cbor  = to_cbor(&calabar).unwrap();
    let as_bincode = to_bincode(&calabar).unwrap();

    println!("json:\n{}\n",  &as_json);
    println!("cbor:\n{:?}\n",  &as_cbor);
    println!("bincode:\n{:?}\n",  &as_bincode);

    println!("json as UTF-8:\n{}\n",  String::from_utf8_lossy(as_json.as_bytes()));
    println!("cbor as UTF-8:\n{:?}\n",  String::from_utf8_lossy(&as_cbor));
    println!("bincode as UTF-8:\n{:?}\n",  String::from_utf8_lossy(&as_bincode));

    let mut buffer : Vec<u8> = vec!();
    INPUT.read_to_end(&mut buffer);
    let mut position_in_input  = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        print!("[0x{:08x}]", position_in_input);
        for byte in line {
            print!("{:02x} ", byte);
        }
        println!();
        position_in_input += BYTES_PER_LINE;
    }

    let arg1 = env::args().nth(1);
    let fname = arg1.expect("usage: fview FILENAME");
    let mut f  = File::open(&fname).expect("Unable to open file.");
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    while let Ok(_) = f.read_exact(&mut buffer) {
        print!("[0x{:08x}]", pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!(". "),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte),
            }
        }
        println!("");
        pos += BYTES_PER_LINE;
    }

}

