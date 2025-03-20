// Copyright (c) 2025, Ibrahim Erturk <me@erturk.me>, ErturkMe
// Licensed under the BSD 3-Clause License.
// See the LICENSE file in the project root for more information.

use dust_dds::{
    dds_async::wait_set::{ConditionAsync, WaitSetAsync},
    infrastructure::{status::StatusKind, time::Duration},
    subscription::sample_info::{ANY_INSTANCE_STATE, ANY_SAMPLE_STATE, ANY_VIEW_STATE},
};

use chrono::offset::Utc;

use crate::dds_rpc_service::DdsRpcService;
use crate::rbic1_wrapper::*;
use common_lib::types::*;

pub struct Proxy {
    writer: dust_dds::dds_async::data_writer::DataWriterAsync<DataContainerType>,
    reader: dust_dds::dds_async::data_reader::DataReaderAsync<DataContainerType>,
}

impl DdsRpcService for Proxy {
    async fn new() -> Self
    where
        Self: Sized,
    {
        let (writer, reader) = Self::create_writer_and_reader(
            RBIC1_DOMAIN_ID,
            RBIC1_REPLY_TOPIC_NAME,
            RBIC1_REPLY_TYPE_NAME,
            RBIC1_REQUEST_TOPIC_NAME,
            RBIC1_REQUEST_TYPE_NAME,
        )
        .await;

        Self { writer, reader }
    }
}

impl Drop for Proxy {
    fn drop(&mut self) {
        println!("Proxy is being dropped. Cleaning up resources...");
    }
}

