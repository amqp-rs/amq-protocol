/// Protocol metadata
pub mod metadata {
    use super::*;

    /// The name of the protocol
    pub const NAME: &str = "AMQP";
    /// The major version of the protocol
    pub const MAJOR_VERSION: ShortShortUInt = 0;
    /// The minor version of the protocol
    pub const MINOR_VERSION: ShortShortUInt = 9;
    /// The revision (version) of the protocol
    pub const REVISION: ShortShortUInt = 1;
    /// The default port of the protocol
    pub const PORT: LongUInt = 5672;
    /// The copyright holding the protocol
    pub const COPYRIGHT: &str = r#"Copyright (C) 2007-2024 Broadcom Inc. and its subsidiaries. All rights reserved.

Permission is hereby granted, free of charge, to any person
obtaining a copy of this file (the "Software"), to deal in the
Software without restriction, including without limitation the 
rights to use, copy, modify, merge, publish, distribute, 
sublicense, and/or sell copies of the Software, and to permit 
persons to whom the Software is furnished to do so, subject to 
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

Class information entered from amqp_xml0-8.pdf and domain types from amqp-xml-doc0-9.pdf
Updated for 0-9-1 by Tony Garnock-Jones

b3cb053f15e7b98808c0ccc67f23cb3e  amqp_xml0-8.pdf
http://twiststandards.org/?option=com_docman&task=cat_view&gid=28&Itemid=90
8444db91e2949dbecfb2585e9eef6d64  amqp-xml-doc0-9.pdf
https://jira.amqp.org/confluence/download/attachments/720900/amqp-xml-doc0-9.pdf?version=1
"#;
}

/// Protocol constants
pub mod constants {
    use super::*;

    /// FRAME-METHOD (Generated)
    pub const FRAME_METHOD: ShortShortUInt = 1;
    /// FRAME-HEADER (Generated)
    pub const FRAME_HEADER: ShortShortUInt = 2;
    /// FRAME-BODY (Generated)
    pub const FRAME_BODY: ShortShortUInt = 3;
    /// FRAME-HEARTBEAT (Generated)
    pub const FRAME_HEARTBEAT: ShortShortUInt = 8;
    /// FRAME-MIN-SIZE (Generated)
    pub const FRAME_MIN_SIZE: LongUInt = 4096;
    /// FRAME-END (Generated)
    pub const FRAME_END: ShortShortUInt = 206;
    /// REPLY-SUCCESS (Generated)
    pub const REPLY_SUCCESS: ShortUInt = 200;
}

/// The available soft AMQP errors
#[derive(Clone, Debug, PartialEq)]
pub enum AMQPSoftError {
    /// CONTENT-TOO-LARGE (Generated)
    CONTENTTOOLARGE,
    /// NO-ROUTE (Generated)
    NOROUTE,
    /// NO-CONSUMERS (Generated)
    NOCONSUMERS,
    /// ACCESS-REFUSED (Generated)
    ACCESSREFUSED,
    /// NOT-FOUND (Generated)
    NOTFOUND,
    /// RESOURCE-LOCKED (Generated)
    RESOURCELOCKED,
    /// PRECONDITION-FAILED (Generated)
    PRECONDITIONFAILED,
}

impl AMQPSoftError {
    /// Get the id of the soft error
    pub fn get_id(&self) -> Identifier {
        match *self {
            AMQPSoftError::CONTENTTOOLARGE => 311,
            AMQPSoftError::NOROUTE => 312,
            AMQPSoftError::NOCONSUMERS => 313,
            AMQPSoftError::ACCESSREFUSED => 403,
            AMQPSoftError::NOTFOUND => 404,
            AMQPSoftError::RESOURCELOCKED => 405,
            AMQPSoftError::PRECONDITIONFAILED => 406,
        }
    }

    /// Get the soft error corresponding to an id
    pub fn from_id(id: Identifier) -> Option<AMQPSoftError> {
        match id {
            311 => Some(AMQPSoftError::CONTENTTOOLARGE),
            312 => Some(AMQPSoftError::NOROUTE),
            313 => Some(AMQPSoftError::NOCONSUMERS),
            403 => Some(AMQPSoftError::ACCESSREFUSED),
            404 => Some(AMQPSoftError::NOTFOUND),
            405 => Some(AMQPSoftError::RESOURCELOCKED),
            406 => Some(AMQPSoftError::PRECONDITIONFAILED),
            _ => None,
        }
    }
}

impl fmt::Display for AMQPSoftError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AMQPSoftError::CONTENTTOOLARGE => write!(f, "CONTENT-TOO-LARGE"),
            AMQPSoftError::NOROUTE => write!(f, "NO-ROUTE"),
            AMQPSoftError::NOCONSUMERS => write!(f, "NO-CONSUMERS"),
            AMQPSoftError::ACCESSREFUSED => write!(f, "ACCESS-REFUSED"),
            AMQPSoftError::NOTFOUND => write!(f, "NOT-FOUND"),
            AMQPSoftError::RESOURCELOCKED => write!(f, "RESOURCE-LOCKED"),
            AMQPSoftError::PRECONDITIONFAILED => write!(f, "PRECONDITION-FAILED"),
        }
    }
}

/// The available hard AMQP errors
#[derive(Clone, Debug, PartialEq)]
pub enum AMQPHardError {
    /// CONNECTION-FORCED (Generated)
    CONNECTIONFORCED,
    /// INVALID-PATH (Generated)
    INVALIDPATH,
    /// FRAME-ERROR (Generated)
    FRAMEERROR,
    /// SYNTAX-ERROR (Generated)
    SYNTAXERROR,
    /// COMMAND-INVALID (Generated)
    COMMANDINVALID,
    /// CHANNEL-ERROR (Generated)
    CHANNELERROR,
    /// UNEXPECTED-FRAME (Generated)
    UNEXPECTEDFRAME,
    /// RESOURCE-ERROR (Generated)
    RESOURCEERROR,
    /// NOT-ALLOWED (Generated)
    NOTALLOWED,
    /// NOT-IMPLEMENTED (Generated)
    NOTIMPLEMENTED,
    /// INTERNAL-ERROR (Generated)
    INTERNALERROR,
}

impl AMQPHardError {
    /// Get the id of the hard error
    pub fn get_id(&self) -> Identifier {
        match *self {
            AMQPHardError::CONNECTIONFORCED => 320,
            AMQPHardError::INVALIDPATH => 402,
            AMQPHardError::FRAMEERROR => 501,
            AMQPHardError::SYNTAXERROR => 502,
            AMQPHardError::COMMANDINVALID => 503,
            AMQPHardError::CHANNELERROR => 504,
            AMQPHardError::UNEXPECTEDFRAME => 505,
            AMQPHardError::RESOURCEERROR => 506,
            AMQPHardError::NOTALLOWED => 530,
            AMQPHardError::NOTIMPLEMENTED => 540,
            AMQPHardError::INTERNALERROR => 541,
        }
    }

    /// Get the hard error corresponding to an id
    pub fn from_id(id: Identifier) -> Option<AMQPHardError> {
        match id {
            320 => Some(AMQPHardError::CONNECTIONFORCED),
            402 => Some(AMQPHardError::INVALIDPATH),
            501 => Some(AMQPHardError::FRAMEERROR),
            502 => Some(AMQPHardError::SYNTAXERROR),
            503 => Some(AMQPHardError::COMMANDINVALID),
            504 => Some(AMQPHardError::CHANNELERROR),
            505 => Some(AMQPHardError::UNEXPECTEDFRAME),
            506 => Some(AMQPHardError::RESOURCEERROR),
            530 => Some(AMQPHardError::NOTALLOWED),
            540 => Some(AMQPHardError::NOTIMPLEMENTED),
            541 => Some(AMQPHardError::INTERNALERROR),
            _ => None,
        }
    }
}

impl fmt::Display for AMQPHardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AMQPHardError::CONNECTIONFORCED => write!(f, "CONNECTION-FORCED"),
            AMQPHardError::INVALIDPATH => write!(f, "INVALID-PATH"),
            AMQPHardError::FRAMEERROR => write!(f, "FRAME-ERROR"),
            AMQPHardError::SYNTAXERROR => write!(f, "SYNTAX-ERROR"),
            AMQPHardError::COMMANDINVALID => write!(f, "COMMAND-INVALID"),
            AMQPHardError::CHANNELERROR => write!(f, "CHANNEL-ERROR"),
            AMQPHardError::UNEXPECTEDFRAME => write!(f, "UNEXPECTED-FRAME"),
            AMQPHardError::RESOURCEERROR => write!(f, "RESOURCE-ERROR"),
            AMQPHardError::NOTALLOWED => write!(f, "NOT-ALLOWED"),
            AMQPHardError::NOTIMPLEMENTED => write!(f, "NOT-IMPLEMENTED"),
            AMQPHardError::INTERNALERROR => write!(f, "INTERNAL-ERROR"),
        }
    }
}

use self::access::parse_access;
use self::basic::parse_basic;
use self::channel::parse_channel;
use self::confirm::parse_confirm;
use self::connection::parse_connection;
use self::exchange::parse_exchange;
use self::queue::parse_queue;
use self::tx::parse_tx;
/// Parse an AMQP class
pub fn parse_class<I: ParsableInput>(i: I) -> ParserResult<I, AMQPClass> {
    context(
        "parse_class",
        map_opt(
            flat_map(parse_id, |id| {
                move |i| match id {
                    60 => map(map(parse_basic, AMQPClass::Basic), Some)(i),
                    10 => map(map(parse_connection, AMQPClass::Connection), Some)(i),
                    20 => map(map(parse_channel, AMQPClass::Channel), Some)(i),
                    30 => map(map(parse_access, AMQPClass::Access), Some)(i),
                    40 => map(map(parse_exchange, AMQPClass::Exchange), Some)(i),
                    50 => map(map(parse_queue, AMQPClass::Queue), Some)(i),
                    90 => map(map(parse_tx, AMQPClass::Tx), Some)(i),
                    85 => map(map(parse_confirm, AMQPClass::Confirm), Some)(i),
                    _ => Ok((i, None)),
                }
            }),
            std::convert::identity,
        ),
    )(i)
}

/// Serialize an AMQP class
pub fn gen_class<'a, W: Write + BackToTheBuffer + 'a>(
    class: &'a AMQPClass,
) -> impl SerializeFn<W> + 'a {
    move |input| match *class {
        AMQPClass::Basic(ref basic) => basic::gen_basic(basic)(input),
        AMQPClass::Connection(ref connection) => connection::gen_connection(connection)(input),
        AMQPClass::Channel(ref channel) => channel::gen_channel(channel)(input),
        AMQPClass::Access(ref access) => access::gen_access(access)(input),
        AMQPClass::Exchange(ref exchange) => exchange::gen_exchange(exchange)(input),
        AMQPClass::Queue(ref queue) => queue::gen_queue(queue)(input),
        AMQPClass::Tx(ref tx) => tx::gen_tx(tx)(input),
        AMQPClass::Confirm(ref confirm) => confirm::gen_confirm(confirm)(input),
    }
}

/// The available AMQP classes
#[derive(Clone, Debug, PartialEq)]
pub enum AMQPClass {
    /// basic (Generated)
    Basic(basic::AMQPMethod),
    /// connection (Generated)
    Connection(connection::AMQPMethod),
    /// channel (Generated)
    Channel(channel::AMQPMethod),
    /// access (Generated)
    Access(access::AMQPMethod),
    /// exchange (Generated)
    Exchange(exchange::AMQPMethod),
    /// queue (Generated)
    Queue(queue::AMQPMethod),
    /// tx (Generated)
    Tx(tx::AMQPMethod),
    /// confirm (Generated)
    Confirm(confirm::AMQPMethod),
}

impl AMQPClass {
    /// Get the AMQP class id (Generated)
    pub fn get_amqp_class_id(&self) -> Identifier {
        match self {
            AMQPClass::Basic(_) => 60,
            AMQPClass::Connection(_) => 10,
            AMQPClass::Channel(_) => 20,
            AMQPClass::Access(_) => 30,
            AMQPClass::Exchange(_) => 40,
            AMQPClass::Queue(_) => 50,
            AMQPClass::Tx(_) => 90,
            AMQPClass::Confirm(_) => 85,
        }
    }

