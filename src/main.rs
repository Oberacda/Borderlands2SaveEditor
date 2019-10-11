extern crate borderlands2;

fn main() {
    let save_file_path = "./resources/Save0001.sav".to_string();
    let _save_game = borderlands2::load_save(save_file_path);
}
