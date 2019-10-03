extern crate protobuf;
extern crate sha1;

use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;

use std::convert::TryFrom;

use protobuf::json;

use sha1::{Sha1, Digest};

mod protos;
use protos::WillowTwoPlayerSaveGame::{WillowTwoPlayerSaveGame};

fn main() {

    let metadata = fs::metadata("./resources/Save0001.sav").unwrap();
    let file_len = metadata.len();

    let mut file = File::open("./resources/Save0001.sav").unwrap();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer);

    let mut buffer_checksum =  &buffer[..20];
    let mut buffer_data = &buffer[20..];

    let mut hasher = Sha1::new();
    hasher.input(&buffer_data);
    let res = hasher.result();
    let checksum = &res[..];


    assert_eq!(res[..], buffer_checksum[..]);

    let mut uncompressed_size_bytes = [0; 4];
    uncompressed_size_bytes.clone_from_slice(&buffer_data[..4]);

    let mut compressed_data = &buffer_data[4..];

    unsafe {
        let uncompressed_size_int = std::mem::transmute::<[u8; 4], u32>(uncompressed_size_bytes).to_be() as u64;
        println!("Uncompressed size: {}", uncompressed_size_int);

        let uncompressed_size = usize::try_from(uncompressed_size_int).unwrap();
        let uncompressed_data = minilzo::decompress(&compressed_data[..],  uncompressed_size);


    }


    let save_game = WillowTwoPlayerSaveGame::new();
    let string = protobuf::json::print_to_string(&save_game);
    println!("{}", string.unwrap());
    println!("Hello, world!");

    let data = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    let compressed = minilzo::compress(&data[..]);
    if compressed.is_ok() {
        let compressed_vector = compressed.unwrap();
        let decompressed = minilzo::decompress(&compressed_vector, data.len()).unwrap(); 
        println!("{}", String::from_utf8(decompressed).unwrap());
    }
}