    /// Get the AMQP method id (Generated)
    pub fn get_amqp_method_id(&self) -> Identifier {
        match self {
            AMQPClass::Basic(basic::AMQPMethod::Qos(_)) => 10,
            AMQPClass::Basic(basic::AMQPMethod::QosOk(_)) => 11,
            AMQPClass::Basic(basic::AMQPMethod::Consume(_)) => 20,
            AMQPClass::Basic(basic::AMQPMethod::ConsumeOk(_)) => 21,
            AMQPClass::Basic(basic::AMQPMethod::Cancel(_)) => 30,
            AMQPClass::Basic(basic::AMQPMethod::CancelOk(_)) => 31,
            AMQPClass::Basic(basic::AMQPMethod::Publish(_)) => 40,
            AMQPClass::Basic(basic::AMQPMethod::Return(_)) => 50,
            AMQPClass::Basic(basic::AMQPMethod::Deliver(_)) => 60,
            AMQPClass::Basic(basic::AMQPMethod::Get(_)) => 70,
            AMQPClass::Basic(basic::AMQPMethod::GetOk(_)) => 71,
            AMQPClass::Basic(basic::AMQPMethod::GetEmpty(_)) => 72,
            AMQPClass::Basic(basic::AMQPMethod::Ack(_)) => 80,
            AMQPClass::Basic(basic::AMQPMethod::Reject(_)) => 90,
            AMQPClass::Basic(basic::AMQPMethod::RecoverAsync(_)) => 100,
            AMQPClass::Basic(basic::AMQPMethod::Recover(_)) => 110,
            AMQPClass::Basic(basic::AMQPMethod::RecoverOk(_)) => 111,
            AMQPClass::Basic(basic::AMQPMethod::Nack(_)) => 120,
            AMQPClass::Connection(connection::AMQPMethod::Start(_)) => 10,
            AMQPClass::Connection(connection::AMQPMethod::StartOk(_)) => 11,
            AMQPClass::Connection(connection::AMQPMethod::Secure(_)) => 20,
            AMQPClass::Connection(connection::AMQPMethod::SecureOk(_)) => 21,
            AMQPClass::Connection(connection::AMQPMethod::Tune(_)) => 30,
            AMQPClass::Connection(connection::AMQPMethod::TuneOk(_)) => 31,
            AMQPClass::Connection(connection::AMQPMethod::Open(_)) => 40,
            AMQPClass::Connection(connection::AMQPMethod::OpenOk(_)) => 41,
            AMQPClass::Connection(connection::AMQPMethod::Close(_)) => 50,
            AMQPClass::Connection(connection::AMQPMethod::CloseOk(_)) => 51,
            AMQPClass::Connection(connection::AMQPMethod::Blocked(_)) => 60,
            AMQPClass::Connection(connection::AMQPMethod::Unblocked(_)) => 61,
            AMQPClass::Connection(connection::AMQPMethod::UpdateSecret(_)) => 70,
            AMQPClass::Connection(connection::AMQPMethod::UpdateSecretOk(_)) => 71,
            AMQPClass::Channel(channel::AMQPMethod::Open(_)) => 10,
            AMQPClass::Channel(channel::AMQPMethod::OpenOk(_)) => 11,
            AMQPClass::Channel(channel::AMQPMethod::Flow(_)) => 20,
            AMQPClass::Channel(channel::AMQPMethod::FlowOk(_)) => 21,
            AMQPClass::Channel(channel::AMQPMethod::Close(_)) => 40,
            AMQPClass::Channel(channel::AMQPMethod::CloseOk(_)) => 41,
            AMQPClass::Access(access::AMQPMethod::Request(_)) => 10,
            AMQPClass::Access(access::AMQPMethod::RequestOk(_)) => 11,
            AMQPClass::Exchange(exchange::AMQPMethod::Declare(_)) => 10,
            AMQPClass::Exchange(exchange::AMQPMethod::DeclareOk(_)) => 11,
            AMQPClass::Exchange(exchange::AMQPMethod::Delete(_)) => 20,
            AMQPClass::Exchange(exchange::AMQPMethod::DeleteOk(_)) => 21,
            AMQPClass::Exchange(exchange::AMQPMethod::Bind(_)) => 30,
            AMQPClass::Exchange(exchange::AMQPMethod::BindOk(_)) => 31,
            AMQPClass::Exchange(exchange::AMQPMethod::Unbind(_)) => 40,
            AMQPClass::Exchange(exchange::AMQPMethod::UnbindOk(_)) => 51,
            AMQPClass::Queue(queue::AMQPMethod::Declare(_)) => 10,
            AMQPClass::Queue(queue::AMQPMethod::DeclareOk(_)) => 11,
            AMQPClass::Queue(queue::AMQPMethod::Bind(_)) => 20,
            AMQPClass::Queue(queue::AMQPMethod::BindOk(_)) => 21,
            AMQPClass::Queue(queue::AMQPMethod::Purge(_)) => 30,
            AMQPClass::Queue(queue::AMQPMethod::PurgeOk(_)) => 31,
            AMQPClass::Queue(queue::AMQPMethod::Delete(_)) => 40,
            AMQPClass::Queue(queue::AMQPMethod::DeleteOk(_)) => 41,
            AMQPClass::Queue(queue::AMQPMethod::Unbind(_)) => 50,
            AMQPClass::Queue(queue::AMQPMethod::UnbindOk(_)) => 51,
            AMQPClass::Tx(tx::AMQPMethod::Select(_)) => 10,
            AMQPClass::Tx(tx::AMQPMethod::SelectOk(_)) => 11,
            AMQPClass::Tx(tx::AMQPMethod::Commit(_)) => 20,
            AMQPClass::Tx(tx::AMQPMethod::CommitOk(_)) => 21,
            AMQPClass::Tx(tx::AMQPMethod::Rollback(_)) => 30,
            AMQPClass::Tx(tx::AMQPMethod::RollbackOk(_)) => 31,
            AMQPClass::Confirm(confirm::AMQPMethod::Select(_)) => 10,
            AMQPClass::Confirm(confirm::AMQPMethod::SelectOk(_)) => 11,
        }
    }
}

/// basic (generated)
pub mod basic {
    use super::*;

    /// Parse basic (Generated)
    pub fn parse_basic<I: ParsableInput>(i: I) -> ParserResult<I, basic::AMQPMethod> {
        context(
            "parse_basic",
            map_opt(
                flat_map(parse_id, |id| {
                    move |i| match id {
                        10 => context("parse_qos", map(map(parse_qos, AMQPMethod::Qos), Some))(i),
                        11 => context(
                            "parse_qos_ok",
                            map(map(parse_qos_ok, AMQPMethod::QosOk), Some),
                        )(i),
                        20 => context(
                            "parse_consume",
                            map(map(parse_consume, AMQPMethod::Consume), Some),
                        )(i),
                        21 => context(
                            "parse_consume_ok",
                            map(map(parse_consume_ok, AMQPMethod::ConsumeOk), Some),
                        )(i),
                        30 => context(
                            "parse_cancel",
                            map(map(parse_cancel, AMQPMethod::Cancel), Some),
                        )(i),
                        31 => context(
                            "parse_cancel_ok",
                            map(map(parse_cancel_ok, AMQPMethod::CancelOk), Some),
                        )(i),
                        40 => context(
                            "parse_publish",
                            map(map(parse_publish, AMQPMethod::Publish), Some),
                        )(i),
                        50 => context(
                            "parse_return",
                            map(map(parse_return, AMQPMethod::Return), Some),
                        )(i),
                        60 => context(
                            "parse_deliver",
                            map(map(parse_deliver, AMQPMethod::Deliver), Some),
                        )(i),
                        70 => context("parse_get", map(map(parse_get, AMQPMethod::Get), Some))(i),
                        71 => context(
                            "parse_get_ok",
                            map(map(parse_get_ok, AMQPMethod::GetOk), Some),
                        )(i),
                        72 => context(
                            "parse_get_empty",
                            map(map(parse_get_empty, AMQPMethod::GetEmpty), Some),
                        )(i),
                        80 => context("parse_ack", map(map(parse_ack, AMQPMethod::Ack), Some))(i),
                        90 => context(
                            "parse_reject",
                            map(map(parse_reject, AMQPMethod::Reject), Some),
                        )(i),
                        100 => context(
                            "parse_recover_async",
                            map(map(parse_recover_async, AMQPMethod::RecoverAsync), Some),
                        )(i),
                        110 => context(
                            "parse_recover",
                            map(map(parse_recover, AMQPMethod::Recover), Some),
                        )(i),
                        111 => context(
                            "parse_recover_ok",
                            map(map(parse_recover_ok, AMQPMethod::RecoverOk), Some),
                        )(i),
                        120 => {
                            context("parse_nack", map(map(parse_nack, AMQPMethod::Nack), Some))(i)
                        }
                        _ => Ok((i, None)),
                    }
                }),
                std::convert::identity,
            ),
        )(i)
    }

