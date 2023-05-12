#![no_main]
use libfuzzer_sys::fuzz_target;
use xxhash_rust::xxh3;

fuzz_target!(|input: &[u8]| {
    xxh3::xxh3_64(input);
});