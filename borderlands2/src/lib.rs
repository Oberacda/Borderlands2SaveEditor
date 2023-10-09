include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
mod hufman;

extern crate protobuf;
extern crate sha1;
extern crate minilzo_rs;

use std::fs;
use std::fs::File;
use std::io::Read;

use std::convert::TryFrom;

use hufman::decode;

use protobuf::Message;
use sha1::{Digest, Sha1};

#[derive(Debug)]
pub enum LoadSaveError {
    IOError { msg: String },
    BufferError { msg: String },
    ParsingError { msg: String },
}

impl std::fmt::Display for LoadSaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoadSaveError::IOError { msg } => write!(f, "IOError: {}", msg),
            LoadSaveError::BufferError { msg } => write!(f, "BufferError: {}", msg),
            LoadSaveError::ParsingError { msg } => write!(f, "ParsingError: {}", msg),
        }
    }
}

///
///
pub fn load_save(
    save_file_path: &str,
) -> Result<WillowTwoPlayerSaveGame::WillowTwoPlayerSaveGame, LoadSaveError> {
    let metadata = match fs::metadata(&save_file_path) {
        Ok(file) => file,
        Err(msg) => {
            return Err(LoadSaveError::IOError {
                msg: msg.to_string(),
            })
        }
    };
    let _file_len = metadata.len();

    let mut file = match File::open(&save_file_path) {
        Ok(file) => file,
        Err(msg) => {
            return Err(LoadSaveError::IOError {
                msg: msg.to_string(),
            })
        }
    };
    let mut buffer = Vec::new();

    match file.read_to_end(&mut buffer) {
        Ok(_) => {}
        Err(msg) => {
            return Err(LoadSaveError::IOError {
                msg: msg.to_string(),
            })
        }
    };
    load_save_mem(buffer)
}

pub fn load_save_mem(
    buffer: Vec<u8>,
) -> Result<WillowTwoPlayerSaveGame::WillowTwoPlayerSaveGame, LoadSaveError> {
    if buffer.len() < 24 {
        return Err(LoadSaveError::ParsingError {
            msg: "Lenght to small".to_string(),
        });
    }

    let buffer_checksum = &buffer[..20];
    let buffer_data = &buffer[20..];

    let mut hasher = Sha1::new();
    hasher.update(buffer_data);
    let res = hasher.finalize();

    if res[..] != buffer_checksum[..] {
        return Err(LoadSaveError::ParsingError {
            msg: "Checksum does not match!".to_string(),
        });
    }

    let mut uncompressed_size_bytes = [0; 4];
    uncompressed_size_bytes.clone_from_slice(&buffer_data[..4]);

    let compressed_data = &buffer_data[4..];

    unsafe {
        let uncompressed_size_int =
            std::mem::transmute::<[u8; 4], u32>(uncompressed_size_bytes).to_be() as u64;
        println!("Uncompressed size: {}", uncompressed_size_int);

        let uncompressed_size = match usize::try_from(uncompressed_size_int) {
            Ok(size) => size,
            Err(_) => {
                return Err(LoadSaveError::ParsingError {
                    msg: "Could not decompress size!".to_string(),
                })
            }
        };
        let lzo = minilzo_rs::LZO::init().unwrap();
        let decompressed_data = match lzo.decompress_safe(compressed_data, uncompressed_size) {
            Ok(decompressed_data) => {decompressed_data}
            Err(err) => {
                return Err(LoadSaveError::ParsingError {
                    msg: format!("Could not decompress using LZO: {}", err)
                })
            }
        };


        handle_uncompressed_data(decompressed_data)
    }
}

pub unsafe fn handle_uncompressed_data(
    uncompressed_data: Vec<u8>,
) -> Result<WillowTwoPlayerSaveGame::WillowTwoPlayerSaveGame, LoadSaveError> {
    let mut inner_size_bytes = [0; 4];
    let mut magic_number_bytes = [0; 3];
    let mut version_bytes = [0; 4];
    let mut hash_bytes = [0; 4];
    let mut inner_uncompressed_size_bytes = [0; 4];
    if uncompressed_data.len() < 20 {
        return Err(LoadSaveError::ParsingError {
            msg: "Uncompressed buffer to small!".to_string(),
        });
    }

    inner_size_bytes.clone_from_slice(&uncompressed_data[..4]);
    let inner_size = std::mem::transmute::<[u8; 4], u32>(inner_size_bytes).to_be() as u64;
    println!("Inner size: {}", inner_size);

    magic_number_bytes.clone_from_slice(&uncompressed_data[4..7]);

    version_bytes.clone_from_slice(&uncompressed_data[7..11]);
    let version = std::mem::transmute::<[u8; 4], u32>(version_bytes).to_le() as u64;
    println!("Version: {}", version);

    hash_bytes.clone_from_slice(&uncompressed_data[11..15]);
    let hash = std::mem::transmute::<[u8; 4], u32>(hash_bytes).to_le() as u64;
    println!("Hash: {}", hash);

    inner_uncompressed_size_bytes.clone_from_slice(&uncompressed_data[15..19]);
    let inner_uncompressed_size =
        std::mem::transmute::<[u8; 4], i32>(inner_uncompressed_size_bytes).to_le() as usize;

    let inner_compressed_data = &uncompressed_data[19..];

    let inner_uncompressed_data = decode(inner_compressed_data, inner_uncompressed_size);

    let save_game_res = WillowTwoPlayerSaveGame::WillowTwoPlayerSaveGame::parse_from_bytes(
        inner_uncompressed_data.as_ref(),
    );
    let save_game = match save_game_res {
        Ok(save_game) => save_game,
        Err(msg) => {
            return Err(LoadSaveError::ParsingError {
                msg: msg.to_string(),
            })
        }
    };

    Ok(save_game)
}

#[cfg(test)]
mod tests {
    use std::env;

    #[test]
    fn load_save_test() {
        let cwd = env::current_dir().unwrap();
        let save_game_file_path = cwd
            .join("resources")
            .join("Save0001.sav");
        let save_game_file_path_string = save_game_file_path.to_str().unwrap();
        println!("{}", &save_game_file_path_string);

        let load_save_result = super::load_save(save_game_file_path_string);
        assert!(load_save_result.is_ok());
    }

    #[test]
    fn load_save_test_2() {
        let cwd = env::current_dir().unwrap();
        let save_game_file_path = cwd
            .join("resources")
            .join("Save0002.sav");
        let save_game_file_path_string = save_game_file_path.to_str().unwrap();

        let load_save_result = super::load_save(save_game_file_path_string);
        assert!(load_save_result.is_err());
    }
}