    /// Serialize basic (Generated)
    pub fn gen_basic<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a AMQPMethod,
    ) -> impl SerializeFn<W> + 'a {
        cookie_factory::sequence::pair(gen_id(60), move |input| match *method {
            AMQPMethod::Qos(ref qos) => gen_qos(qos)(input),
            AMQPMethod::QosOk(ref qos_ok) => gen_qos_ok(qos_ok)(input),
            AMQPMethod::Consume(ref consume) => gen_consume(consume)(input),
            AMQPMethod::ConsumeOk(ref consume_ok) => gen_consume_ok(consume_ok)(input),
            AMQPMethod::Cancel(ref cancel) => gen_cancel(cancel)(input),
            AMQPMethod::CancelOk(ref cancel_ok) => gen_cancel_ok(cancel_ok)(input),
            AMQPMethod::Publish(ref publish) => gen_publish(publish)(input),
            AMQPMethod::Return(ref r#return) => gen_return(r#return)(input),
            AMQPMethod::Deliver(ref deliver) => gen_deliver(deliver)(input),
            AMQPMethod::Get(ref get) => gen_get(get)(input),
            AMQPMethod::GetOk(ref get_ok) => gen_get_ok(get_ok)(input),
            AMQPMethod::GetEmpty(ref get_empty) => gen_get_empty(get_empty)(input),
            AMQPMethod::Ack(ref ack) => gen_ack(ack)(input),
            AMQPMethod::Reject(ref reject) => gen_reject(reject)(input),
            AMQPMethod::RecoverAsync(ref recover_async) => gen_recover_async(recover_async)(input),
            AMQPMethod::Recover(ref recover) => gen_recover(recover)(input),
            AMQPMethod::RecoverOk(ref recover_ok) => gen_recover_ok(recover_ok)(input),
            AMQPMethod::Nack(ref nack) => gen_nack(nack)(input),
        })
    }

    /// The available methods in basic
    #[derive(Clone, Debug, PartialEq)]
    pub enum AMQPMethod {
        /// qos (Generated)
        Qos(Qos),
        /// qos-ok (Generated)
        QosOk(QosOk),
        /// consume (Generated)
        Consume(Consume),
        /// consume-ok (Generated)
        ConsumeOk(ConsumeOk),
        /// cancel (Generated)
        Cancel(Cancel),
        /// cancel-ok (Generated)
        CancelOk(CancelOk),
        /// publish (Generated)
        Publish(Publish),
        /// return (Generated)
        Return(Return),
        /// deliver (Generated)
        Deliver(Deliver),
        /// get (Generated)
        Get(Get),
        /// get-ok (Generated)
        GetOk(GetOk),
        /// get-empty (Generated)
        GetEmpty(GetEmpty),
        /// ack (Generated)
        Ack(Ack),
        /// reject (Generated)
        Reject(Reject),
        /// recover-async (Generated)
        RecoverAsync(RecoverAsync),
        /// recover (Generated)
        Recover(Recover),
        /// recover-ok (Generated)
        RecoverOk(RecoverOk),
        /// nack (Generated)
        Nack(Nack),
    }

    /// qos (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Qos {
        /// prefetch-count (Generated)
        pub prefetch_count: ShortUInt,
        /// global (Generated)
        pub global: Boolean,
    }

    impl Qos {
        /// Get the AMQP class id for qos (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            60
        }

        /// Get the AMQP method id for qos (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            10
        }
    }

    /// Parse qos (Generated)
    pub fn parse_qos<I: ParsableInput>(i: I) -> ParserResult<I, Qos> {
        let (i, _) = parse_long_uint(i)?;
        let (i, prefetch_count) = parse_short_uint(i)?;
        let (i, flags) = parse_flags(i, &["global"])?;
        Ok((
            i,
            Qos {
                prefetch_count,
                global: flags.get_flag("global").unwrap_or(false),
            },
        ))
    }

    /// Serialize qos (Generated)
    pub fn gen_qos<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Qos,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("global".to_string(), method.global);
            input = gen_id(10)(input)?;
            input = gen_long_uint(0)(input)?;
            input = gen_short_uint(method.prefetch_count)(input)?;
            input = gen_flags(&flags)(input)?;
            Ok(input)
        }
    }
    /// qos-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct QosOk {}

    impl QosOk {
        /// Get the AMQP class id for qos-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            60
        }

        /// Get the AMQP method id for qos-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            11
        }
    }

    /// Parse qos-ok (Generated)
    pub fn parse_qos_ok<I: ParsableInput>(i: I) -> ParserResult<I, QosOk> {
        Ok((i, QosOk {}))
    }

    /// Serialize qos-ok (Generated)
    pub fn gen_qos_ok<'a, W: Write + BackToTheBuffer + 'a>(
        _: &'a QosOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(11)(input)?;
            Ok(input)
        }
    }
    /// consume (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Consume {
        /// queue (Generated)
        pub queue: ShortString,
        /// consumer-tag (Generated)
        pub consumer_tag: ShortString,
        /// no-local (Generated)
        pub no_local: Boolean,
        /// no-ack (Generated)
        pub no_ack: Boolean,
        /// exclusive (Generated)
        pub exclusive: Boolean,
        /// nowait (Generated)
        pub nowait: Boolean,
        /// arguments (Generated)
        pub arguments: FieldTable,
    }

    impl Consume {
        /// Get the AMQP class id for consume (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            60
        }

        /// Get the AMQP method id for consume (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            20
        }
    }

    /// Parse consume (Generated)
    pub fn parse_consume<I: ParsableInput>(i: I) -> ParserResult<I, Consume> {
        let (i, _) = parse_short_uint(i)?;
        let (i, queue) = parse_short_string(i)?;
        let (i, consumer_tag) = parse_short_string(i)?;
        let (i, flags) = parse_flags(i, &["no-local", "no-ack", "exclusive", "nowait"])?;
        let (i, arguments) = parse_field_table(i)?;
        Ok((
            i,
            Consume {
                queue,
                consumer_tag,
                no_local: flags.get_flag("no_local").unwrap_or(false),
                no_ack: flags.get_flag("no_ack").unwrap_or(false),
                exclusive: flags.get_flag("exclusive").unwrap_or(false),
                nowait: flags.get_flag("nowait").unwrap_or(false),
                arguments,
            },
        ))
    }

    /// Serialize consume (Generated)
    pub fn gen_consume<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Consume,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("no_local".to_string(), method.no_local);
            flags.add_flag("no_ack".to_string(), method.no_ack);
            flags.add_flag("exclusive".to_string(), method.exclusive);
            flags.add_flag("nowait".to_string(), method.nowait);
            input = gen_id(20)(input)?;
            input = gen_short_uint(0)(input)?;
            input = gen_short_string(method.queue.as_str())(input)?;
            input = gen_short_string(method.consumer_tag.as_str())(input)?;
            input = gen_flags(&flags)(input)?;
            input = gen_field_table(&method.arguments)(input)?;
            Ok(input)
        }
    }
    /// consume-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct ConsumeOk {
        /// consumer-tag (Generated)
        pub consumer_tag: ShortString,
    }

    impl ConsumeOk {
        /// Get the AMQP class id for consume-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            60
        }

        /// Get the AMQP method id for consume-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            21
        }
    }

    /// Parse consume-ok (Generated)
    pub fn parse_consume_ok<I: ParsableInput>(i: I) -> ParserResult<I, ConsumeOk> {
        let (i, consumer_tag) = parse_short_string(i)?;
        Ok((i, ConsumeOk { consumer_tag }))
    }

    /// Serialize consume-ok (Generated)
    pub fn gen_consume_ok<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a ConsumeOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(21)(input)?;
            input = gen_short_string(method.consumer_tag.as_str())(input)?;
            Ok(input)
        }
    }
    /// cancel (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Cancel {
        /// consumer-tag (Generated)
        pub consumer_tag: ShortString,
        /// nowait (Generated)
        pub nowait: Boolean,
    }

    impl Cancel {
        /// Get the AMQP class id for cancel (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            60
        }

        /// Get the AMQP method id for cancel (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            30
        }
    }

    /// Parse cancel (Generated)
    pub fn parse_cancel<I: ParsableInput>(i: I) -> ParserResult<I, Cancel> {
        let (i, consumer_tag) = parse_short_string(i)?;
        let (i, flags) = parse_flags(i, &["nowait"])?;
        Ok((
            i,
            Cancel {
                consumer_tag,
                nowait: flags.get_flag("nowait").unwrap_or(false),
            },
        ))
    }

    /// Serialize cancel (Generated)
    pub fn gen_cancel<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Cancel,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("nowait".to_string(), method.nowait);
            input = gen_id(30)(input)?;
            input = gen_short_string(method.consumer_tag.as_str())(input)?;
            input = gen_flags(&flags)(input)?;
            Ok(input)
        }
    }
    /// cancel-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct CancelOk {
        /// consumer-tag (Generated)
        pub consumer_tag: ShortString,
    }

    impl CancelOk {
        /// Get the AMQP class id for cancel-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            60
        }

        /// Get the AMQP method id for cancel-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            31
        }
    }

    /// Parse cancel-ok (Generated)
    pub fn parse_cancel_ok<I: ParsableInput>(i: I) -> ParserResult<I, CancelOk> {
        let (i, consumer_tag) = parse_short_string(i)?;
        Ok((i, CancelOk { consumer_tag }))
    }

    /// Serialize cancel-ok (Generated)
    pub fn gen_cancel_ok<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a CancelOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(31)(input)?;
            input = gen_short_string(method.consumer_tag.as_str())(input)?;
            Ok(input)
        }
    }
    /// publish (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Publish {
        /// exchange (Generated)
        pub exchange: ShortString,
        /// routing-key (Generated)
        pub routing_key: ShortString,
        /// mandatory (Generated)
        pub mandatory: Boolean,
        /// immediate (Generated)
        pub immediate: Boolean,
    }

    impl Publish {
        /// Get the AMQP class id for publish (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            60
        }

        /// Get the AMQP method id for publish (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            40
        }
    }

    /// Parse publish (Generated)
    pub fn parse_publish<I: ParsableInput>(i: I) -> ParserResult<I, Publish> {
        let (i, _) = parse_short_uint(i)?;
        let (i, exchange) = parse_short_string(i)?;
        let (i, routing_key) = parse_short_string(i)?;
        let (i, flags) = parse_flags(i, &["mandatory", "immediate"])?;
        Ok((
            i,
            Publish {
                exchange,
                routing_key,
                mandatory: flags.get_flag("mandatory").unwrap_or(false),
                immediate: flags.get_flag("immediate").unwrap_or(false),
            },
        ))
    }

    /// Serialize publish (Generated)
    pub fn gen_publish<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Publish,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("mandatory".to_string(), method.mandatory);
            flags.add_flag("immediate".to_string(), method.immediate);
            input = gen_id(40)(input)?;
            input = gen_short_uint(0)(input)?;
            input = gen_short_string(method.exchange.as_str())(input)?;
            input = gen_short_string(method.routing_key.as_str())(input)?;
            input = gen_flags(&flags)(input)?;
            Ok(input)
        }
    }
    /// return (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Return {
        /// reply-code (Generated)
        pub reply_code: ShortUInt,
        /// reply-text (Generated)
        pub reply_text: ShortString,
        /// exchange (Generated)
        pub exchange: ShortString,
        /// routing-key (Generated)
        pub routing_key: ShortString,
    }

    impl Return {
        /// Get the AMQP class id for return (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            60
        }

        /// Get the AMQP method id for return (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            50
        }
    }

    /// Parse return (Generated)
    pub fn parse_return<I: ParsableInput>(i: I) -> ParserResult<I, Return> {
        let (i, reply_code) = parse_short_uint(i)?;
        let (i, reply_text) = parse_short_string(i)?;
        let (i, exchange) = parse_short_string(i)?;
        let (i, routing_key) = parse_short_string(i)?;
        Ok((
            i,
            Return {
                reply_code,
                reply_text,
                exchange,
                routing_key,
            },
        ))
    }

    /// Serialize return (Generated)
    pub fn gen_return<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Return,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(50)(input)?;
            input = gen_short_uint(method.reply_code)(input)?;
            input = gen_short_string(method.reply_text.as_str())(input)?;
            input = gen_short_string(method.exchange.as_str())(input)?;
            input = gen_short_string(method.routing_key.as_str())(input)?;
            Ok(input)
        }
    }
    /// deliver (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Deliver {
        /// consumer-tag (Generated)
        pub consumer_tag: ShortString,
        /// delivery-tag (Generated)
        pub delivery_tag: LongLongUInt,
        /// redelivered (Generated)
        pub redelivered: Boolean,
        /// exchange (Generated)
        pub exchange: ShortString,
        /// routing-key (Generated)
        pub routing_key: ShortString,
    }

    impl Deliver {
        /// Get the AMQP class id for deliver (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            60
        }

        /// Get the AMQP method id for deliver (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            60
        }
    }

    /// Parse deliver (Generated)
    pub fn parse_deliver<I: ParsableInput>(i: I) -> ParserResult<I, Deliver> {
        let (i, consumer_tag) = parse_short_string(i)?;
        let (i, delivery_tag) = parse_long_long_uint(i)?;
        let (i, flags) = parse_flags(i, &["redelivered"])?;
        let (i, exchange) = parse_short_string(i)?;
        let (i, routing_key) = parse_short_string(i)?;
        Ok((
            i,
            Deliver {
                consumer_tag,
                delivery_tag,
                redelivered: flags.get_flag("redelivered").unwrap_or(false),
                exchange,
                routing_key,
            },
        ))
    }

    /// Serialize deliver (Generated)
    pub fn gen_deliver<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Deliver,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("redelivered".to_string(), method.redelivered);
            input = gen_id(60)(input)?;
            input = gen_short_string(method.consumer_tag.as_str())(input)?;
            input = gen_long_long_uint(method.delivery_tag)(input)?;
            input = gen_flags(&flags)(input)?;
            input = gen_short_string(method.exchange.as_str())(input)?;
            input = gen_short_string(method.routing_key.as_str())(input)?;
            Ok(input)
        }
    }
    /// get (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Get {
        /// queue (Generated)
        pub queue: ShortString,
        /// no-ack (Generated)
        pub no_ack: Boolean,
    }

    impl Get {
        /// Get the AMQP class id for get (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            60
        }

        /// Get the AMQP method id for get (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            70
        }
    }

    /// Parse get (Generated)
    pub fn parse_get<I: ParsableInput>(i: I) -> ParserResult<I, Get> {
        let (i, _) = parse_short_uint(i)?;
        let (i, queue) = parse_short_string(i)?;
        let (i, flags) = parse_flags(i, &["no-ack"])?;
        Ok((
            i,
            Get {
                queue,
                no_ack: flags.get_flag("no_ack").unwrap_or(false),
            },
        ))
    }

    /// Serialize get (Generated)
    pub fn gen_get<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Get,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("no_ack".to_string(), method.no_ack);
            input = gen_id(70)(input)?;
            input = gen_short_uint(0)(input)?;
            input = gen_short_string(method.queue.as_str())(input)?;
            input = gen_flags(&flags)(input)?;
            Ok(input)
        }
    }
    /// get-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GetOk {
        /// delivery-tag (Generated)
        pub delivery_tag: LongLongUInt,
        /// redelivered (Generated)
        pub redelivered: Boolean,
        /// exchange (Generated)
        pub exchange: ShortString,
        /// routing-key (Generated)
        pub routing_key: ShortString,
        /// message-count (Generated)
        pub message_count: LongUInt,
    }

    impl GetOk {
        /// Get the AMQP class id for get-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            60
        }

        /// Get the AMQP method id for get-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            71
        }
    }

    /// Parse get-ok (Generated)
    pub fn parse_get_ok<I: ParsableInput>(i: I) -> ParserResult<I, GetOk> {
        let (i, delivery_tag) = parse_long_long_uint(i)?;
        let (i, flags) = parse_flags(i, &["redelivered"])?;
        let (i, exchange) = parse_short_string(i)?;
        let (i, routing_key) = parse_short_string(i)?;
        let (i, message_count) = parse_long_uint(i)?;
        Ok((
            i,
            GetOk {
                delivery_tag,
                redelivered: flags.get_flag("redelivered").unwrap_or(false),
                exchange,
                routing_key,
                message_count,
            },
        ))
    }

    /// Serialize get-ok (Generated)
    pub fn gen_get_ok<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a GetOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("redelivered".to_string(), method.redelivered);
            input = gen_id(71)(input)?;
            input = gen_long_long_uint(method.delivery_tag)(input)?;
            input = gen_flags(&flags)(input)?;
            input = gen_short_string(method.exchange.as_str())(input)?;
            input = gen_short_string(method.routing_key.as_str())(input)?;
            input = gen_long_uint(method.message_count)(input)?;
            Ok(input)
        }
    }
    /// get-empty (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GetEmpty {}

    impl GetEmpty {
        /// Get the AMQP class id for get-empty (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            60
        }

        /// Get the AMQP method id for get-empty (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            72
        }
    }

    /// Parse get-empty (Generated)
    pub fn parse_get_empty<I: ParsableInput>(i: I) -> ParserResult<I, GetEmpty> {
        let (i, _) = parse_short_string(i)?;
        Ok((i, GetEmpty {}))
    }

    /// Serialize get-empty (Generated)
    pub fn gen_get_empty<'a, W: Write + BackToTheBuffer + 'a>(
        _method: &'a GetEmpty,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(72)(input)?;
            input = gen_short_string("")(input)?;
            Ok(input)
        }
    }
    /// ack (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Ack {
        /// delivery-tag (Generated)
        pub delivery_tag: LongLongUInt,
        /// multiple (Generated)
        pub multiple: Boolean,
    }

    impl Ack {
        /// Get the AMQP class id for ack (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            60
        }

        /// Get the AMQP method id for ack (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            80
        }
    }

    /// Parse ack (Generated)
    pub fn parse_ack<I: ParsableInput>(i: I) -> ParserResult<I, Ack> {
        let (i, delivery_tag) = parse_long_long_uint(i)?;
        let (i, flags) = parse_flags(i, &["multiple"])?;
        Ok((
            i,
            Ack {
                delivery_tag,
                multiple: flags.get_flag("multiple").unwrap_or(false),
            },
        ))
    }

    /// Serialize ack (Generated)
    pub fn gen_ack<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Ack,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("multiple".to_string(), method.multiple);
            input = gen_id(80)(input)?;
            input = gen_long_long_uint(method.delivery_tag)(input)?;
            input = gen_flags(&flags)(input)?;
            Ok(input)
        }
    }
    /// reject (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Reject {
        /// delivery-tag (Generated)
        pub delivery_tag: LongLongUInt,
        /// requeue (Generated)
        pub requeue: Boolean,
    }

    impl Reject {
        /// Get the AMQP class id for reject (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            60
        }

        /// Get the AMQP method id for reject (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            90
        }
    }

    /// Parse reject (Generated)
    pub fn parse_reject<I: ParsableInput>(i: I) -> ParserResult<I, Reject> {
        let (i, delivery_tag) = parse_long_long_uint(i)?;
        let (i, flags) = parse_flags(i, &["requeue"])?;
        Ok((
            i,
            Reject {
                delivery_tag,
                requeue: flags.get_flag("requeue").unwrap_or(false),
            },
        ))
    }

    /// Serialize reject (Generated)
    pub fn gen_reject<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Reject,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("requeue".to_string(), method.requeue);
            input = gen_id(90)(input)?;
            input = gen_long_long_uint(method.delivery_tag)(input)?;
            input = gen_flags(&flags)(input)?;
            Ok(input)
        }
    }
    /// recover-async (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct RecoverAsync {
        /// requeue (Generated)
        pub requeue: Boolean,
    }

    impl RecoverAsync {
        /// Get the AMQP class id for recover-async (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            60
        }

        /// Get the AMQP method id for recover-async (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            100
        }
    }

    /// Parse recover-async (Generated)
    pub fn parse_recover_async<I: ParsableInput>(i: I) -> ParserResult<I, RecoverAsync> {
        let (i, flags) = parse_flags(i, &["requeue"])?;
        Ok((
            i,
            RecoverAsync {
                requeue: flags.get_flag("requeue").unwrap_or(false),
            },
        ))
    }

    /// Serialize recover-async (Generated)
    pub fn gen_recover_async<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a RecoverAsync,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("requeue".to_string(), method.requeue);
            input = gen_id(100)(input)?;
            input = gen_flags(&flags)(input)?;
            Ok(input)
        }
    }
    /// recover (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Recover {
        /// requeue (Generated)
        pub requeue: Boolean,
    }

    impl Recover {
        /// Get the AMQP class id for recover (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            60
        }

        /// Get the AMQP method id for recover (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            110
        }
    }

    /// Parse recover (Generated)
    pub fn parse_recover<I: ParsableInput>(i: I) -> ParserResult<I, Recover> {
        let (i, flags) = parse_flags(i, &["requeue"])?;
        Ok((
            i,
            Recover {
                requeue: flags.get_flag("requeue").unwrap_or(false),
            },
        ))
    }

    /// Serialize recover (Generated)
    pub fn gen_recover<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Recover,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("requeue".to_string(), method.requeue);
            input = gen_id(110)(input)?;
            input = gen_flags(&flags)(input)?;
            Ok(input)
        }
    }
    /// recover-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct RecoverOk {}

    impl RecoverOk {
        /// Get the AMQP class id for recover-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            60
        }

        /// Get the AMQP method id for recover-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            111
        }
    }

    /// Parse recover-ok (Generated)
    pub fn parse_recover_ok<I: ParsableInput>(i: I) -> ParserResult<I, RecoverOk> {
        Ok((i, RecoverOk {}))
    }

    /// Serialize recover-ok (Generated)
    pub fn gen_recover_ok<'a, W: Write + BackToTheBuffer + 'a>(
        _: &'a RecoverOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(111)(input)?;
            Ok(input)
        }
    }
    /// nack (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Nack {
        /// delivery-tag (Generated)
        pub delivery_tag: LongLongUInt,
        /// multiple (Generated)
        pub multiple: Boolean,
        /// requeue (Generated)
        pub requeue: Boolean,
    }

    impl Nack {
        /// Get the AMQP class id for nack (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            60
        }

        /// Get the AMQP method id for nack (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            120
        }
    }

    /// Parse nack (Generated)
    pub fn parse_nack<I: ParsableInput>(i: I) -> ParserResult<I, Nack> {
        let (i, delivery_tag) = parse_long_long_uint(i)?;
        let (i, flags) = parse_flags(i, &["multiple", "requeue"])?;
        Ok((
            i,
            Nack {
                delivery_tag,
                multiple: flags.get_flag("multiple").unwrap_or(false),
                requeue: flags.get_flag("requeue").unwrap_or(false),
            },
        ))
    }

    /// Serialize nack (Generated)
    pub fn gen_nack<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Nack,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("multiple".to_string(), method.multiple);
            flags.add_flag("requeue".to_string(), method.requeue);
            input = gen_id(120)(input)?;
            input = gen_long_long_uint(method.delivery_tag)(input)?;
            input = gen_flags(&flags)(input)?;
            Ok(input)
        }
    }
    /// basic properties (Generated)
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct AMQPProperties {
        content_type: Option<ShortString>,
        content_encoding: Option<ShortString>,
        headers: Option<FieldTable>,
        delivery_mode: Option<ShortShortUInt>,
        priority: Option<ShortShortUInt>,
        correlation_id: Option<ShortString>,
        reply_to: Option<ShortString>,
        expiration: Option<ShortString>,
        message_id: Option<ShortString>,
        timestamp: Option<Timestamp>,
        kind: Option<ShortString>,
        user_id: Option<ShortString>,
        app_id: Option<ShortString>,
        cluster_id: Option<ShortString>,
    }

    impl AMQPProperties {
        /// Set content-type (Generated)
        pub fn with_content_type(mut self, value: ShortString) -> Self {
            self.content_type = Some(value);
            self
        }
        /// Set content-encoding (Generated)
        pub fn with_content_encoding(mut self, value: ShortString) -> Self {
            self.content_encoding = Some(value);
            self
        }
        /// Set headers (Generated)
        pub fn with_headers(mut self, value: FieldTable) -> Self {
            self.headers = Some(value);
            self
        }
        /// Set delivery-mode (Generated)
        pub fn with_delivery_mode(mut self, value: ShortShortUInt) -> Self {
            self.delivery_mode = Some(value);
            self
        }
        /// Set priority (Generated)
        pub fn with_priority(mut self, value: ShortShortUInt) -> Self {
            self.priority = Some(value);
            self
        }
        /// Set correlation-id (Generated)
        pub fn with_correlation_id(mut self, value: ShortString) -> Self {
            self.correlation_id = Some(value);
            self
        }
        /// Set reply-to (Generated)
        pub fn with_reply_to(mut self, value: ShortString) -> Self {
            self.reply_to = Some(value);
            self
        }
        /// Set expiration (Generated)
        pub fn with_expiration(mut self, value: ShortString) -> Self {
            self.expiration = Some(value);
            self
        }
        /// Set message-id (Generated)
        pub fn with_message_id(mut self, value: ShortString) -> Self {
            self.message_id = Some(value);
            self
        }
        /// Set timestamp (Generated)
        pub fn with_timestamp(mut self, value: Timestamp) -> Self {
            self.timestamp = Some(value);
            self
        }
        /// Set type (Generated)
        pub fn with_type(mut self, value: ShortString) -> Self {
            self.kind = Some(value);
            self
        }
        /// Set user-id (Generated)
        pub fn with_user_id(mut self, value: ShortString) -> Self {
            self.user_id = Some(value);
            self
        }
        /// Set app-id (Generated)
        pub fn with_app_id(mut self, value: ShortString) -> Self {
            self.app_id = Some(value);
            self
        }
        /// Set cluster-id (Generated)
        pub fn with_cluster_id(mut self, value: ShortString) -> Self {
            self.cluster_id = Some(value);
            self
        }
        /// Get content-type (Generated)
        pub fn content_type(&self) -> &Option<ShortString> {
            &self.content_type
        }
        /// Get content-encoding (Generated)
        pub fn content_encoding(&self) -> &Option<ShortString> {
            &self.content_encoding
        }
        /// Get headers (Generated)
        pub fn headers(&self) -> &Option<FieldTable> {
            &self.headers
        }
        /// Get delivery-mode (Generated)
        pub fn delivery_mode(&self) -> &Option<ShortShortUInt> {
            &self.delivery_mode
        }
        /// Get priority (Generated)
        pub fn priority(&self) -> &Option<ShortShortUInt> {
            &self.priority
        }
        /// Get correlation-id (Generated)
        pub fn correlation_id(&self) -> &Option<ShortString> {
            &self.correlation_id
        }
        /// Get reply-to (Generated)
        pub fn reply_to(&self) -> &Option<ShortString> {
            &self.reply_to
        }
        /// Get expiration (Generated)
        pub fn expiration(&self) -> &Option<ShortString> {
            &self.expiration
        }
        /// Get message-id (Generated)
        pub fn message_id(&self) -> &Option<ShortString> {
            &self.message_id
        }
        /// Get timestamp (Generated)
        pub fn timestamp(&self) -> &Option<Timestamp> {
            &self.timestamp
        }
        /// Get type (Generated)
        pub fn kind(&self) -> &Option<ShortString> {
            &self.kind
        }
        /// Get user-id (Generated)
        pub fn user_id(&self) -> &Option<ShortString> {
            &self.user_id
        }
        /// Get app-id (Generated)
        pub fn app_id(&self) -> &Option<ShortString> {
            &self.app_id
        }
        /// Get cluster-id (Generated)
        pub fn cluster_id(&self) -> &Option<ShortString> {
            &self.cluster_id
        }
        /// Get the bitmask for serialization (Generated)
        #[allow(clippy::identity_op)]
        pub fn bitmask(&self) -> ShortUInt {
            (if self.content_type.is_some() {
                1 << (15 - 0)
            } else {
                0
            }) + (if self.content_encoding.is_some() {
                1 << (15 - 1)
            } else {
                0
            }) + (if self.headers.is_some() {
                1 << (15 - 2)
            } else {
                0
            }) + (if self.delivery_mode.is_some() {
                1 << (15 - 3)
            } else {
                0
            }) + (if self.priority.is_some() {
                1 << (15 - 4)
            } else {
                0
            }) + (if self.correlation_id.is_some() {
                1 << (15 - 5)
            } else {
                0
            }) + (if self.reply_to.is_some() {
                1 << (15 - 6)
            } else {
                0
            }) + (if self.expiration.is_some() {
                1 << (15 - 7)
            } else {
                0
            }) + (if self.message_id.is_some() {
                1 << (15 - 8)
            } else {
                0
            }) + (if self.timestamp.is_some() {
                1 << (15 - 9)
            } else {
                0
            }) + (if self.kind.is_some() {
                1 << (15 - 10)
            } else {
                0
            }) + (if self.user_id.is_some() {
                1 << (15 - 11)
            } else {
                0
            }) + (if self.app_id.is_some() {
                1 << (15 - 12)
            } else {
                0
            }) + (if self.cluster_id.is_some() {
                1 << (15 - 13)
            } else {
                0
            })
        }
    }

    /// Parse basic properties (Generated)
    #[allow(clippy::identity_op)]
    pub fn parse_properties<I: ParsableInput>(i: I) -> ParserResult<I, AMQPProperties> {
        let (i, flags) = parse_short_uint(i)?;
        let (i, content_type) = if flags & (1 << (15 - 0)) != 0 {
            map(parse_short_string, Some)(i)?
        } else {
            (i, None)
        };
        let (i, content_encoding) = if flags & (1 << (15 - 1)) != 0 {
            map(parse_short_string, Some)(i)?
        } else {
            (i, None)
        };
        let (i, headers) = if flags & (1 << (15 - 2)) != 0 {
            map(parse_field_table, Some)(i)?
        } else {
            (i, None)
        };
        let (i, delivery_mode) = if flags & (1 << (15 - 3)) != 0 {
            map(parse_short_short_uint, Some)(i)?
        } else {
            (i, None)
        };
        let (i, priority) = if flags & (1 << (15 - 4)) != 0 {
            map(parse_short_short_uint, Some)(i)?
        } else {
            (i, None)
        };
        let (i, correlation_id) = if flags & (1 << (15 - 5)) != 0 {
            map(parse_short_string, Some)(i)?
        } else {
            (i, None)
        };
        let (i, reply_to) = if flags & (1 << (15 - 6)) != 0 {
            map(parse_short_string, Some)(i)?
        } else {
            (i, None)
        };
        let (i, expiration) = if flags & (1 << (15 - 7)) != 0 {
            map(parse_short_string, Some)(i)?
        } else {
            (i, None)
        };
        let (i, message_id) = if flags & (1 << (15 - 8)) != 0 {
            map(parse_short_string, Some)(i)?
        } else {
            (i, None)
        };
        let (i, timestamp) = if flags & (1 << (15 - 9)) != 0 {
            map(parse_timestamp, Some)(i)?
        } else {
            (i, None)
        };
        let (i, kind) = if flags & (1 << (15 - 10)) != 0 {
            map(parse_short_string, Some)(i)?
        } else {
            (i, None)
        };
        let (i, user_id) = if flags & (1 << (15 - 11)) != 0 {
            map(parse_short_string, Some)(i)?
        } else {
            (i, None)
        };
        let (i, app_id) = if flags & (1 << (15 - 12)) != 0 {
            map(parse_short_string, Some)(i)?
        } else {
            (i, None)
        };
        let (i, cluster_id) = if flags & (1 << (15 - 13)) != 0 {
            map(parse_short_string, Some)(i)?
        } else {
            (i, None)
        };
        Ok((
            i,
            AMQPProperties {
                content_type,
                content_encoding,
                headers,
                delivery_mode,
                priority,
                correlation_id,
                reply_to,
                expiration,
                message_id,
                timestamp,
                kind,
                user_id,
                app_id,
                cluster_id,
            },
        ))
    }

    /// Serialize basic properties (Generated)
    pub fn gen_properties<'a, W: Write + BackToTheBuffer + 'a>(
        props: &'a AMQPProperties,
    ) -> impl SerializeFn<W> + 'a {
        cookie_factory::sequence::pair(gen_short_uint(props.bitmask()), move |mut input| {
            if let Some(prop) = props.content_type.as_ref() {
                input = gen_short_string(prop.as_str())(input)?;
            }
            if let Some(prop) = props.content_encoding.as_ref() {
                input = gen_short_string(prop.as_str())(input)?;
            }
            if let Some(prop) = props.headers.as_ref() {
                input = gen_field_table(prop)(input)?;
            }
            if let Some(prop) = props.delivery_mode {
                input = gen_short_short_uint(prop)(input)?;
            }
            if let Some(prop) = props.priority {
                input = gen_short_short_uint(prop)(input)?;
            }
            if let Some(prop) = props.correlation_id.as_ref() {
                input = gen_short_string(prop.as_str())(input)?;
            }
            if let Some(prop) = props.reply_to.as_ref() {
                input = gen_short_string(prop.as_str())(input)?;
            }
            if let Some(prop) = props.expiration.as_ref() {
                input = gen_short_string(prop.as_str())(input)?;
            }
            if let Some(prop) = props.message_id.as_ref() {
                input = gen_short_string(prop.as_str())(input)?;
            }
            if let Some(prop) = props.timestamp {
                input = gen_timestamp(prop)(input)?;
            }
            if let Some(prop) = props.kind.as_ref() {
                input = gen_short_string(prop.as_str())(input)?;
            }
            if let Some(prop) = props.user_id.as_ref() {
                input = gen_short_string(prop.as_str())(input)?;
            }
            if let Some(prop) = props.app_id.as_ref() {
                input = gen_short_string(prop.as_str())(input)?;
            }
            if let Some(prop) = props.cluster_id.as_ref() {
                input = gen_short_string(prop.as_str())(input)?;
            }
            Ok(input)
        })
    }
}
/// connection (generated)
pub mod connection {
    use super::*;

