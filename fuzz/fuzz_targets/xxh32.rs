#![no_main]
use libfuzzer_sys::fuzz_target;
use xxhash_rust::xxh32;

fuzz_target!(|input: (&[u8], u32)| {
    xxh32::xxh32(input.0, input.1);
});