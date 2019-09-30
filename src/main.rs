mod protos;

use protos::WillowTwoPlayerSaveGame::{WillowTwoPlayerSaveGame};
use protobuf::json;

fn main() {
    let save_game = WillowTwoPlayerSaveGame::new();
    let string = protobuf::json::print_to_string(&save_game);
    println!("{}", string.unwrap());
    println!("Hello, world!");

    let data = b"foobar_foobar";

    let compressed = minilzo::compress(&data[..]);
    if (compressed.is_ok()) {
        let compressed_vector = compressed.unwrap();
        println!("{}", String::from_utf8(compressed_vector.clone()).unwrap());

        let decompressed = minilzo::decompress(&compressed_vector, data.len()).unwrap(); 
        println!("{}", String::from_utf8(decompressed).unwrap());
    }
}
