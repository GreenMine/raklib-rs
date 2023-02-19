use raklib::protocol::{packets::connected::Ack, types::u24};

#[test]
fn test_ack_generation() {
    let mut input_vec: Vec<u24> = vec![
        0u32, 1u32, 2u32, 3u32, 6u32, 4u32, 7u32, 8u32, 10u32, 12u32, 13u32,
    ]
    .into_iter()
    .map(|v| u24::from(v))
    .collect();

    let _ = Ack::from_packets(&mut input_vec);
}
