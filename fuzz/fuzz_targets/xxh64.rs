#![no_main]
use libfuzzer_sys::fuzz_target;
use xxhash_rust::xxh64;

fuzz_target!(|input: (&[u8], u64)| {
    xxh64::xxh64(input.0, input.1);
});