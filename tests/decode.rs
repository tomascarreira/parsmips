use parsmips::{Decode, Immediate, MipsI};

#[test]
fn api_works() {
    assert_eq!(
        0x3c080013.decode(),
        Ok(MipsI::Lui(Immediate {
            rs: 0,
            rt: 8,
            immediate: 0x13
        }))
    );
}
