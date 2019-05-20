use amq_protocol_types::{
    *,
    generation::gen_value,
    parsing::parse_value,
};

#[test]
fn test_full_integration() {
    let mut table  = FieldTable::default();
    let mut table2 = FieldTable::default();

    table2.0.insert("foo".into(), AMQPValue::DecimalValue(DecimalValue { scale: 55, value: 999, }));
    table2.0.insert("baz".into(), AMQPValue::LongString("blah42".into()));

    table.0.insert("foo".into(),  AMQPValue::FieldArray(FieldArray(vec![AMQPValue::Boolean(true), AMQPValue::Void])));
    table.0.insert("oof".into(),  AMQPValue::FieldArray(FieldArray::default()));
    table.0.insert("bar".into(),  AMQPValue::FieldTable(FieldTable::default()));
    table.0.insert("blah".into(), AMQPValue::Boolean(false));
    table.0.insert("aaaa".into(), AMQPValue::ShortShortInt(42));
    table.0.insert("bbbb".into(), AMQPValue::ShortShortUInt(64));
    table.0.insert("cccc".into(), AMQPValue::ShortInt(32));
    table.0.insert("dddd".into(), AMQPValue::ShortUInt(132));
    table.0.insert("eeee".into(), AMQPValue::LongInt(-53));
    table.0.insert("ffff".into(), AMQPValue::LongUInt(66666));
    table.0.insert("gggg".into(), AMQPValue::LongLongInt(-9999));
    table.0.insert("hhhh".into(), AMQPValue::ByteArray(ByteArray(vec![42, 1, 2, 3])));
    table.0.insert("iiii".into(), AMQPValue::Float(42.3));
    table.0.insert("tabl".into(), AMQPValue::FieldTable(table2));
    table.0.insert("jjjj".into(), AMQPValue::Double(0.00987654321));
    table.0.insert("kkkk".into(), AMQPValue::Timestamp(1234567890));
    table.0.insert("llll".into(), AMQPValue::Void);

    let value              = AMQPValue::FieldTable(table);
    let mut buf: [u8; 199] = [0; 199];

    gen_value(&mut buf[..], &value).unwrap();

    assert_eq!(parse_value(&buf), Ok((&[][..], value)));
}
