// Copyright (c) 2025, Ibrahim Erturk <me@erturk.me>, ErturkMe
// Licensed under the BSD 3-Clause License.
// See the LICENSE file in the project root for more information.

use dust_dds::topic_definition::type_support::DdsType;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const RBIC1_DOMAIN_ID: i32 = 1;
pub const RBIC1_REQUEST_TOPIC_NAME: &str = "RBIC1RequestTopic";
pub const RBIC1_REQUEST_TYPE_NAME: &str = "RBIC1RequestType";
pub const RBIC1_REPLY_TOPIC_NAME: &str = "RBIC1ReplyTopic";
pub const RBIC1_REPLY_TYPE_NAME: &str = "RBIC1ReplyType";

#[derive(Error, Debug)]
pub enum RBIC1ServiceErrorType {
    #[error("Application failed to start")]
    AppFailedToStart,
    #[error("Operation failed")]
    OperationFailed,
    #[error("Invoker failed")]
    InvokerFailed,
    #[error("Proxy failed")]
    ProxyFailed,
    #[error("NoSubscribers")]
    NoSubscribers,
    #[error("UnknownRequest")]
    UnknownRequest,
    #[error("UnknownReply")]
    UnknownReply,
    #[error("DeserializationError")]
    DeserializationError,
    #[error("DDSError")]
    DDSError,
    #[error("DDSTimeout")]
    DDSTimeout,
}

#[derive(DdsType, Debug)]
pub struct DataContainerType {
    pub correlation_id: [u8; 16],
    pub timestamp: i64,
    pub payload: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PayLoad {
    Dummy,
    Request(RequestType),
    Reply(ReplyType),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RequestType {
    OutLin {
        zp1m: f32,
        zp2m: f32,
        a: f32,
        b: f32,
    },
    TLin {
        ztmed: f32,
        ztupp: f32,
        tmed: f32,
        tupp: f32,
    },
    OutQuad {
        zp1m: f32,
        zp2m: f32,
        zp3m: f32,
        a: f32,
        b: f32,
        m: f32,
        adc_reso: i32,
    },
    OutThird {
        zp1m: f32,
        zp2m: f32,
        zp3m: f32,
        zp4m: f32,
        a: f32,
        b: f32,
        m: f32,
        m2: f32,
        adc_reso: i32,
    },
    TQuad {
        ztlow: f32,
        ztupp: f32,
        ztmed: f32,
        tlow: f32,
        tupp: f32,
        tmed: f32,
        adc_reso: i32,
    },
    OutLinTLin {
        zp1m: f32,
        zp2m: f32,
        zp1u: f32,
        zp2u: f32,
        a: f32,
        b: f32,
        ztmed: f32,
        ztupp: f32,
        adc_reso: i32,
    },
    OutQuadTLin {
        zp1m: f32,
        zp2m: f32,
        zp3m: f32,
        zp1u: f32,
        zp2u: f32,
        a: f32,
        b: f32,
        m: f32,
        ztmed: f32,
        ztupp: f32,
        adc_reso: i32,
    },
    OutLinTQuad {
        zp1m: f32,
        zp2m: f32,
        zp1u: f32,
        zp2u: f32,
        zp1l: f32,
        zp2l: f32,
        a: f32,
        b: f32,
        ztmed: f32,
        ztupp: f32,
        ztlow: f32,
        adc_reso: i32,
    },
    OutQuadTQuad {
        zp1m: f32,
        zp2m: f32,
        zp3m: f32,
        zp1u: f32,
        zp2u: f32,
        zp1l: f32,
        zp2l: f32,
        a: f32,
        b: f32,
        m: f32,
        ztmed: f32,
        ztupp: f32,
        ztlow: f32,
        adc_reso: i32,
    },
    ZMD31050Cal1 {
        zp1m: f32,
        zp2m: f32,
        zp3m: f32,
        zp4m: f32,
        zp1u: f32,
        zp2u: f32,
        zp1l: f32,
        zp2l: f32,
        a: f32,
        b: f32,
        m: f32,
        m2: f32,
        ztmed: f32,
        ztupp: f32,
        ztlow: f32,
        adc_reso: i32,
    },
    ZMD31050Sim1 {
        c0: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
        c5: i32,
        c6: i32,
        c7: i32,
        adc_reso: i32,
        range_shift: f32,
        izmin: i32,
        izmax: i32,
        zt: i32,
        ztmin: i32,
        ztmax: i32,
    },
    GetMessage,
    DLLVersion,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ReplyType {
    OutLin {
        c0: f32,
        c1: f32,
        ret: bool,
    },
    TLin {
        ct0: f32,
        ct1: f32,
        ret: bool,
    },
    OutQuad {
        c0: f32,
        c1: f32,
        c2: f32,
        ret: bool,
    },
    OutThird {
        c0: f32,
        c1: f32,
        c2: f32,
        c3: f32,
        ret: bool,
    },
    TQuad {
        ct0: f32,
        ct1: f32,
        ct2: f32,
        ret: bool,
    },
    OutLinTLin {
        c0: f32,
        c1: f32,
        c4: f32,
        c6: f32,
        ret: bool,
    },
    OutQuadTLin {
        c0: f32,
        c1: f32,
        c2: f32,
        c4: f32,
        c6: f32,
        ret: bool,
    },
    OutLinTQuad {
        c0: f32,
        c1: f32,
        c4: f32,
        c6: f32,
        c5: f32,
        c7: f32,
        ret: bool,
    },
    OutQuadTQuad {
        c0: f32,
        c1: f32,
        c2: f32,
        c4: f32,
        c6: f32,
        c5: f32,
        c7: f32,
        ret: bool,
    },
    ZMD31050Cal1 {
        c0: f32,
        c1: f32,
        c2: f32,
        c3: f32,
        c4: f32,
        c6: f32,
        c5: f32,
        c7: f32,
        ret: i32,
    },
    Zmd31050Sim1 {
        ret: i32,
    },
    GetMessage {
        message: String,
    },
    DLLVersion {
        version: String,
    },
}
