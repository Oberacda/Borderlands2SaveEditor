extern crate protobuf;
extern crate sha1;

mod protos;
mod hufman;


pub mod save_files {
    //!
    //! Module to load and save savefile instances for borderlands2.
    //!

    use std::convert::TryFrom;
    use std::error::Error;
    use std::fmt;
    use std::fs::File;
    use std::io::Read;
    use std::num::TryFromIntError;
    use std::path::Path;

    use sha1::{Digest, Sha1};

    use crate::hufman::hufman::decode;
    use crate::protos::WillowTwoPlayerSaveGame::WillowTwoPlayerSaveGame;

    ///
    /// Struct to manage a error from the save file management function.
    ///
    pub struct SaveFileError {
        ///
        /// Error code.
        ///
        /// # Ranges
        ///
        /// * 100..=199 - Program or checksum errors.
        /// * 200..=299 - I/O Errors.
        ///
        pub code: i32,

        ///
        /// Message describing the error.
        ///
        pub message: String,
    }

    impl fmt::Display for SaveFileError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let err_msg = match self.code {
                0 => format!("General error ({}): \'{}\'", self.code, self.message),
                200..=299 => format!("I/O error ({}): \'{}\'", self.code, self.message),
                _ => format!("Record not found ({}): \'{}\'", self.code, self.message),
            };

