use raklib::protocol::{packets::connected::Ack, types::u24};
use raklib_std::utils::BinaryStream;

#[test]
fn test_convert() {
    let mut bs = BinaryStream::with_len(3);
    bs.add(u24::from(0x141312));
    bs.p = 0;

    let read_u24 = bs.read::<u24>();
    assert_eq!(0x141312, u32::from(read_u24));
}
