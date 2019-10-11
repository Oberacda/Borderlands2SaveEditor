
mod protos;
mod hufman;

extern crate protobuf;
extern crate sha1;

use std::fs;
use std::fs::File;
use std::io::Read;

use std::convert::TryFrom;

use protos::WillowTwoPlayerSaveGame::{WillowTwoPlayerSaveGame};
use hufman::hufman::decode;

use sha1::{Sha1, Digest};

///
/// 
pub fn load_save(save_file_path: std::string::String) -> Result<WillowTwoPlayerSaveGame, std::io::Error> {
    let metadata = fs::metadata(&save_file_path)?;
    let _file_len = metadata.len();

    let mut file =  File::open(&save_file_path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    let buffer_checksum =  &buffer[..20];
    let buffer_data = &buffer[20..];

    let mut hasher = Sha1::new();
    hasher.input(&buffer_data);
    let res = hasher.result();

    assert_eq!(res[..], buffer_checksum[..]);

    let mut uncompressed_size_bytes = [0; 4];
    uncompressed_size_bytes.clone_from_slice(&buffer_data[..4]);

    let compressed_data = &buffer_data[4..];

    unsafe {
        let uncompressed_size_int = std::mem::transmute::<[u8; 4], u32>(uncompressed_size_bytes).to_be() as u64;
        println!("Uncompressed size: {}", uncompressed_size_int);

        let uncompressed_size = usize::try_from(uncompressed_size_int).unwrap();
        let uncompressed_data = minilzo::decompress(&compressed_data[..],  uncompressed_size).unwrap();

        decode(&uncompressed_data);

    }


    let save_game = WillowTwoPlayerSaveGame::new();
    return Ok(save_game);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let _save_file =  super::load_save("./resources/Save0001.sav".to_string());
    }
}