            write!(f, "{}", err_msg)
        }
    }

    impl fmt::Debug for SaveFileError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "SaveFileError {{ code: {}, message: {} }}",
                self.code, self.message
            )
        }
    }

    impl Error for SaveFileError {}

    impl std::convert::From<std::io::Error> for SaveFileError {
        fn from(io_error: std::io::Error) -> Self {
            let error_code: i32 = 200 + io_error.raw_os_error().unwrap_or(0);
            return SaveFileError { code: error_code, message: io_error.to_string() };
        }
    }

    impl std::convert::From<std::num::TryFromIntError> for SaveFileError {
        fn from(from_error: TryFromIntError) -> Self {
            return SaveFileError { code: 101, message: from_error.to_string() };
        }
    }

    impl std::convert::From<minilzo::Error> for SaveFileError {
        fn from(minilzo_error: minilzo::Error) -> Self {
            return SaveFileError { code: 102, message: minilzo_error.to_string() };
        }
    }

    ///
    /// Loads a save file from the given path.
    ///
    /// The file from the given path is loaded and analyzed following a strict decompression format.
    /// The data is minlzo, huffman and then google protobuf encoded.
    ///
    /// # Arguments
    ///
    /// * `save_file_path` - The input path of the save file. The file is resolved from the cwd or
    /// the root path.
    ///
    /// # Returns
    ///
    /// * `Ok(data)` - Returns the decoded protobuf instance from the save file.
    /// * `Err(_)` - Iff the input file cannot be found, the decoding of the minilzo, huffmann or
    /// google protobuf compression fails or a checksum is mismatched the error is forwarded.
    ///
    pub fn load_save(save_file_path: std::string::String) -> Result<WillowTwoPlayerSaveGame, SaveFileError> {
        let buffer_data: Vec<u8> = load_verify_bytes_from_file(save_file_path)?;

        let lzo_decompressed_data: Vec<u8> = decompress_lzo(buffer_data)?;

        let huffman_decompressed_data: Vec<u8> = decompress_huffman(lzo_decompressed_data)?;

        match protobuf::parse_from_bytes::<WillowTwoPlayerSaveGame>(huffman_decompressed_data.as_ref()) {
            Ok(result) => return Ok(result),
            Err(err) => return Err(SaveFileError { code: 104, message: err.to_string() })
        };
    }

    ///
    /// Function to decompress a encoded huffman tree followed by data encoded
    /// with the huffman tree.
    ///
    /// First some metadata is decoded from the first 19 bytes of the input bytes.
    /// Then the huffman tree is decoded and then used to decoded the following data.
    ///
    /// # Arguments
    /// * `bytes` -  The input bytes. The first 19 bytes are metadata, the following should be the
    /// huffman tree followed by the huffman encoded data.
    ///
    /// # Returns
    ///
    /// * `Ok(data)` - The huffman decoded data from the input array.
    /// * `Err(_)` - Iff an error occurred this is returned.
    fn decompress_huffman(bytes: Vec<u8>) -> Result<Vec<u8>, SaveFileError> {
        let mut inner_size_bytes = [0; 4];
        inner_size_bytes.clone_from_slice(&bytes[..4]);

        let mut magic_number_bytes = [0; 3];
        magic_number_bytes.clone_from_slice(&bytes[4..7]);

        let mut version_bytes = [0; 4];
        version_bytes.clone_from_slice(&bytes[7..11]);

        let mut hash_bytes = [0; 4];
        hash_bytes.clone_from_slice(&bytes[11..15]);

        let mut inner_uncompressed_size_bytes = [0; 4];
        inner_uncompressed_size_bytes.clone_from_slice(&bytes[15..19]);

        let magic_bytes: [u8; 3] = [87, 83, 71];

        unsafe {

            let _inner_size: u64 =
                std::mem::transmute::<[u8; 4], u32>(inner_size_bytes).to_be() as u64;

            if magic_number_bytes.ne(&magic_bytes) {
                return Err(SaveFileError {
                    code: 105,
                    message: "Invalid magic bytes!".to_string(),
                });
            }

            let _version: u64 =
                std::mem::transmute::<[u8; 4], u32>(version_bytes).to_le() as u64;

            let _hash: u64 =
                std::mem::transmute::<[u8; 4], u32>(hash_bytes).to_le() as u64;

            let inner_uncompressed_size: usize =
                std::mem::transmute::<[u8; 4], i32>(inner_uncompressed_size_bytes).to_le()
                    as usize;

            return Ok(decode(&bytes[19..], inner_uncompressed_size));
        }
    }

    ///
    /// Loads all bytes int the file at *input_path_string*.
    /// Additionally the first 20 bytes of the file are interpreted as a SHA1 checksum.
    /// If this checksum does not match the checksum of the rest of the file, the function throws
    /// an error.
    ///
    /// # Arguments
    ///
    /// * `input_path_string` - Input path of the file to load & verify.
    ///
    /// # Returns
    ///
    /// * `Ok(data)` - Returns the data in the file withput the first 20 byte checksum.
    /// * `Err(_)` - Error if the file cannot be loaded or the checksum does not match.
    ///
    fn load_verify_bytes_from_file(input_path_string: String) -> Result<Vec<u8>, SaveFileError> {
        let input_path = Path::new(&input_path_string);
        let canonical_input_path = input_path.canonicalize()?;

        let mut file = File::open(&canonical_input_path)?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        let buffer_checksum = &buffer[..20];
        let buffer_data = &buffer[20..];

        let mut hasher = Sha1::new();
        hasher.input(&buffer_data);
        let result_checksum = hasher.result();

        if result_checksum.as_slice().ne(&buffer_checksum[..]) {
            return Err(SaveFileError {
                code: 100,
                message: "Buffer checksums do not match".to_string(),
            });
        }

        return Ok(Vec::from(buffer_data));
    }

    ///
    /// Function to decompress the data with the minilzo algorithm.
    ///
    /// The first 4 bytes of the input buffer are interpreted as a unsigned 32-bit
    /// big-endian integer encoding the uncompressed size of the data.
    /// The result array will have exactly this capacity.
    ///
    /// # Arguments
    ///
    /// * `bytes` - Input bytes. The first 4 bytes encode the length of the uncompressed data.
    ///
    /// # Results
    ///
    /// * `Ok(data)` - The decoded data.
    /// * `Err(_)` - Iff an error occurs during the decoding.
    ///
    fn decompress_lzo(bytes: Vec<u8>) -> Result<Vec<u8>, SaveFileError> {
        let mut uncompressed_size_bytes = [0; 4];
        uncompressed_size_bytes.clone_from_slice(&bytes[..4]);

        let compressed_data = &bytes[4..];

        unsafe {
            let uncompressed_size_int =
                std::mem::transmute::<[u8; 4], u32>(uncompressed_size_bytes).to_be() as u64;

            let uncompressed_size = usize::try_from(uncompressed_size_int)?;
            return Ok(minilzo::decompress(&compressed_data[..], uncompressed_size)?);
        }
    }

    #[cfg(test)]
    mod tests {
        //!
        //! Module to test loading and saving savefile instances for borderlands2.
        //!

        #[test]
        ///
        /// Test case to load and validate a save file instance.
        ///
        fn load_save_test() {
            let valid_path = "../resources/Save0001.sav".to_string();
            let save_file_res = super::load_save(valid_path);
            match &save_file_res {
                Ok(_) => assert!(true, "Input file should be processed!"),
                Err(err) => assert!(false, "Input file is not invalid: \'{}\'", err)
            };
        }

        #[test]
        ///
        /// Test case to check if invalid path are correctly detected.
        ///
        fn invalid_load_save_test() {
            let save_file_res = super::load_save("../resources/Save0001.save".to_string());
            match &save_file_res {
                Ok(_) => assert!(false, "Input file does not exist!"),
                Err(_) => assert!(true, "Input file is inavlid!")
            };
            let save_file_err = save_file_res.unwrap_err();
            assert_eq!(save_file_err.code, 202, "Invalid error code!");
        }
    }
}




