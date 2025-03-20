// Copyright (c) 2025, Ibrahim Erturk <me@erturk.me>, ErturkMe
// Licensed under the BSD 3-Clause License.
// See the LICENSE file in the project root for more information.

use dust_dds::{
    dds_async::wait_set::{ConditionAsync, WaitSetAsync},
    infrastructure::{status::StatusKind, time::Duration},
    subscription::sample_info::{ANY_INSTANCE_STATE, ANY_SAMPLE_STATE, ANY_VIEW_STATE},
};

use chrono::offset::Utc;
use uuid::Uuid;

use crate::dds_rpc_service::DdsRpcService;
use common_lib::types::*;
pub struct Invoker {
    writer: dust_dds::dds_async::data_writer::DataWriterAsync<DataContainerType>,
    reader: dust_dds::dds_async::data_reader::DataReaderAsync<DataContainerType>,
}

impl DdsRpcService for Invoker {
    async fn new() -> Self
    where
        Self: Sized,
    {
        let (writer, reader) = Self::create_writer_and_reader(
            RBIC1_DOMAIN_ID,
            RBIC1_REQUEST_TOPIC_NAME,
            RBIC1_REQUEST_TYPE_NAME,
            RBIC1_REPLY_TOPIC_NAME,
            RBIC1_REPLY_TYPE_NAME,
        )
        .await;

        Self { writer, reader }
    }
}

impl Invoker {
    pub async fn invoke(&self, request: RequestType) -> Result<ReplyType, RBIC1ServiceErrorType> {
        let request_data = DataContainerType {
            correlation_id: *Uuid::new_v4().as_bytes(),
            timestamp: Utc::now().timestamp(),
            payload: serde_json::to_string(&PayLoad::Request(request)).unwrap(),
        };

        let writer_cond = self.writer.get_statuscondition();
        writer_cond
            .set_enabled_statuses(&[StatusKind::PublicationMatched])
            .await
            .unwrap();

        let mut writer_wait_set = WaitSetAsync::new();
        writer_wait_set
            .attach_condition(ConditionAsync::StatusCondition(writer_cond))
            .await
            .unwrap();

        match writer_wait_set.wait(Duration::new(2, 0)).await {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error waiting for writer: {:?}", e);
                return Err(RBIC1ServiceErrorType::InvokerFailed);
            }
        }

        match self.writer.write(&request_data, None).await {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error writing request data: {:?}", e);
                return Err(RBIC1ServiceErrorType::InvokerFailed);
            }
        }

        let reader_cond = self.reader.get_statuscondition();
        reader_cond
            .set_enabled_statuses(&[StatusKind::DataAvailable])
            .await
            .unwrap();

        let mut reader_wait_set = WaitSetAsync::new();
        reader_wait_set
            .attach_condition(ConditionAsync::StatusCondition(reader_cond))
            .await
            .unwrap();

        match reader_wait_set.wait(Duration::new(2, 0)).await {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error waiting for reader: {:?}", e);
                return Err(RBIC1ServiceErrorType::InvokerFailed);
            }
        }

        match self
            .reader
            .take(1, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE)
            .await
        {
            Ok(samples) => {
                if samples.is_empty()
                    || request_data.correlation_id != samples[0].data().unwrap().correlation_id
                {
                    return Err(RBIC1ServiceErrorType::OperationFailed);
                }

                match serde_json::from_str(&samples[0].data().unwrap().payload) {
                    Ok(PayLoad::Reply(reply)) => return Ok(reply),
                    _ => return Err(RBIC1ServiceErrorType::UnknownReply),
                }
            }
            Err(e) => {
                eprintln!("Error taking reply data: {:?}", e);
                return Err(RBIC1ServiceErrorType::InvokerFailed);
            }
        }
    }
}