impl Proxy {
    pub async fn run(&self) -> Result<(), RBIC1ServiceErrorType> {
        let cond = self.reader.get_statuscondition();
        cond.set_enabled_statuses(&[StatusKind::DataAvailable])
            .await
            .unwrap();
        let mut reader_wait_set = WaitSetAsync::new();
        reader_wait_set
            .attach_condition(ConditionAsync::StatusCondition(cond))
            .await
            .unwrap();

        loop {
            match reader_wait_set.wait(Duration::new(10, 0)).await {
                Ok(_) => {
                    match self
                        .reader
                        .take(1, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE)
                        .await
                    {
                        Ok(samples) => {
                            if !samples.is_empty() {
                                match samples[0].data() {
                                    Ok(data) => {
                                        match serde_json::from_str(&data.payload).unwrap() {
                                            PayLoad::Request(request) => {
                                                match get_reply_from_request(request).unwrap() {
                                                    ReplyType::DLLVersion { version } => {
                                                        let reply_data = DataContainerType {
                                                            correlation_id: data.correlation_id,
                                                            timestamp: Utc::now().timestamp(),
                                                            payload: serde_json::to_string(
                                                                &PayLoad::Reply(
                                                                    ReplyType::DLLVersion {
                                                                        version,
                                                                    },
                                                                ),
                                                            )
                                                            .unwrap(),
                                                        };

                                                        _ = match dbg!(
                                                            self.writer
                                                                .write(&reply_data, None)
                                                                .await
                                                        ) {
                                                            Ok(_) => {
                                                                // println!(
                                                                //     "Replied with: {:?}",
                                                                //     reply_data
                                                                // );
                                                            }
                                                            Err(e) => {
                                                                eprintln!(
                                                                    "Failed to write: {:?}",
                                                                    e
                                                                );
                                                                return Err(RBIC1ServiceErrorType::ProxyFailed);
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        return Err(
                                                            RBIC1ServiceErrorType::UnknownRequest,
                                                        );
                                                    }
                                                }
                                            }
                                            _ => {
                                                return Err(
                                                    RBIC1ServiceErrorType::DeserializationError,
                                                );
                                            }
                                        }
                                    }
                                    Err(_) => {
                                        return Err(RBIC1ServiceErrorType::DDSError);
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            return Err(RBIC1ServiceErrorType::DDSTimeout);
                        }
                    }
                }
                Err(_) => {
                    println!("Timeout waiting for data");
                    // return Err(RBIC1ServiceErrorType::DDSTimeout);
                }
            }
        }
    }
}

pub fn get_reply_from_request(request: RequestType) -> Result<ReplyType, RBIC1ServiceErrorType> {
    match request {
        RequestType::OutLin { zp1m, zp2m, a, b } => {
            let (c0, c1, ret) = unsafe { out_lin(zp1m, zp2m, a, b).unwrap() };

            Ok(ReplyType::OutLin { c0, c1, ret })
        }

        RequestType::TLin {
            ztmed,
            ztupp,
            tmed,
            tupp,
        } => {
            let (ct0, ct1, ret) = unsafe { t_lin(ztmed, ztupp, tmed, tupp).unwrap() };

            Ok(ReplyType::TLin { ct0, ct1, ret })
        }

        RequestType::OutQuad {
            zp1m,
            zp2m,
            zp3m,
            a,
            b,
            m,
            adc_reso,
        } => {
            let (c0, c1, c2, ret) =
                unsafe { out_quad(zp1m, zp2m, zp3m, a, b, m, adc_reso).unwrap() };

            Ok(ReplyType::OutQuad { c0, c1, c2, ret })
        }

        RequestType::OutThird {
            zp1m,
            zp2m,
            zp3m,
            zp4m,
            a,
            b,
            m,
            m2,
            adc_reso,
        } => {
            let (c0, c1, c2, c3, ret) =
                unsafe { out_third(zp1m, zp2m, zp3m, zp4m, a, b, m, m2, adc_reso).unwrap() };

            Ok(ReplyType::OutThird {
                c0,
                c1,
                c2,
                c3,
                ret,
            })
        }

        RequestType::TQuad {
            ztlow,
            ztupp,
            ztmed,
            tlow,
            tupp,
            tmed,
            adc_reso,
        } => {
            let (ct0, ct1, ct2, ret) =
                unsafe { t_quad(ztlow, ztupp, ztmed, tlow, tupp, tmed, adc_reso).unwrap() };

            Ok(ReplyType::TQuad { ct0, ct1, ct2, ret })
        }

        RequestType::OutLinTLin {
            zp1m,
            zp2m,
            zp1u,
            zp2u,
            a,
            b,
            ztmed,
            ztupp,
            adc_reso,
        } => {
            let (c0, c1, c4, c6, ret) = unsafe {
                out_lin_t_lin(zp1m, zp2m, zp1u, zp2u, a, b, ztmed, ztupp, adc_reso).unwrap()
            };

            Ok(ReplyType::OutLinTLin {
                c0,
                c1,
                c4,
                c6,
                ret,
            })
        }

        RequestType::OutQuadTLin {
            zp1m,
            zp2m,
            zp3m,
            zp1u,
            zp2u,
            a,
            b,
            m,
            ztmed,
            ztupp,
            adc_reso,
        } => {
            let (c0, c1, c2, c4, c6, ret) = unsafe {
                out_quad_t_lin(
                    zp1m, zp2m, zp3m, zp1u, zp2u, a, b, m, ztmed, ztupp, adc_reso,
                )
                .unwrap()
            };

            Ok(ReplyType::OutQuadTLin {
                c0,
                c1,
                c2,
                c4,
                c6,
                ret,
            })
        }

        RequestType::OutLinTQuad {
            zp1m,
            zp2m,
            zp1u,
            zp2u,
            zp1l,
            zp2l,
            a,
            b,
            ztmed,
            ztupp,
            ztlow,
            adc_reso,
        } => {
            let (c0, c1, c4, c6, c5, c7, ret) = unsafe {
                out_lin_t_quad(
                    zp1m, zp2m, zp1u, zp2u, zp1l, zp2l, a, b, ztmed, ztupp, ztlow, adc_reso,
                )
                .unwrap()
            };

            Ok(ReplyType::OutLinTQuad {
                c0,
                c1,
                c4,
                c6,
                c5,
                c7,
                ret,
            })
        }

        RequestType::OutQuadTQuad {
            zp1m,
            zp2m,
            zp3m,
            zp1u,
            zp2u,
            zp1l,
            zp2l,
            a,
            b,
            m,
            ztmed,
            ztupp,
            ztlow,
            adc_reso,
        } => {
            let (c0, c1, c2, c4, c6, c5, c7, ret) = unsafe {
                out_quad_t_quad(
                    zp1m, zp2m, zp3m, zp1u, zp2u, zp1l, zp2l, a, b, m, ztmed, ztupp, ztlow,
                    adc_reso,
                )
                .unwrap()
            };

            Ok(ReplyType::OutQuadTQuad {
                c0,
                c1,
                c2,
                c4,
                c6,
                c5,
                c7,
                ret,
            })
        }

        RequestType::ZMD31050Cal1 {
            zp1m,
            zp2m,
            zp3m,
            zp4m,
            zp1u,
            zp2u,
            zp1l,
            zp2l,
            a,
            b,
            m,
            m2,
            ztmed,
            ztupp,
            ztlow,
            adc_reso,
        } => {
            let (c0, c1, c2, c3, c4, c5, c6, c7, ret) = unsafe {
                zmd31050_cal1(
                    zp1m, zp2m, zp3m, zp4m, zp1u, zp2u, zp1l, zp2l, a, b, m, m2, ztmed, ztupp,
                    ztlow, adc_reso,
                )
                .unwrap()
            };

            Ok(ReplyType::ZMD31050Cal1 {
                c0,
                c1,
                c2,
                c3,
                c4,
                c5,
                c6,
                c7,
                ret,
            })
        }

        RequestType::ZMD31050Sim1 {
            c0,
            c1,
            c2,
            c3,
            c4,
            c5,
            c6,
            c7,
            adc_reso,
            range_shift,
            izmin,
            izmax,
            zt,
            ztmin,
            ztmax,
        } => Ok(ReplyType::Zmd31050Sim1 {
            ret: unsafe {
                zmd31050_sim1(
                    c0,
                    c1,
                    c2,
                    c3,
                    c4,
                    c5,
                    c6,
                    c7,
                    adc_reso,
                    range_shift,
                    izmin,
                    izmax,
                    zt,
                    ztmin,
                    ztmax,
                )
                .unwrap()
            },
        }),

        RequestType::DLLVersion => Ok(ReplyType::DLLVersion {
            version: unsafe { dll_version().unwrap() },
        }),

        RequestType::GetMessage => Ok(ReplyType::GetMessage {
            message: unsafe { get_message().unwrap() },
        }),
    }
}
