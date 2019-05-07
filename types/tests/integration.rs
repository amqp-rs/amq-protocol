use amq_protocol_types::{
    *,
    generation::gen_value,
    parsing::parse_value,
};

#[test]
fn test_full_integration() {
    let mut table  = FieldTable::new();
    let mut table2 = FieldTable::new();

    table2.insert("foo".to_string(), AMQPValue::DecimalValue(DecimalValue { scale: 55, value: 999, }));
    table2.insert("baz".to_string(), AMQPValue::LongString("blah42".to_string()));

    table.insert("foo".to_string(),  AMQPValue::FieldArray(vec![AMQPValue::Boolean(true), AMQPValue::Void]));
    table.insert("oof".to_string(),  AMQPValue::FieldArray(FieldArray::new()));
    table.insert("bar".to_string(),  AMQPValue::FieldTable(FieldTable::new()));
    table.insert("blah".to_string(), AMQPValue::Boolean(false));
    table.insert("aaaa".to_string(), AMQPValue::ShortShortInt(42));
    table.insert("bbbb".to_string(), AMQPValue::ShortShortUInt(64));
    table.insert("cccc".to_string(), AMQPValue::ShortInt(32));
    table.insert("dddd".to_string(), AMQPValue::ShortUInt(132));
    table.insert("eeee".to_string(), AMQPValue::LongInt(-53));
    table.insert("ffff".to_string(), AMQPValue::LongUInt(66666));
    table.insert("gggg".to_string(), AMQPValue::LongLongInt(-9999));
    table.insert("hhhh".to_string(), AMQPValue::ByteArray(vec![42, 1, 2, 3]));
    table.insert("iiii".to_string(), AMQPValue::Float(42.3));
    table.insert("tabl".to_string(), AMQPValue::FieldTable(table2));
    table.insert("jjjj".to_string(), AMQPValue::Double(0.00987654321));
    table.insert("kkkk".to_string(), AMQPValue::Timestamp(1234567890));
    table.insert("llll".to_string(), AMQPValue::Void);

    let value              = AMQPValue::FieldTable(table);
    let mut buf: [u8; 199] = [0; 199];

    assert_eq!(parse_value(gen_value((&mut buf[..], 0), &value).unwrap().0), Ok((&[][..], value)));
}
