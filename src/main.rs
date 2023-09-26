extern crate borderlands2;
extern crate protobuf;

use std::fs;
use std::process;

fn main() -> std::io::Result<()> {
    let save_path = std::string::String::from("./resources/Save0001.sav");
    let save_file = match borderlands2::load_save(save_path) {
        Ok(res) => res,
        Err(err) => {
            println!("Application error: {}", err);
            process::exit(1);
        }
    };

    let string_result = protobuf_json_mapping::print_to_string(&save_file);
    fs::write("dump.json", string_result.unwrap())?;
    Ok(())
}
