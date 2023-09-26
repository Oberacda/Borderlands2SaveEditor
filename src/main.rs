extern crate borderlands2;
extern crate protobuf;

use std::process;
use std::fs;

fn main() -> std::io::Result<()> {
    let save_path = std::string::String::from("./resources/Save0001.sav");
    let save_file_res = borderlands2::load_save(save_path);
    if save_file_res.is_err() {
        println!("Application error: {}", save_file_res.unwrap_err());
        process::exit(1);
    }

    let save_file = save_file_res.unwrap();
    let string_result = protobuf_json_mapping::print_to_string(&save_file);
    fs::write("dump.json", &string_result.unwrap())?;
    Ok(())
}
