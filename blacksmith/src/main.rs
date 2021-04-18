extern crate clap;

use clap::{Arg, App};
use std::fs::File;
use std::io::Read;
use std::io::prelude::*;

pub mod process_character;

fn get_file_as_byte_vec(filename: String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("Couldn't read file");

    return buffer;
}

fn main() {
   let matches = App::new("Shining Force II Blacksmith")
        .version("0.1.0")
        .author("ashbachj")
        .about("Processes an Everdrive savestate to skip the annoying blacksmith part in Shining Force II")
        .arg(Arg::with_name("file")
                 .short("f")
                 .long("file")
                 .takes_value(true)
                 .help("Filename of save state"))
	.arg(Arg::with_name("output")
		.short("o")
		.long("output")
		.takes_value(true)
		.help("Filename of processed save state"))
        .get_matches();

    let filename = matches.value_of("file").unwrap_or("shining.sav");
    let output_filename = matches.value_of("output").unwrap_or("shining_out.sav");

    let mut buffer = get_file_as_byte_vec(filename.to_string());

    for current_character in 0..29 {
    	process_character::process_character(&mut buffer, current_character);
    }
    {
	let mut output_file = File::create(output_filename).expect("Unable to create file");
	output_file.write_all(&buffer).expect("Error writing file");
    }
}