    /// Parse connection (Generated)
    pub fn parse_connection<I: ParsableInput>(i: I) -> ParserResult<I, connection::AMQPMethod> {
        context(
            "parse_connection",
            map_opt(
                flat_map(parse_id, |id| {
                    move |i| match id {
                        10 => context(
                            "parse_start",
                            map(map(parse_start, AMQPMethod::Start), Some),
                        )(i),
                        11 => context(
                            "parse_start_ok",
                            map(map(parse_start_ok, AMQPMethod::StartOk), Some),
                        )(i),
                        20 => context(
                            "parse_secure",
                            map(map(parse_secure, AMQPMethod::Secure), Some),
                        )(i),
                        21 => context(
                            "parse_secure_ok",
                            map(map(parse_secure_ok, AMQPMethod::SecureOk), Some),
                        )(i),
                        30 => {
                            context("parse_tune", map(map(parse_tune, AMQPMethod::Tune), Some))(i)
                        }
                        31 => context(
                            "parse_tune_ok",
                            map(map(parse_tune_ok, AMQPMethod::TuneOk), Some),
                        )(i),
                        40 => {
                            context("parse_open", map(map(parse_open, AMQPMethod::Open), Some))(i)
                        }
                        41 => context(
                            "parse_open_ok",
                            map(map(parse_open_ok, AMQPMethod::OpenOk), Some),
                        )(i),
                        50 => context(
                            "parse_close",
                            map(map(parse_close, AMQPMethod::Close), Some),
                        )(i),
                        51 => context(
                            "parse_close_ok",
                            map(map(parse_close_ok, AMQPMethod::CloseOk), Some),
                        )(i),
                        60 => context(
                            "parse_blocked",
                            map(map(parse_blocked, AMQPMethod::Blocked), Some),
                        )(i),
                        61 => context(
                            "parse_unblocked",
                            map(map(parse_unblocked, AMQPMethod::Unblocked), Some),
                        )(i),
                        70 => context(
                            "parse_update_secret",
                            map(map(parse_update_secret, AMQPMethod::UpdateSecret), Some),
                        )(i),
                        71 => context(
                            "parse_update_secret_ok",
                            map(
                                map(parse_update_secret_ok, AMQPMethod::UpdateSecretOk),
                                Some,
                            ),
                        )(i),
                        _ => Ok((i, None)),
                    }
                }),
                std::convert::identity,
            ),
        )(i)
    }

