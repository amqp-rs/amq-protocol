extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

/* Fields with a custom rename are for deserializing
 * the secifications json file in amq-protocol-codegen
 */
#[derive(Debug, Deserialize, Serialize)]
pub enum AMQPType {
    #[serde(rename="bit")]
    Boolean,
    #[serde(rename="octet")]
    ShortShortInt,
    ShortShortUInt,
    #[serde(rename="short")]
    ShortInt,
    ShortUInt,
    #[serde(rename="long")]
    LongInt,
    LongUInt,
    #[serde(rename="longlong")]
    LongLongInt,
    LongLongUInt,
    Float,
    Double,
    DecimalValue,
    #[serde(rename="shortstr")]
    ShortString,
    #[serde(rename="longstr")]
    LongString,
    FieldArray,
    #[serde(rename="timestamp")]
    TimeStamp,
    #[serde(rename="table")]
    FieldTable,
    Void,
}
