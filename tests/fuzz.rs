use datakiste::io::Datakiste;

#[test]
fn fuzz_case_1() {
    let data = &[
        201u8, 196, 181, 172, 42, 100, 161, 226, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 3, 100, 226, 161, 0, 201, 86, 38,
    ] as &[u8];
    let mut config = bincode::config();
    config.limit(32);
    let _: Result<Datakiste, _> = config.deserialize(data);
}
