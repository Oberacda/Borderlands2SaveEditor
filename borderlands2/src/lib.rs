
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

    let mut inner_size_bytes = [0; 4];
    let mut magic_number_bytes = [0; 3];
    let mut version_bytes = [0; 4];
    let mut hash_bytes = [0; 4];
    let mut inner_uncompressed_size_bytes = [0; 4];

    unsafe {
        let uncompressed_size_int = std::mem::transmute::<[u8; 4], u32>(uncompressed_size_bytes).to_be() as u64;
        println!("Uncompressed size: {}", uncompressed_size_int);

        let uncompressed_size = usize::try_from(uncompressed_size_int).unwrap();
        let uncompressed_data = minilzo::decompress(&compressed_data[..],  uncompressed_size).unwrap();

        inner_size_bytes.clone_from_slice(&uncompressed_data[..4]);
        let inner_size = std::mem::transmute::<[u8; 4], u32>(inner_size_bytes).to_be() as u64;

        magic_number_bytes.clone_from_slice(&uncompressed_data[4..7]);

        version_bytes.clone_from_slice(&uncompressed_data[7..11]);
        let version = std::mem::transmute::<[u8; 4], u32>(version_bytes).to_le() as u64;

        hash_bytes.clone_from_slice(&uncompressed_data[11..15]);
        let hash = std::mem::transmute::<[u8; 4], u32>(hash_bytes).to_le() as u64;

        inner_uncompressed_size_bytes.clone_from_slice(&uncompressed_data[15..19]);
        let inner_uncompressed_size = std::mem::transmute::<[u8; 4], i32>(inner_uncompressed_size_bytes ).to_le() as usize;

        let inner_compressed_data = &uncompressed_data[19..];

        let inner_uncompressed_data = decode(&inner_compressed_data, inner_uncompressed_size);

        let save_game_res = protobuf::parse_from_bytes::<WillowTwoPlayerSaveGame>(inner_uncompressed_data.as_ref());

        let save_game = save_game_res.unwrap();

        return Ok(save_game);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn load_save_test() {
        let _save_file =  super::load_save("./resources/Save0001.sav".to_string());
    }
}