    /// Serialize connection (Generated)
    pub fn gen_connection<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a AMQPMethod,
    ) -> impl SerializeFn<W> + 'a {
        cookie_factory::sequence::pair(gen_id(10), move |input| match *method {
            AMQPMethod::Start(ref start) => gen_start(start)(input),
            AMQPMethod::StartOk(ref start_ok) => gen_start_ok(start_ok)(input),
            AMQPMethod::Secure(ref secure) => gen_secure(secure)(input),
            AMQPMethod::SecureOk(ref secure_ok) => gen_secure_ok(secure_ok)(input),
            AMQPMethod::Tune(ref tune) => gen_tune(tune)(input),
            AMQPMethod::TuneOk(ref tune_ok) => gen_tune_ok(tune_ok)(input),
            AMQPMethod::Open(ref open) => gen_open(open)(input),
            AMQPMethod::OpenOk(ref open_ok) => gen_open_ok(open_ok)(input),
            AMQPMethod::Close(ref close) => gen_close(close)(input),
            AMQPMethod::CloseOk(ref close_ok) => gen_close_ok(close_ok)(input),
            AMQPMethod::Blocked(ref blocked) => gen_blocked(blocked)(input),
            AMQPMethod::Unblocked(ref unblocked) => gen_unblocked(unblocked)(input),
            AMQPMethod::UpdateSecret(ref update_secret) => gen_update_secret(update_secret)(input),
            AMQPMethod::UpdateSecretOk(ref update_secret_ok) => {
                gen_update_secret_ok(update_secret_ok)(input)
            }
        })
    }

    /// The available methods in connection
    #[derive(Clone, Debug, PartialEq)]
    pub enum AMQPMethod {
        /// start (Generated)
        Start(Start),
        /// start-ok (Generated)
        StartOk(StartOk),
        /// secure (Generated)
        Secure(Secure),
        /// secure-ok (Generated)
        SecureOk(SecureOk),
        /// tune (Generated)
        Tune(Tune),
        /// tune-ok (Generated)
        TuneOk(TuneOk),
        /// open (Generated)
        Open(Open),
        /// open-ok (Generated)
        OpenOk(OpenOk),
        /// close (Generated)
        Close(Close),
        /// close-ok (Generated)
        CloseOk(CloseOk),
        /// blocked (Generated)
        Blocked(Blocked),
        /// unblocked (Generated)
        Unblocked(Unblocked),
        /// update-secret (Generated)
        UpdateSecret(UpdateSecret),
        /// update-secret-ok (Generated)
        UpdateSecretOk(UpdateSecretOk),
    }

    /// start (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Start {
        /// version-major (Generated)
        pub version_major: ShortShortUInt,
        /// version-minor (Generated)
        pub version_minor: ShortShortUInt,
        /// server-properties (Generated)
        pub server_properties: FieldTable,
        /// mechanisms (Generated)
        pub mechanisms: LongString,
        /// locales (Generated)
        pub locales: LongString,
    }

    impl Start {
        /// Get the AMQP class id for start (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            10
        }

        /// Get the AMQP method id for start (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            10
        }
    }

    /// Parse start (Generated)
    pub fn parse_start<I: ParsableInput>(i: I) -> ParserResult<I, Start> {
        let (i, version_major) = parse_short_short_uint(i)?;
        let (i, version_minor) = parse_short_short_uint(i)?;
        let (i, server_properties) = parse_field_table(i)?;
        let (i, mechanisms) = parse_long_string(i)?;
        let (i, locales) = parse_long_string(i)?;
        Ok((
            i,
            Start {
                version_major,
                version_minor,
                server_properties,
                mechanisms,
                locales,
            },
        ))
    }

    /// Serialize start (Generated)
    pub fn gen_start<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Start,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(10)(input)?;
            input = gen_short_short_uint(method.version_major)(input)?;
            input = gen_short_short_uint(method.version_minor)(input)?;
            input = gen_field_table(&method.server_properties)(input)?;
            input = gen_long_string(method.mechanisms.as_bytes())(input)?;
            input = gen_long_string(method.locales.as_bytes())(input)?;
            Ok(input)
        }
    }
    /// start-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct StartOk {
        /// client-properties (Generated)
        pub client_properties: FieldTable,
        /// mechanism (Generated)
        pub mechanism: ShortString,
        /// response (Generated)
        pub response: LongString,
        /// locale (Generated)
        pub locale: ShortString,
    }

    impl StartOk {
        /// Get the AMQP class id for start-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            10
        }

        /// Get the AMQP method id for start-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            11
        }
    }

    /// Parse start-ok (Generated)
    pub fn parse_start_ok<I: ParsableInput>(i: I) -> ParserResult<I, StartOk> {
        let (i, client_properties) = parse_field_table(i)?;
        let (i, mechanism) = parse_short_string(i)?;
        let (i, response) = parse_long_string(i)?;
        let (i, locale) = parse_short_string(i)?;
        Ok((
            i,
            StartOk {
                client_properties,
                mechanism,
                response,
                locale,
            },
        ))
    }

    /// Serialize start-ok (Generated)
    pub fn gen_start_ok<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a StartOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(11)(input)?;
            input = gen_field_table(&method.client_properties)(input)?;
            input = gen_short_string(method.mechanism.as_str())(input)?;
            input = gen_long_string(method.response.as_bytes())(input)?;
            input = gen_short_string(method.locale.as_str())(input)?;
            Ok(input)
        }
    }
    /// secure (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Secure {
        /// challenge (Generated)
        pub challenge: LongString,
    }

    impl Secure {
        /// Get the AMQP class id for secure (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            10
        }

        /// Get the AMQP method id for secure (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            20
        }
    }

    /// Parse secure (Generated)
    pub fn parse_secure<I: ParsableInput>(i: I) -> ParserResult<I, Secure> {
        let (i, challenge) = parse_long_string(i)?;
        Ok((i, Secure { challenge }))
    }

    /// Serialize secure (Generated)
    pub fn gen_secure<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Secure,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(20)(input)?;
            input = gen_long_string(method.challenge.as_bytes())(input)?;
            Ok(input)
        }
    }
    /// secure-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct SecureOk {
        /// response (Generated)
        pub response: LongString,
    }

    impl SecureOk {
        /// Get the AMQP class id for secure-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            10
        }

        /// Get the AMQP method id for secure-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            21
        }
    }

    /// Parse secure-ok (Generated)
    pub fn parse_secure_ok<I: ParsableInput>(i: I) -> ParserResult<I, SecureOk> {
        let (i, response) = parse_long_string(i)?;
        Ok((i, SecureOk { response }))
    }

    /// Serialize secure-ok (Generated)
    pub fn gen_secure_ok<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a SecureOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(21)(input)?;
            input = gen_long_string(method.response.as_bytes())(input)?;
            Ok(input)
        }
    }
    /// tune (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Tune {
        /// channel-max (Generated)
        pub channel_max: ShortUInt,
        /// frame-max (Generated)
        pub frame_max: LongUInt,
        /// heartbeat (Generated)
        pub heartbeat: ShortUInt,
    }

    impl Tune {
        /// Get the AMQP class id for tune (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            10
        }

        /// Get the AMQP method id for tune (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            30
        }
    }

    /// Parse tune (Generated)
    pub fn parse_tune<I: ParsableInput>(i: I) -> ParserResult<I, Tune> {
        let (i, channel_max) = parse_short_uint(i)?;
        let (i, frame_max) = parse_long_uint(i)?;
        let (i, heartbeat) = parse_short_uint(i)?;
        Ok((
            i,
            Tune {
                channel_max,
                frame_max,
                heartbeat,
            },
        ))
    }

    /// Serialize tune (Generated)
    pub fn gen_tune<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Tune,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(30)(input)?;
            input = gen_short_uint(method.channel_max)(input)?;
            input = gen_long_uint(method.frame_max)(input)?;
            input = gen_short_uint(method.heartbeat)(input)?;
            Ok(input)
        }
    }
    /// tune-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct TuneOk {
        /// channel-max (Generated)
        pub channel_max: ShortUInt,
        /// frame-max (Generated)
        pub frame_max: LongUInt,
        /// heartbeat (Generated)
        pub heartbeat: ShortUInt,
    }

    impl TuneOk {
        /// Get the AMQP class id for tune-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            10
        }

        /// Get the AMQP method id for tune-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            31
        }
    }

    /// Parse tune-ok (Generated)
    pub fn parse_tune_ok<I: ParsableInput>(i: I) -> ParserResult<I, TuneOk> {
        let (i, channel_max) = parse_short_uint(i)?;
        let (i, frame_max) = parse_long_uint(i)?;
        let (i, heartbeat) = parse_short_uint(i)?;
        Ok((
            i,
            TuneOk {
                channel_max,
                frame_max,
                heartbeat,
            },
        ))
    }

    /// Serialize tune-ok (Generated)
    pub fn gen_tune_ok<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a TuneOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(31)(input)?;
            input = gen_short_uint(method.channel_max)(input)?;
            input = gen_long_uint(method.frame_max)(input)?;
            input = gen_short_uint(method.heartbeat)(input)?;
            Ok(input)
        }
    }
    /// open (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Open {
        /// virtual-host (Generated)
        pub virtual_host: ShortString,
    }

    impl Open {
        /// Get the AMQP class id for open (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            10
        }

        /// Get the AMQP method id for open (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            40
        }
    }

    /// Parse open (Generated)
    pub fn parse_open<I: ParsableInput>(i: I) -> ParserResult<I, Open> {
        let (i, virtual_host) = parse_short_string(i)?;
        let (i, _) = parse_short_string(i)?;
        let (i, _) = parse_flags(i, &["insist"])?;
        Ok((i, Open { virtual_host }))
    }

    /// Serialize open (Generated)
    pub fn gen_open<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Open,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("insist".to_string(), false);
            input = gen_id(40)(input)?;
            input = gen_short_string(method.virtual_host.as_str())(input)?;
            input = gen_short_string("")(input)?;
            input = gen_flags(&flags)(input)?;
            Ok(input)
        }
    }
    /// open-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct OpenOk {}

    impl OpenOk {
        /// Get the AMQP class id for open-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            10
        }

        /// Get the AMQP method id for open-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            41
        }
    }

    /// Parse open-ok (Generated)
    pub fn parse_open_ok<I: ParsableInput>(i: I) -> ParserResult<I, OpenOk> {
        let (i, _) = parse_short_string(i)?;
        Ok((i, OpenOk {}))
    }

    /// Serialize open-ok (Generated)
    pub fn gen_open_ok<'a, W: Write + BackToTheBuffer + 'a>(
        _method: &'a OpenOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(41)(input)?;
            input = gen_short_string("")(input)?;
            Ok(input)
        }
    }
    /// close (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Close {
        /// reply-code (Generated)
        pub reply_code: ShortUInt,
        /// reply-text (Generated)
        pub reply_text: ShortString,
        /// class-id (Generated)
        pub class_id: ShortUInt,
        /// method-id (Generated)
        pub method_id: ShortUInt,
    }

    impl Close {
        /// Get the AMQP class id for close (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            10
        }

        /// Get the AMQP method id for close (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            50
        }
    }

    /// Parse close (Generated)
    pub fn parse_close<I: ParsableInput>(i: I) -> ParserResult<I, Close> {
        let (i, reply_code) = parse_short_uint(i)?;
        let (i, reply_text) = parse_short_string(i)?;
        let (i, class_id) = parse_short_uint(i)?;
        let (i, method_id) = parse_short_uint(i)?;
        Ok((
            i,
            Close {
                reply_code,
                reply_text,
                class_id,
                method_id,
            },
        ))
    }

    /// Serialize close (Generated)
    pub fn gen_close<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Close,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(50)(input)?;
            input = gen_short_uint(method.reply_code)(input)?;
            input = gen_short_string(method.reply_text.as_str())(input)?;
            input = gen_short_uint(method.class_id)(input)?;
            input = gen_short_uint(method.method_id)(input)?;
            Ok(input)
        }
    }
    /// close-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct CloseOk {}

    impl CloseOk {
        /// Get the AMQP class id for close-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            10
        }

        /// Get the AMQP method id for close-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            51
        }
    }

    /// Parse close-ok (Generated)
    pub fn parse_close_ok<I: ParsableInput>(i: I) -> ParserResult<I, CloseOk> {
        Ok((i, CloseOk {}))
    }

    /// Serialize close-ok (Generated)
    pub fn gen_close_ok<'a, W: Write + BackToTheBuffer + 'a>(
        _: &'a CloseOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(51)(input)?;
            Ok(input)
        }
    }
    /// blocked (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Blocked {
        /// reason (Generated)
        pub reason: ShortString,
    }

    impl Blocked {
        /// Get the AMQP class id for blocked (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            10
        }

        /// Get the AMQP method id for blocked (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            60
        }
    }

    /// Parse blocked (Generated)
    pub fn parse_blocked<I: ParsableInput>(i: I) -> ParserResult<I, Blocked> {
        let (i, reason) = parse_short_string(i)?;
        Ok((i, Blocked { reason }))
    }

    /// Serialize blocked (Generated)
    pub fn gen_blocked<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Blocked,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(60)(input)?;
            input = gen_short_string(method.reason.as_str())(input)?;
            Ok(input)
        }
    }
    /// unblocked (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Unblocked {}

    impl Unblocked {
        /// Get the AMQP class id for unblocked (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            10
        }

        /// Get the AMQP method id for unblocked (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            61
        }
    }

    /// Parse unblocked (Generated)
    pub fn parse_unblocked<I: ParsableInput>(i: I) -> ParserResult<I, Unblocked> {
        Ok((i, Unblocked {}))
    }

    /// Serialize unblocked (Generated)
    pub fn gen_unblocked<'a, W: Write + BackToTheBuffer + 'a>(
        _: &'a Unblocked,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(61)(input)?;
            Ok(input)
        }
    }
    /// update-secret (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct UpdateSecret {
        /// new-secret (Generated)
        pub new_secret: LongString,
        /// reason (Generated)
        pub reason: ShortString,
    }

    impl UpdateSecret {
        /// Get the AMQP class id for update-secret (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            10
        }

        /// Get the AMQP method id for update-secret (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            70
        }
    }

    /// Parse update-secret (Generated)
    pub fn parse_update_secret<I: ParsableInput>(i: I) -> ParserResult<I, UpdateSecret> {
        let (i, new_secret) = parse_long_string(i)?;
        let (i, reason) = parse_short_string(i)?;
        Ok((i, UpdateSecret { new_secret, reason }))
    }

    /// Serialize update-secret (Generated)
    pub fn gen_update_secret<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a UpdateSecret,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(70)(input)?;
            input = gen_long_string(method.new_secret.as_bytes())(input)?;
            input = gen_short_string(method.reason.as_str())(input)?;
            Ok(input)
        }
    }
    /// update-secret-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct UpdateSecretOk {}

    impl UpdateSecretOk {
        /// Get the AMQP class id for update-secret-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            10
        }

        /// Get the AMQP method id for update-secret-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            71
        }
    }

    /// Parse update-secret-ok (Generated)
    pub fn parse_update_secret_ok<I: ParsableInput>(i: I) -> ParserResult<I, UpdateSecretOk> {
        Ok((i, UpdateSecretOk {}))
    }

    /// Serialize update-secret-ok (Generated)
    pub fn gen_update_secret_ok<'a, W: Write + BackToTheBuffer + 'a>(
        _: &'a UpdateSecretOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(71)(input)?;
            Ok(input)
        }
    }
}
/// channel (generated)
pub mod channel {
    use super::*;

    /// Parse channel (Generated)
    pub fn parse_channel<I: ParsableInput>(i: I) -> ParserResult<I, channel::AMQPMethod> {
        context(
            "parse_channel",
            map_opt(
                flat_map(parse_id, |id| {
                    move |i| match id {
                        10 => {
                            context("parse_open", map(map(parse_open, AMQPMethod::Open), Some))(i)
                        }
                        11 => context(
                            "parse_open_ok",
                            map(map(parse_open_ok, AMQPMethod::OpenOk), Some),
                        )(i),
                        20 => {
                            context("parse_flow", map(map(parse_flow, AMQPMethod::Flow), Some))(i)
                        }
                        21 => context(
                            "parse_flow_ok",
                            map(map(parse_flow_ok, AMQPMethod::FlowOk), Some),
                        )(i),
                        40 => context(
                            "parse_close",
                            map(map(parse_close, AMQPMethod::Close), Some),
                        )(i),
                        41 => context(
                            "parse_close_ok",
                            map(map(parse_close_ok, AMQPMethod::CloseOk), Some),
                        )(i),
                        _ => Ok((i, None)),
                    }
                }),
                std::convert::identity,
            ),
        )(i)
    }

