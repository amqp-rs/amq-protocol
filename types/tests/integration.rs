use amq_protocol_types::{generation::gen_value, parsing::parse_value, *};

use cookie_factory::gen;

#[test]
fn test_full_integration() {
    let mut table = FieldTable::default();
    let mut table2 = FieldTable::default();

    table2.insert(
        "foo".into(),
        AMQPValue::DecimalValue(DecimalValue {
            scale: 55,
            value: 999,
        }),
    );
    table2.insert("baz".into(), AMQPValue::LongString("blah42".into()));

    table.insert(
        "foo".into(),
        AMQPValue::FieldArray(vec![AMQPValue::Boolean(true), AMQPValue::Void].into()),
    );
    table.insert("oof".into(), AMQPValue::FieldArray(FieldArray::default()));
    table.insert("bar".into(), AMQPValue::FieldTable(FieldTable::default()));
    table.insert("blah".into(), AMQPValue::Boolean(false));
    table.insert("aaaa".into(), AMQPValue::ShortShortInt(42));
    table.insert("bbbb".into(), AMQPValue::ShortShortUInt(64));
    table.insert("cccc".into(), AMQPValue::ShortInt(32));
    table.insert("dddd".into(), AMQPValue::ShortUInt(132));
    table.insert("eeee".into(), AMQPValue::LongInt(-53));
    table.insert("ffff".into(), AMQPValue::LongUInt(66666));
    table.insert("gggg".into(), AMQPValue::LongLongInt(-9999));
    table.insert(
        "hhhh".into(),
        AMQPValue::ByteArray(vec![42u8, 1u8, 2u8, 3u8].into()),
    );
    table.insert("iiii".into(), AMQPValue::Float(42.3));
    table.insert("tabl".into(), AMQPValue::FieldTable(table2));
    table.insert("jjjj".into(), AMQPValue::Double(0.00987654321));
    table.insert("kkkk".into(), AMQPValue::Timestamp(1234567890));
    table.insert("llll".into(), AMQPValue::Void);

    let value = AMQPValue::FieldTable(table);
    let mut buf: [u8; 199] = [0; 199];

    gen(gen_value(&value), &mut buf[..]).unwrap();

    assert_eq!(parse_value(&buf[..]), Ok((&[][..], value)));
}
