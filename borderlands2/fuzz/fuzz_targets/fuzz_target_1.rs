#![no_main]

use libfuzzer_sys::fuzz_target;

use borderlands2::load_save_mem;

fuzz_target!(|data: &[u8]| {
    load_save_mem(data.to_vec());
});