    /// Serialize channel (Generated)
    pub fn gen_channel<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a AMQPMethod,
    ) -> impl SerializeFn<W> + 'a {
        cookie_factory::sequence::pair(gen_id(20), move |input| match *method {
            AMQPMethod::Open(ref open) => gen_open(open)(input),
            AMQPMethod::OpenOk(ref open_ok) => gen_open_ok(open_ok)(input),
            AMQPMethod::Flow(ref flow) => gen_flow(flow)(input),
            AMQPMethod::FlowOk(ref flow_ok) => gen_flow_ok(flow_ok)(input),
            AMQPMethod::Close(ref close) => gen_close(close)(input),
            AMQPMethod::CloseOk(ref close_ok) => gen_close_ok(close_ok)(input),
        })
    }

    /// The available methods in channel
    #[derive(Clone, Debug, PartialEq)]
    pub enum AMQPMethod {
        /// open (Generated)
        Open(Open),
        /// open-ok (Generated)
        OpenOk(OpenOk),
        /// flow (Generated)
        Flow(Flow),
        /// flow-ok (Generated)
        FlowOk(FlowOk),
        /// close (Generated)
        Close(Close),
        /// close-ok (Generated)
        CloseOk(CloseOk),
    }

    /// open (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Open {}

    impl Open {
        /// Get the AMQP class id for open (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            20
        }

        /// Get the AMQP method id for open (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            10
        }
    }

    /// Parse open (Generated)
    pub fn parse_open<I: ParsableInput>(i: I) -> ParserResult<I, Open> {
        let (i, _) = parse_short_string(i)?;
        Ok((i, Open {}))
    }

    /// Serialize open (Generated)
    pub fn gen_open<'a, W: Write + BackToTheBuffer + 'a>(
        _method: &'a Open,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(10)(input)?;
            input = gen_short_string("")(input)?;
            Ok(input)
        }
    }
    /// open-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct OpenOk {}

    impl OpenOk {
        /// Get the AMQP class id for open-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            20
        }

        /// Get the AMQP method id for open-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            11
        }
    }

    /// Parse open-ok (Generated)
    pub fn parse_open_ok<I: ParsableInput>(i: I) -> ParserResult<I, OpenOk> {
        let (i, _) = parse_long_string(i)?;
        Ok((i, OpenOk {}))
    }

    /// Serialize open-ok (Generated)
    pub fn gen_open_ok<'a, W: Write + BackToTheBuffer + 'a>(
        _method: &'a OpenOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(11)(input)?;
            input = gen_long_string(b"")(input)?;
            Ok(input)
        }
    }
    /// flow (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Flow {
        /// active (Generated)
        pub active: Boolean,
    }

    impl Flow {
        /// Get the AMQP class id for flow (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            20
        }

        /// Get the AMQP method id for flow (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            20
        }
    }

    /// Parse flow (Generated)
    pub fn parse_flow<I: ParsableInput>(i: I) -> ParserResult<I, Flow> {
        let (i, flags) = parse_flags(i, &["active"])?;
        Ok((
            i,
            Flow {
                active: flags.get_flag("active").unwrap_or(false),
            },
        ))
    }

    /// Serialize flow (Generated)
    pub fn gen_flow<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Flow,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("active".to_string(), method.active);
            input = gen_id(20)(input)?;
            input = gen_flags(&flags)(input)?;
            Ok(input)
        }
    }
    /// flow-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct FlowOk {
        /// active (Generated)
        pub active: Boolean,
    }

    impl FlowOk {
        /// Get the AMQP class id for flow-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            20
        }

        /// Get the AMQP method id for flow-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            21
        }
    }

    /// Parse flow-ok (Generated)
    pub fn parse_flow_ok<I: ParsableInput>(i: I) -> ParserResult<I, FlowOk> {
        let (i, flags) = parse_flags(i, &["active"])?;
        Ok((
            i,
            FlowOk {
                active: flags.get_flag("active").unwrap_or(false),
            },
        ))
    }

    /// Serialize flow-ok (Generated)
    pub fn gen_flow_ok<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a FlowOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("active".to_string(), method.active);
            input = gen_id(21)(input)?;
            input = gen_flags(&flags)(input)?;
            Ok(input)
        }
    }
    /// close (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Close {
        /// reply-code (Generated)
        pub reply_code: ShortUInt,
        /// reply-text (Generated)
        pub reply_text: ShortString,
        /// class-id (Generated)
        pub class_id: ShortUInt,
        /// method-id (Generated)
        pub method_id: ShortUInt,
    }

    impl Close {
        /// Get the AMQP class id for close (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            20
        }

        /// Get the AMQP method id for close (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            40
        }
    }

    /// Parse close (Generated)
    pub fn parse_close<I: ParsableInput>(i: I) -> ParserResult<I, Close> {
        let (i, reply_code) = parse_short_uint(i)?;
        let (i, reply_text) = parse_short_string(i)?;
        let (i, class_id) = parse_short_uint(i)?;
        let (i, method_id) = parse_short_uint(i)?;
        Ok((
            i,
            Close {
                reply_code,
                reply_text,
                class_id,
                method_id,
            },
        ))
    }

    /// Serialize close (Generated)
    pub fn gen_close<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Close,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(40)(input)?;
            input = gen_short_uint(method.reply_code)(input)?;
            input = gen_short_string(method.reply_text.as_str())(input)?;
            input = gen_short_uint(method.class_id)(input)?;
            input = gen_short_uint(method.method_id)(input)?;
            Ok(input)
        }
    }
    /// close-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct CloseOk {}

    impl CloseOk {
        /// Get the AMQP class id for close-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            20
        }

        /// Get the AMQP method id for close-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            41
        }
    }

    /// Parse close-ok (Generated)
    pub fn parse_close_ok<I: ParsableInput>(i: I) -> ParserResult<I, CloseOk> {
        Ok((i, CloseOk {}))
    }

    /// Serialize close-ok (Generated)
    pub fn gen_close_ok<'a, W: Write + BackToTheBuffer + 'a>(
        _: &'a CloseOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(41)(input)?;
            Ok(input)
        }
    }
}
/// access (generated)
pub mod access {
    use super::*;

    /// Parse access (Generated)
    pub fn parse_access<I: ParsableInput>(i: I) -> ParserResult<I, access::AMQPMethod> {
        context(
            "parse_access",
            map_opt(
                flat_map(parse_id, |id| {
                    move |i| match id {
                        10 => context(
                            "parse_request",
                            map(map(parse_request, AMQPMethod::Request), Some),
                        )(i),
                        11 => context(
                            "parse_request_ok",
                            map(map(parse_request_ok, AMQPMethod::RequestOk), Some),
                        )(i),
                        _ => Ok((i, None)),
                    }
                }),
                std::convert::identity,
            ),
        )(i)
    }

    /// Serialize access (Generated)
    pub fn gen_access<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a AMQPMethod,
    ) -> impl SerializeFn<W> + 'a {
        cookie_factory::sequence::pair(gen_id(30), move |input| match *method {
            AMQPMethod::Request(ref request) => gen_request(request)(input),
            AMQPMethod::RequestOk(ref request_ok) => gen_request_ok(request_ok)(input),
        })
    }

    /// The available methods in access
    #[derive(Clone, Debug, PartialEq)]
    pub enum AMQPMethod {
        /// request (Generated)
        Request(Request),
        /// request-ok (Generated)
        RequestOk(RequestOk),
    }

    /// request (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Request {
        /// realm (Generated)
        pub realm: ShortString,
        /// exclusive (Generated)
        pub exclusive: Boolean,
        /// passive (Generated)
        pub passive: Boolean,
        /// active (Generated)
        pub active: Boolean,
        /// write (Generated)
        pub write: Boolean,
        /// read (Generated)
        pub read: Boolean,
    }

    impl Request {
        /// Get the AMQP class id for request (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            30
        }

        /// Get the AMQP method id for request (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            10
        }
    }

    /// Parse request (Generated)
    pub fn parse_request<I: ParsableInput>(i: I) -> ParserResult<I, Request> {
        let (i, realm) = parse_short_string(i)?;
        let (i, flags) = parse_flags(i, &["exclusive", "passive", "active", "write", "read"])?;
        Ok((
            i,
            Request {
                realm,
                exclusive: flags.get_flag("exclusive").unwrap_or(false),
                passive: flags.get_flag("passive").unwrap_or(false),
                active: flags.get_flag("active").unwrap_or(false),
                write: flags.get_flag("write").unwrap_or(false),
                read: flags.get_flag("read").unwrap_or(false),
            },
        ))
    }

    /// Serialize request (Generated)
    pub fn gen_request<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Request,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("exclusive".to_string(), method.exclusive);
            flags.add_flag("passive".to_string(), method.passive);
            flags.add_flag("active".to_string(), method.active);
            flags.add_flag("write".to_string(), method.write);
            flags.add_flag("read".to_string(), method.read);
            input = gen_id(10)(input)?;
            input = gen_short_string(method.realm.as_str())(input)?;
            input = gen_flags(&flags)(input)?;
            Ok(input)
        }
    }
    /// request-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct RequestOk {}

    impl RequestOk {
        /// Get the AMQP class id for request-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            30
        }

        /// Get the AMQP method id for request-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            11
        }
    }

    /// Parse request-ok (Generated)
    pub fn parse_request_ok<I: ParsableInput>(i: I) -> ParserResult<I, RequestOk> {
        let (i, _) = parse_short_uint(i)?;
        Ok((i, RequestOk {}))
    }

    /// Serialize request-ok (Generated)
    pub fn gen_request_ok<'a, W: Write + BackToTheBuffer + 'a>(
        _method: &'a RequestOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(11)(input)?;
            input = gen_short_uint(1)(input)?;
            Ok(input)
        }
    }
}
/// exchange (generated)
pub mod exchange {
    use super::*;

    /// Parse exchange (Generated)
    pub fn parse_exchange<I: ParsableInput>(i: I) -> ParserResult<I, exchange::AMQPMethod> {
        context(
            "parse_exchange",
            map_opt(
                flat_map(parse_id, |id| {
                    move |i| match id {
                        10 => context(
                            "parse_declare",
                            map(map(parse_declare, AMQPMethod::Declare), Some),
                        )(i),
                        11 => context(
                            "parse_declare_ok",
                            map(map(parse_declare_ok, AMQPMethod::DeclareOk), Some),
                        )(i),
                        20 => context(
                            "parse_delete",
                            map(map(parse_delete, AMQPMethod::Delete), Some),
                        )(i),
                        21 => context(
                            "parse_delete_ok",
                            map(map(parse_delete_ok, AMQPMethod::DeleteOk), Some),
                        )(i),
                        30 => {
                            context("parse_bind", map(map(parse_bind, AMQPMethod::Bind), Some))(i)
                        }
                        31 => context(
                            "parse_bind_ok",
                            map(map(parse_bind_ok, AMQPMethod::BindOk), Some),
                        )(i),
                        40 => context(
                            "parse_unbind",
                            map(map(parse_unbind, AMQPMethod::Unbind), Some),
                        )(i),
                        51 => context(
                            "parse_unbind_ok",
                            map(map(parse_unbind_ok, AMQPMethod::UnbindOk), Some),
                        )(i),
                        _ => Ok((i, None)),
                    }
                }),
                std::convert::identity,
            ),
        )(i)
    }

