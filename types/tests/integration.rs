use amq_protocol_types::{
    *,
    generation::gen_value,
    parsing::parse_value,
};

#[test]
fn test_full_integration() {
    let mut table  = FieldTable::default();
    let mut table2 = FieldTable::default();

    table2.0.insert(ShortString("foo".to_string()), AMQPValue::DecimalValue(DecimalValue { scale: 55, value: 999, }));
    table2.0.insert(ShortString("baz".to_string()), AMQPValue::LongString(LongString("blah42".to_string())));

    table.0.insert(ShortString("foo".to_string()),  AMQPValue::FieldArray(FieldArray(vec![AMQPValue::Boolean(true), AMQPValue::Void])));
    table.0.insert(ShortString("oof".to_string()),  AMQPValue::FieldArray(FieldArray::default()));
    table.0.insert(ShortString("bar".to_string()),  AMQPValue::FieldTable(FieldTable::default()));
    table.0.insert(ShortString("blah".to_string()), AMQPValue::Boolean(false));
    table.0.insert(ShortString("aaaa".to_string()), AMQPValue::ShortShortInt(42));
    table.0.insert(ShortString("bbbb".to_string()), AMQPValue::ShortShortUInt(64));
    table.0.insert(ShortString("cccc".to_string()), AMQPValue::ShortInt(32));
    table.0.insert(ShortString("dddd".to_string()), AMQPValue::ShortUInt(132));
    table.0.insert(ShortString("eeee".to_string()), AMQPValue::LongInt(-53));
    table.0.insert(ShortString("ffff".to_string()), AMQPValue::LongUInt(66666));
    table.0.insert(ShortString("gggg".to_string()), AMQPValue::LongLongInt(-9999));
    table.0.insert(ShortString("hhhh".to_string()), AMQPValue::ByteArray(ByteArray(vec![42, 1, 2, 3])));
    table.0.insert(ShortString("iiii".to_string()), AMQPValue::Float(42.3));
    table.0.insert(ShortString("tabl".to_string()), AMQPValue::FieldTable(table2));
    table.0.insert(ShortString("jjjj".to_string()), AMQPValue::Double(0.00987654321));
    table.0.insert(ShortString("kkkk".to_string()), AMQPValue::Timestamp(1234567890));
    table.0.insert(ShortString("llll".to_string()), AMQPValue::Void);

    let value              = AMQPValue::FieldTable(table);
    let mut buf: [u8; 199] = [0; 199];

    gen_value(&mut buf[..], &value).unwrap();

    assert_eq!(parse_value(&buf), Ok((&[][..], value)));
}
