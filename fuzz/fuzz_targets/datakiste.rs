#![no_main]
use libfuzzer_sys::fuzz_target;
use datakiste::io::Datakiste;

fuzz_target!(|data: &[u8]| {
    let _: Result<Datakiste, _> = bincode::deserialize(data);
});