    /// Serialize exchange (Generated)
    pub fn gen_exchange<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a AMQPMethod,
    ) -> impl SerializeFn<W> + 'a {
        cookie_factory::sequence::pair(gen_id(40), move |input| match *method {
            AMQPMethod::Declare(ref declare) => gen_declare(declare)(input),
            AMQPMethod::DeclareOk(ref declare_ok) => gen_declare_ok(declare_ok)(input),
            AMQPMethod::Delete(ref delete) => gen_delete(delete)(input),
            AMQPMethod::DeleteOk(ref delete_ok) => gen_delete_ok(delete_ok)(input),
            AMQPMethod::Bind(ref bind) => gen_bind(bind)(input),
            AMQPMethod::BindOk(ref bind_ok) => gen_bind_ok(bind_ok)(input),
            AMQPMethod::Unbind(ref unbind) => gen_unbind(unbind)(input),
            AMQPMethod::UnbindOk(ref unbind_ok) => gen_unbind_ok(unbind_ok)(input),
        })
    }

    /// The available methods in exchange
    #[derive(Clone, Debug, PartialEq)]
    pub enum AMQPMethod {
        /// declare (Generated)
        Declare(Declare),
        /// declare-ok (Generated)
        DeclareOk(DeclareOk),
        /// delete (Generated)
        Delete(Delete),
        /// delete-ok (Generated)
        DeleteOk(DeleteOk),
        /// bind (Generated)
        Bind(Bind),
        /// bind-ok (Generated)
        BindOk(BindOk),
        /// unbind (Generated)
        Unbind(Unbind),
        /// unbind-ok (Generated)
        UnbindOk(UnbindOk),
    }

    /// declare (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Declare {
        /// exchange (Generated)
        pub exchange: ShortString,
        /// type (Generated)
        pub kind: ShortString,
        /// passive (Generated)
        pub passive: Boolean,
        /// durable (Generated)
        pub durable: Boolean,
        /// auto-delete (Generated)
        pub auto_delete: Boolean,
        /// internal (Generated)
        pub internal: Boolean,
        /// nowait (Generated)
        pub nowait: Boolean,
        /// arguments (Generated)
        pub arguments: FieldTable,
    }

    impl Declare {
        /// Get the AMQP class id for declare (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            40
        }

        /// Get the AMQP method id for declare (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            10
        }
    }

    /// Parse declare (Generated)
    pub fn parse_declare<I: ParsableInput>(i: I) -> ParserResult<I, Declare> {
        let (i, _) = parse_short_uint(i)?;
        let (i, exchange) = parse_short_string(i)?;
        let (i, kind) = parse_short_string(i)?;
        let (i, flags) = parse_flags(
            i,
            &["passive", "durable", "auto-delete", "internal", "nowait"],
        )?;
        let (i, arguments) = parse_field_table(i)?;
        Ok((
            i,
            Declare {
                exchange,
                kind,
                passive: flags.get_flag("passive").unwrap_or(false),
                durable: flags.get_flag("durable").unwrap_or(false),
                auto_delete: flags.get_flag("auto_delete").unwrap_or(false),
                internal: flags.get_flag("internal").unwrap_or(false),
                nowait: flags.get_flag("nowait").unwrap_or(false),
                arguments,
            },
        ))
    }

    /// Serialize declare (Generated)
    pub fn gen_declare<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Declare,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("passive".to_string(), method.passive);
            flags.add_flag("durable".to_string(), method.durable);
            flags.add_flag("auto_delete".to_string(), method.auto_delete);
            flags.add_flag("internal".to_string(), method.internal);
            flags.add_flag("nowait".to_string(), method.nowait);
            input = gen_id(10)(input)?;
            input = gen_short_uint(0)(input)?;
            input = gen_short_string(method.exchange.as_str())(input)?;
            input = gen_short_string(method.kind.as_str())(input)?;
            input = gen_flags(&flags)(input)?;
            input = gen_field_table(&method.arguments)(input)?;
            Ok(input)
        }
    }
    /// declare-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct DeclareOk {}

    impl DeclareOk {
        /// Get the AMQP class id for declare-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            40
        }

        /// Get the AMQP method id for declare-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            11
        }
    }

    /// Parse declare-ok (Generated)
    pub fn parse_declare_ok<I: ParsableInput>(i: I) -> ParserResult<I, DeclareOk> {
        Ok((i, DeclareOk {}))
    }

    /// Serialize declare-ok (Generated)
    pub fn gen_declare_ok<'a, W: Write + BackToTheBuffer + 'a>(
        _: &'a DeclareOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(11)(input)?;
            Ok(input)
        }
    }
    /// delete (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Delete {
        /// exchange (Generated)
        pub exchange: ShortString,
        /// if-unused (Generated)
        pub if_unused: Boolean,
        /// nowait (Generated)
        pub nowait: Boolean,
    }

    impl Delete {
        /// Get the AMQP class id for delete (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            40
        }

        /// Get the AMQP method id for delete (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            20
        }
    }

    /// Parse delete (Generated)
    pub fn parse_delete<I: ParsableInput>(i: I) -> ParserResult<I, Delete> {
        let (i, _) = parse_short_uint(i)?;
        let (i, exchange) = parse_short_string(i)?;
        let (i, flags) = parse_flags(i, &["if-unused", "nowait"])?;
        Ok((
            i,
            Delete {
                exchange,
                if_unused: flags.get_flag("if_unused").unwrap_or(false),
                nowait: flags.get_flag("nowait").unwrap_or(false),
            },
        ))
    }

    /// Serialize delete (Generated)
    pub fn gen_delete<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Delete,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("if_unused".to_string(), method.if_unused);
            flags.add_flag("nowait".to_string(), method.nowait);
            input = gen_id(20)(input)?;
            input = gen_short_uint(0)(input)?;
            input = gen_short_string(method.exchange.as_str())(input)?;
            input = gen_flags(&flags)(input)?;
            Ok(input)
        }
    }
    /// delete-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct DeleteOk {}

    impl DeleteOk {
        /// Get the AMQP class id for delete-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            40
        }

        /// Get the AMQP method id for delete-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            21
        }
    }

    /// Parse delete-ok (Generated)
    pub fn parse_delete_ok<I: ParsableInput>(i: I) -> ParserResult<I, DeleteOk> {
        Ok((i, DeleteOk {}))
    }

    /// Serialize delete-ok (Generated)
    pub fn gen_delete_ok<'a, W: Write + BackToTheBuffer + 'a>(
        _: &'a DeleteOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(21)(input)?;
            Ok(input)
        }
    }
    /// bind (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Bind {
        /// destination (Generated)
        pub destination: ShortString,
        /// source (Generated)
        pub source: ShortString,
        /// routing-key (Generated)
        pub routing_key: ShortString,
        /// nowait (Generated)
        pub nowait: Boolean,
        /// arguments (Generated)
        pub arguments: FieldTable,
    }

    impl Bind {
        /// Get the AMQP class id for bind (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            40
        }

        /// Get the AMQP method id for bind (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            30
        }
    }

    /// Parse bind (Generated)
    pub fn parse_bind<I: ParsableInput>(i: I) -> ParserResult<I, Bind> {
        let (i, _) = parse_short_uint(i)?;
        let (i, destination) = parse_short_string(i)?;
        let (i, source) = parse_short_string(i)?;
        let (i, routing_key) = parse_short_string(i)?;
        let (i, flags) = parse_flags(i, &["nowait"])?;
        let (i, arguments) = parse_field_table(i)?;
        Ok((
            i,
            Bind {
                destination,
                source,
                routing_key,
                nowait: flags.get_flag("nowait").unwrap_or(false),
                arguments,
            },
        ))
    }

    /// Serialize bind (Generated)
    pub fn gen_bind<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Bind,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("nowait".to_string(), method.nowait);
            input = gen_id(30)(input)?;
            input = gen_short_uint(0)(input)?;
            input = gen_short_string(method.destination.as_str())(input)?;
            input = gen_short_string(method.source.as_str())(input)?;
            input = gen_short_string(method.routing_key.as_str())(input)?;
            input = gen_flags(&flags)(input)?;
            input = gen_field_table(&method.arguments)(input)?;
            Ok(input)
        }
    }
    /// bind-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct BindOk {}

    impl BindOk {
        /// Get the AMQP class id for bind-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            40
        }

        /// Get the AMQP method id for bind-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            31
        }
    }

    /// Parse bind-ok (Generated)
    pub fn parse_bind_ok<I: ParsableInput>(i: I) -> ParserResult<I, BindOk> {
        Ok((i, BindOk {}))
    }

    /// Serialize bind-ok (Generated)
    pub fn gen_bind_ok<'a, W: Write + BackToTheBuffer + 'a>(
        _: &'a BindOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(31)(input)?;
            Ok(input)
        }
    }
    /// unbind (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Unbind {
        /// destination (Generated)
        pub destination: ShortString,
        /// source (Generated)
        pub source: ShortString,
        /// routing-key (Generated)
        pub routing_key: ShortString,
        /// nowait (Generated)
        pub nowait: Boolean,
        /// arguments (Generated)
        pub arguments: FieldTable,
    }

    impl Unbind {
        /// Get the AMQP class id for unbind (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            40
        }

        /// Get the AMQP method id for unbind (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            40
        }
    }

    /// Parse unbind (Generated)
    pub fn parse_unbind<I: ParsableInput>(i: I) -> ParserResult<I, Unbind> {
        let (i, _) = parse_short_uint(i)?;
        let (i, destination) = parse_short_string(i)?;
        let (i, source) = parse_short_string(i)?;
        let (i, routing_key) = parse_short_string(i)?;
        let (i, flags) = parse_flags(i, &["nowait"])?;
        let (i, arguments) = parse_field_table(i)?;
        Ok((
            i,
            Unbind {
                destination,
                source,
                routing_key,
                nowait: flags.get_flag("nowait").unwrap_or(false),
                arguments,
            },
        ))
    }

    /// Serialize unbind (Generated)
    pub fn gen_unbind<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Unbind,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("nowait".to_string(), method.nowait);
            input = gen_id(40)(input)?;
            input = gen_short_uint(0)(input)?;
            input = gen_short_string(method.destination.as_str())(input)?;
            input = gen_short_string(method.source.as_str())(input)?;
            input = gen_short_string(method.routing_key.as_str())(input)?;
            input = gen_flags(&flags)(input)?;
            input = gen_field_table(&method.arguments)(input)?;
            Ok(input)
        }
    }
    /// unbind-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct UnbindOk {}

    impl UnbindOk {
        /// Get the AMQP class id for unbind-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            40
        }

        /// Get the AMQP method id for unbind-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            51
        }
    }

    /// Parse unbind-ok (Generated)
    pub fn parse_unbind_ok<I: ParsableInput>(i: I) -> ParserResult<I, UnbindOk> {
        Ok((i, UnbindOk {}))
    }

    /// Serialize unbind-ok (Generated)
    pub fn gen_unbind_ok<'a, W: Write + BackToTheBuffer + 'a>(
        _: &'a UnbindOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(51)(input)?;
            Ok(input)
        }
    }
}
/// queue (generated)
pub mod queue {
    use super::*;

    /// Parse queue (Generated)
    pub fn parse_queue<I: ParsableInput>(i: I) -> ParserResult<I, queue::AMQPMethod> {
        context(
            "parse_queue",
            map_opt(
                flat_map(parse_id, |id| {
                    move |i| match id {
                        10 => context(
                            "parse_declare",
                            map(map(parse_declare, AMQPMethod::Declare), Some),
                        )(i),
                        11 => context(
                            "parse_declare_ok",
                            map(map(parse_declare_ok, AMQPMethod::DeclareOk), Some),
                        )(i),
                        20 => {
                            context("parse_bind", map(map(parse_bind, AMQPMethod::Bind), Some))(i)
                        }
                        21 => context(
                            "parse_bind_ok",
                            map(map(parse_bind_ok, AMQPMethod::BindOk), Some),
                        )(i),
                        30 => context(
                            "parse_purge",
                            map(map(parse_purge, AMQPMethod::Purge), Some),
                        )(i),
                        31 => context(
                            "parse_purge_ok",
                            map(map(parse_purge_ok, AMQPMethod::PurgeOk), Some),
                        )(i),
                        40 => context(
                            "parse_delete",
                            map(map(parse_delete, AMQPMethod::Delete), Some),
                        )(i),
                        41 => context(
                            "parse_delete_ok",
                            map(map(parse_delete_ok, AMQPMethod::DeleteOk), Some),
                        )(i),
                        50 => context(
                            "parse_unbind",
                            map(map(parse_unbind, AMQPMethod::Unbind), Some),
                        )(i),
                        51 => context(
                            "parse_unbind_ok",
                            map(map(parse_unbind_ok, AMQPMethod::UnbindOk), Some),
                        )(i),
                        _ => Ok((i, None)),
                    }
                }),
                std::convert::identity,
            ),
        )(i)
    }

    /// Serialize queue (Generated)
    pub fn gen_queue<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a AMQPMethod,
    ) -> impl SerializeFn<W> + 'a {
        cookie_factory::sequence::pair(gen_id(50), move |input| match *method {
            AMQPMethod::Declare(ref declare) => gen_declare(declare)(input),
            AMQPMethod::DeclareOk(ref declare_ok) => gen_declare_ok(declare_ok)(input),
            AMQPMethod::Bind(ref bind) => gen_bind(bind)(input),
            AMQPMethod::BindOk(ref bind_ok) => gen_bind_ok(bind_ok)(input),
            AMQPMethod::Purge(ref purge) => gen_purge(purge)(input),
            AMQPMethod::PurgeOk(ref purge_ok) => gen_purge_ok(purge_ok)(input),
            AMQPMethod::Delete(ref delete) => gen_delete(delete)(input),
            AMQPMethod::DeleteOk(ref delete_ok) => gen_delete_ok(delete_ok)(input),
            AMQPMethod::Unbind(ref unbind) => gen_unbind(unbind)(input),
            AMQPMethod::UnbindOk(ref unbind_ok) => gen_unbind_ok(unbind_ok)(input),
        })
    }

    /// The available methods in queue
    #[derive(Clone, Debug, PartialEq)]
    pub enum AMQPMethod {
        /// declare (Generated)
        Declare(Declare),
        /// declare-ok (Generated)
        DeclareOk(DeclareOk),
        /// bind (Generated)
        Bind(Bind),
        /// bind-ok (Generated)
        BindOk(BindOk),
        /// purge (Generated)
        Purge(Purge),
        /// purge-ok (Generated)
        PurgeOk(PurgeOk),
        /// delete (Generated)
        Delete(Delete),
        /// delete-ok (Generated)
        DeleteOk(DeleteOk),
        /// unbind (Generated)
        Unbind(Unbind),
        /// unbind-ok (Generated)
        UnbindOk(UnbindOk),
    }

    /// declare (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Declare {
        /// queue (Generated)
        pub queue: ShortString,
        /// passive (Generated)
        pub passive: Boolean,
        /// durable (Generated)
        pub durable: Boolean,
        /// exclusive (Generated)
        pub exclusive: Boolean,
        /// auto-delete (Generated)
        pub auto_delete: Boolean,
        /// nowait (Generated)
        pub nowait: Boolean,
        /// arguments (Generated)
        pub arguments: FieldTable,
    }

    impl Declare {
        /// Get the AMQP class id for declare (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            50
        }

        /// Get the AMQP method id for declare (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            10
        }
    }

    /// Parse declare (Generated)
    pub fn parse_declare<I: ParsableInput>(i: I) -> ParserResult<I, Declare> {
        let (i, _) = parse_short_uint(i)?;
        let (i, queue) = parse_short_string(i)?;
        let (i, flags) = parse_flags(
            i,
            &["passive", "durable", "exclusive", "auto-delete", "nowait"],
        )?;
        let (i, arguments) = parse_field_table(i)?;
        Ok((
            i,
            Declare {
                queue,
                passive: flags.get_flag("passive").unwrap_or(false),
                durable: flags.get_flag("durable").unwrap_or(false),
                exclusive: flags.get_flag("exclusive").unwrap_or(false),
                auto_delete: flags.get_flag("auto_delete").unwrap_or(false),
                nowait: flags.get_flag("nowait").unwrap_or(false),
                arguments,
            },
        ))
    }

    /// Serialize declare (Generated)
    pub fn gen_declare<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Declare,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("passive".to_string(), method.passive);
            flags.add_flag("durable".to_string(), method.durable);
            flags.add_flag("exclusive".to_string(), method.exclusive);
            flags.add_flag("auto_delete".to_string(), method.auto_delete);
            flags.add_flag("nowait".to_string(), method.nowait);
            input = gen_id(10)(input)?;
            input = gen_short_uint(0)(input)?;
            input = gen_short_string(method.queue.as_str())(input)?;
            input = gen_flags(&flags)(input)?;
            input = gen_field_table(&method.arguments)(input)?;
            Ok(input)
        }
    }
    /// declare-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct DeclareOk {
        /// queue (Generated)
        pub queue: ShortString,
        /// message-count (Generated)
        pub message_count: LongUInt,
        /// consumer-count (Generated)
        pub consumer_count: LongUInt,
    }

    impl DeclareOk {
        /// Get the AMQP class id for declare-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            50
        }

        /// Get the AMQP method id for declare-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            11
        }
    }

    /// Parse declare-ok (Generated)
    pub fn parse_declare_ok<I: ParsableInput>(i: I) -> ParserResult<I, DeclareOk> {
        let (i, queue) = parse_short_string(i)?;
        let (i, message_count) = parse_long_uint(i)?;
        let (i, consumer_count) = parse_long_uint(i)?;
        Ok((
            i,
            DeclareOk {
                queue,
                message_count,
                consumer_count,
            },
        ))
    }

    /// Serialize declare-ok (Generated)
    pub fn gen_declare_ok<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a DeclareOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(11)(input)?;
            input = gen_short_string(method.queue.as_str())(input)?;
            input = gen_long_uint(method.message_count)(input)?;
            input = gen_long_uint(method.consumer_count)(input)?;
            Ok(input)
        }
    }
    /// bind (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Bind {
        /// queue (Generated)
        pub queue: ShortString,
        /// exchange (Generated)
        pub exchange: ShortString,
        /// routing-key (Generated)
        pub routing_key: ShortString,
        /// nowait (Generated)
        pub nowait: Boolean,
        /// arguments (Generated)
        pub arguments: FieldTable,
    }

    impl Bind {
        /// Get the AMQP class id for bind (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            50
        }

        /// Get the AMQP method id for bind (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            20
        }
    }

    /// Parse bind (Generated)
    pub fn parse_bind<I: ParsableInput>(i: I) -> ParserResult<I, Bind> {
        let (i, _) = parse_short_uint(i)?;
        let (i, queue) = parse_short_string(i)?;
        let (i, exchange) = parse_short_string(i)?;
        let (i, routing_key) = parse_short_string(i)?;
        let (i, flags) = parse_flags(i, &["nowait"])?;
        let (i, arguments) = parse_field_table(i)?;
        Ok((
            i,
            Bind {
                queue,
                exchange,
                routing_key,
                nowait: flags.get_flag("nowait").unwrap_or(false),
                arguments,
            },
        ))
    }

    /// Serialize bind (Generated)
    pub fn gen_bind<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Bind,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("nowait".to_string(), method.nowait);
            input = gen_id(20)(input)?;
            input = gen_short_uint(0)(input)?;
            input = gen_short_string(method.queue.as_str())(input)?;
            input = gen_short_string(method.exchange.as_str())(input)?;
            input = gen_short_string(method.routing_key.as_str())(input)?;
            input = gen_flags(&flags)(input)?;
            input = gen_field_table(&method.arguments)(input)?;
            Ok(input)
        }
    }
    /// bind-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct BindOk {}

    impl BindOk {
        /// Get the AMQP class id for bind-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            50
        }

        /// Get the AMQP method id for bind-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            21
        }
    }

    /// Parse bind-ok (Generated)
    pub fn parse_bind_ok<I: ParsableInput>(i: I) -> ParserResult<I, BindOk> {
        Ok((i, BindOk {}))
    }

    /// Serialize bind-ok (Generated)
    pub fn gen_bind_ok<'a, W: Write + BackToTheBuffer + 'a>(
        _: &'a BindOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(21)(input)?;
            Ok(input)
        }
    }
    /// purge (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Purge {
        /// queue (Generated)
        pub queue: ShortString,
        /// nowait (Generated)
        pub nowait: Boolean,
    }

    impl Purge {
        /// Get the AMQP class id for purge (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            50
        }

        /// Get the AMQP method id for purge (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            30
        }
    }

    /// Parse purge (Generated)
    pub fn parse_purge<I: ParsableInput>(i: I) -> ParserResult<I, Purge> {
        let (i, _) = parse_short_uint(i)?;
        let (i, queue) = parse_short_string(i)?;
        let (i, flags) = parse_flags(i, &["nowait"])?;
        Ok((
            i,
            Purge {
                queue,
                nowait: flags.get_flag("nowait").unwrap_or(false),
            },
        ))
    }

    /// Serialize purge (Generated)
    pub fn gen_purge<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Purge,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("nowait".to_string(), method.nowait);
            input = gen_id(30)(input)?;
            input = gen_short_uint(0)(input)?;
            input = gen_short_string(method.queue.as_str())(input)?;
            input = gen_flags(&flags)(input)?;
            Ok(input)
        }
    }
    /// purge-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct PurgeOk {
        /// message-count (Generated)
        pub message_count: LongUInt,
    }

    impl PurgeOk {
        /// Get the AMQP class id for purge-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            50
        }

        /// Get the AMQP method id for purge-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            31
        }
    }

    /// Parse purge-ok (Generated)
    pub fn parse_purge_ok<I: ParsableInput>(i: I) -> ParserResult<I, PurgeOk> {
        let (i, message_count) = parse_long_uint(i)?;
        Ok((i, PurgeOk { message_count }))
    }

    /// Serialize purge-ok (Generated)
    pub fn gen_purge_ok<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a PurgeOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(31)(input)?;
            input = gen_long_uint(method.message_count)(input)?;
            Ok(input)
        }
    }
    /// delete (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Delete {
        /// queue (Generated)
        pub queue: ShortString,
        /// if-unused (Generated)
        pub if_unused: Boolean,
        /// if-empty (Generated)
        pub if_empty: Boolean,
        /// nowait (Generated)
        pub nowait: Boolean,
    }

    impl Delete {
        /// Get the AMQP class id for delete (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            50
        }

        /// Get the AMQP method id for delete (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            40
        }
    }

    /// Parse delete (Generated)
    pub fn parse_delete<I: ParsableInput>(i: I) -> ParserResult<I, Delete> {
        let (i, _) = parse_short_uint(i)?;
        let (i, queue) = parse_short_string(i)?;
        let (i, flags) = parse_flags(i, &["if-unused", "if-empty", "nowait"])?;
        Ok((
            i,
            Delete {
                queue,
                if_unused: flags.get_flag("if_unused").unwrap_or(false),
                if_empty: flags.get_flag("if_empty").unwrap_or(false),
                nowait: flags.get_flag("nowait").unwrap_or(false),
            },
        ))
    }

    /// Serialize delete (Generated)
    pub fn gen_delete<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Delete,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("if_unused".to_string(), method.if_unused);
            flags.add_flag("if_empty".to_string(), method.if_empty);
            flags.add_flag("nowait".to_string(), method.nowait);
            input = gen_id(40)(input)?;
            input = gen_short_uint(0)(input)?;
            input = gen_short_string(method.queue.as_str())(input)?;
            input = gen_flags(&flags)(input)?;
            Ok(input)
        }
    }
    /// delete-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct DeleteOk {
        /// message-count (Generated)
        pub message_count: LongUInt,
    }

    impl DeleteOk {
        /// Get the AMQP class id for delete-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            50
        }

        /// Get the AMQP method id for delete-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            41
        }
    }

    /// Parse delete-ok (Generated)
    pub fn parse_delete_ok<I: ParsableInput>(i: I) -> ParserResult<I, DeleteOk> {
        let (i, message_count) = parse_long_uint(i)?;
        Ok((i, DeleteOk { message_count }))
    }

    /// Serialize delete-ok (Generated)
    pub fn gen_delete_ok<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a DeleteOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(41)(input)?;
            input = gen_long_uint(method.message_count)(input)?;
            Ok(input)
        }
    }
    /// unbind (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Unbind {
        /// queue (Generated)
        pub queue: ShortString,
        /// exchange (Generated)
        pub exchange: ShortString,
        /// routing-key (Generated)
        pub routing_key: ShortString,
        /// arguments (Generated)
        pub arguments: FieldTable,
    }

    impl Unbind {
        /// Get the AMQP class id for unbind (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            50
        }

        /// Get the AMQP method id for unbind (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            50
        }
    }

    /// Parse unbind (Generated)
    pub fn parse_unbind<I: ParsableInput>(i: I) -> ParserResult<I, Unbind> {
        let (i, _) = parse_short_uint(i)?;
        let (i, queue) = parse_short_string(i)?;
        let (i, exchange) = parse_short_string(i)?;
        let (i, routing_key) = parse_short_string(i)?;
        let (i, arguments) = parse_field_table(i)?;
        Ok((
            i,
            Unbind {
                queue,
                exchange,
                routing_key,
                arguments,
            },
        ))
    }

    /// Serialize unbind (Generated)
    pub fn gen_unbind<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Unbind,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(50)(input)?;
            input = gen_short_uint(0)(input)?;
            input = gen_short_string(method.queue.as_str())(input)?;
            input = gen_short_string(method.exchange.as_str())(input)?;
            input = gen_short_string(method.routing_key.as_str())(input)?;
            input = gen_field_table(&method.arguments)(input)?;
            Ok(input)
        }
    }
    /// unbind-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct UnbindOk {}

    impl UnbindOk {
        /// Get the AMQP class id for unbind-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            50
        }

        /// Get the AMQP method id for unbind-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            51
        }
    }

    /// Parse unbind-ok (Generated)
    pub fn parse_unbind_ok<I: ParsableInput>(i: I) -> ParserResult<I, UnbindOk> {
        Ok((i, UnbindOk {}))
    }

    /// Serialize unbind-ok (Generated)
    pub fn gen_unbind_ok<'a, W: Write + BackToTheBuffer + 'a>(
        _: &'a UnbindOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(51)(input)?;
            Ok(input)
        }
    }
}
/// tx (generated)
pub mod tx {
    use super::*;

