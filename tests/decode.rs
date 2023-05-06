use parsmips::Decode;

#[test]
fn api_works() {
    let word: u32 = 0x3c080013;

    let _instr = word.decode();
}
