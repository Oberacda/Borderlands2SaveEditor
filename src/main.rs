mod protos;

use protos::WillowTwoPlayerSaveGame::{WillowTwoPlayerSaveGame};
use protobuf::json;

fn main() {
    let save_game = WillowTwoPlayerSaveGame::new();
    let string = protobuf::json::print_to_string(&save_game);
    println!("{}", string.unwrap());
    println!("Hello, world!");
}