    /// Parse tx (Generated)
    pub fn parse_tx<I: ParsableInput>(i: I) -> ParserResult<I, tx::AMQPMethod> {
        context(
            "parse_tx",
            map_opt(
                flat_map(parse_id, |id| {
                    move |i| match id {
                        10 => context(
                            "parse_select",
                            map(map(parse_select, AMQPMethod::Select), Some),
                        )(i),
                        11 => context(
                            "parse_select_ok",
                            map(map(parse_select_ok, AMQPMethod::SelectOk), Some),
                        )(i),
                        20 => context(
                            "parse_commit",
                            map(map(parse_commit, AMQPMethod::Commit), Some),
                        )(i),
                        21 => context(
                            "parse_commit_ok",
                            map(map(parse_commit_ok, AMQPMethod::CommitOk), Some),
                        )(i),
                        30 => context(
                            "parse_rollback",
                            map(map(parse_rollback, AMQPMethod::Rollback), Some),
                        )(i),
                        31 => context(
                            "parse_rollback_ok",
                            map(map(parse_rollback_ok, AMQPMethod::RollbackOk), Some),
                        )(i),
                        _ => Ok((i, None)),
                    }
                }),
                std::convert::identity,
            ),
        )(i)
    }

    /// Serialize tx (Generated)
    pub fn gen_tx<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a AMQPMethod,
    ) -> impl SerializeFn<W> + 'a {
        cookie_factory::sequence::pair(gen_id(90), move |input| match *method {
            AMQPMethod::Select(ref select) => gen_select(select)(input),
            AMQPMethod::SelectOk(ref select_ok) => gen_select_ok(select_ok)(input),
            AMQPMethod::Commit(ref commit) => gen_commit(commit)(input),
            AMQPMethod::CommitOk(ref commit_ok) => gen_commit_ok(commit_ok)(input),
            AMQPMethod::Rollback(ref rollback) => gen_rollback(rollback)(input),
            AMQPMethod::RollbackOk(ref rollback_ok) => gen_rollback_ok(rollback_ok)(input),
        })
    }

    /// The available methods in tx
    #[derive(Clone, Debug, PartialEq)]
    pub enum AMQPMethod {
        /// select (Generated)
        Select(Select),
        /// select-ok (Generated)
        SelectOk(SelectOk),
        /// commit (Generated)
        Commit(Commit),
        /// commit-ok (Generated)
        CommitOk(CommitOk),
        /// rollback (Generated)
        Rollback(Rollback),
        /// rollback-ok (Generated)
        RollbackOk(RollbackOk),
    }

    /// select (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Select {}

    impl Select {
        /// Get the AMQP class id for select (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            90
        }

        /// Get the AMQP method id for select (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            10
        }
    }

    /// Parse select (Generated)
    pub fn parse_select<I: ParsableInput>(i: I) -> ParserResult<I, Select> {
        Ok((i, Select {}))
    }

    /// Serialize select (Generated)
    pub fn gen_select<'a, W: Write + BackToTheBuffer + 'a>(
        _: &'a Select,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(10)(input)?;
            Ok(input)
        }
    }
    /// select-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct SelectOk {}

    impl SelectOk {
        /// Get the AMQP class id for select-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            90
        }

        /// Get the AMQP method id for select-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            11
        }
    }

    /// Parse select-ok (Generated)
    pub fn parse_select_ok<I: ParsableInput>(i: I) -> ParserResult<I, SelectOk> {
        Ok((i, SelectOk {}))
    }

    /// Serialize select-ok (Generated)
    pub fn gen_select_ok<'a, W: Write + BackToTheBuffer + 'a>(
        _: &'a SelectOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(11)(input)?;
            Ok(input)
        }
    }
    /// commit (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Commit {}

    impl Commit {
        /// Get the AMQP class id for commit (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            90
        }

        /// Get the AMQP method id for commit (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            20
        }
    }

    /// Parse commit (Generated)
    pub fn parse_commit<I: ParsableInput>(i: I) -> ParserResult<I, Commit> {
        Ok((i, Commit {}))
    }

    /// Serialize commit (Generated)
    pub fn gen_commit<'a, W: Write + BackToTheBuffer + 'a>(
        _: &'a Commit,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(20)(input)?;
            Ok(input)
        }
    }
    /// commit-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct CommitOk {}

    impl CommitOk {
        /// Get the AMQP class id for commit-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            90
        }

        /// Get the AMQP method id for commit-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            21
        }
    }

    /// Parse commit-ok (Generated)
    pub fn parse_commit_ok<I: ParsableInput>(i: I) -> ParserResult<I, CommitOk> {
        Ok((i, CommitOk {}))
    }

    /// Serialize commit-ok (Generated)
    pub fn gen_commit_ok<'a, W: Write + BackToTheBuffer + 'a>(
        _: &'a CommitOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(21)(input)?;
            Ok(input)
        }
    }
    /// rollback (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Rollback {}

    impl Rollback {
        /// Get the AMQP class id for rollback (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            90
        }

        /// Get the AMQP method id for rollback (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            30
        }
    }

    /// Parse rollback (Generated)
    pub fn parse_rollback<I: ParsableInput>(i: I) -> ParserResult<I, Rollback> {
        Ok((i, Rollback {}))
    }

    /// Serialize rollback (Generated)
    pub fn gen_rollback<'a, W: Write + BackToTheBuffer + 'a>(
        _: &'a Rollback,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(30)(input)?;
            Ok(input)
        }
    }
    /// rollback-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct RollbackOk {}

    impl RollbackOk {
        /// Get the AMQP class id for rollback-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            90
        }

        /// Get the AMQP method id for rollback-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            31
        }
    }

    /// Parse rollback-ok (Generated)
    pub fn parse_rollback_ok<I: ParsableInput>(i: I) -> ParserResult<I, RollbackOk> {
        Ok((i, RollbackOk {}))
    }

    /// Serialize rollback-ok (Generated)
    pub fn gen_rollback_ok<'a, W: Write + BackToTheBuffer + 'a>(
        _: &'a RollbackOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(31)(input)?;
            Ok(input)
        }
    }
}
/// confirm (generated)
pub mod confirm {
    use super::*;

    /// Parse confirm (Generated)
    pub fn parse_confirm<I: ParsableInput>(i: I) -> ParserResult<I, confirm::AMQPMethod> {
        context(
            "parse_confirm",
            map_opt(
                flat_map(parse_id, |id| {
                    move |i| match id {
                        10 => context(
                            "parse_select",
                            map(map(parse_select, AMQPMethod::Select), Some),
                        )(i),
                        11 => context(
                            "parse_select_ok",
                            map(map(parse_select_ok, AMQPMethod::SelectOk), Some),
                        )(i),
                        _ => Ok((i, None)),
                    }
                }),
                std::convert::identity,
            ),
        )(i)
    }

    /// Serialize confirm (Generated)
    pub fn gen_confirm<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a AMQPMethod,
    ) -> impl SerializeFn<W> + 'a {
        cookie_factory::sequence::pair(gen_id(85), move |input| match *method {
            AMQPMethod::Select(ref select) => gen_select(select)(input),
            AMQPMethod::SelectOk(ref select_ok) => gen_select_ok(select_ok)(input),
        })
    }

    /// The available methods in confirm
    #[derive(Clone, Debug, PartialEq)]
    pub enum AMQPMethod {
        /// select (Generated)
        Select(Select),
        /// select-ok (Generated)
        SelectOk(SelectOk),
    }

    /// select (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Select {
        /// nowait (Generated)
        pub nowait: Boolean,
    }

    impl Select {
        /// Get the AMQP class id for select (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            85
        }

        /// Get the AMQP method id for select (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            10
        }
    }

    /// Parse select (Generated)
    pub fn parse_select<I: ParsableInput>(i: I) -> ParserResult<I, Select> {
        let (i, flags) = parse_flags(i, &["nowait"])?;
        Ok((
            i,
            Select {
                nowait: flags.get_flag("nowait").unwrap_or(false),
            },
        ))
    }

    /// Serialize select (Generated)
    pub fn gen_select<'a, W: Write + BackToTheBuffer + 'a>(
        method: &'a Select,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            let mut flags = AMQPFlags::default();
            flags.add_flag("nowait".to_string(), method.nowait);
            input = gen_id(10)(input)?;
            input = gen_flags(&flags)(input)?;
            Ok(input)
        }
    }
    /// select-ok (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct SelectOk {}

    impl SelectOk {
        /// Get the AMQP class id for select-ok (Generated)
        pub fn get_amqp_class_id(&self) -> Identifier {
            85
        }

        /// Get the AMQP method id for select-ok (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            11
        }
    }

    /// Parse select-ok (Generated)
    pub fn parse_select_ok<I: ParsableInput>(i: I) -> ParserResult<I, SelectOk> {
        Ok((i, SelectOk {}))
    }

    /// Serialize select-ok (Generated)
    pub fn gen_select_ok<'a, W: Write + BackToTheBuffer + 'a>(
        _: &'a SelectOk,
    ) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            input = gen_id(11)(input)?;
            Ok(input)
        }
    }
}
