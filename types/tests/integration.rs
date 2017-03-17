extern crate amq_protocol_types;

use amq_protocol_types::*;

#[test]
fn test_full_integration() {
    let mut table          = FieldTable::new();
    let value              = AMQPValue::FieldTable(table);
    let mut buf: [u8; 512] = [0; 512];
    assert_eq!(parse_value(gen_value((&mut buf[..], 0), &value).unwrap().0).to_result().unwrap(), value);
}
