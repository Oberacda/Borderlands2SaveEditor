#![no_main]

use libfuzzer_sys::fuzz_target;

use borderlands2::handle_uncompressed_data;

fuzz_target!(|data: &[u8]| {
    unsafe {
        handle_uncompressed_data(data.to_vec());
    }
});
