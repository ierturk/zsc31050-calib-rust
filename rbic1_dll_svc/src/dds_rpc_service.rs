// Copyright (c) 2025, Ibrahim Erturk <me@erturk.me>, ErturkMe
// Licensed under the BSD 3-Clause License.
// See the LICENSE file in the project root for more information.

use common_lib::types::*;

use dust_dds::{
    dds_async::{
        data_reader::DataReaderAsync, data_writer::DataWriterAsync,
        domain_participant_factory::DomainParticipantFactoryAsync,
    },
    domain::domain_participant_factory::DomainId,
    infrastructure::{
        qos::{DataReaderQos, DataWriterQos, QosKind},
        qos_policy::{ReliabilityQosPolicy, ReliabilityQosPolicyKind},
        status::NO_STATUS,
        time::{Duration, DurationKind},
    },
};

pub trait DdsRpcService {
    fn new() -> impl std::future::Future<Output = Self> + Send
    where
        Self: Sized;

    fn create_writer_and_reader(
        domain_id: DomainId,
        writer_name: &str,
        writer_type: &str,
        reader_name: &str,
        reader_type: &str,
    ) -> impl std::future::Future<
        Output = (
            DataWriterAsync<DataContainerType>,
            DataReaderAsync<DataContainerType>,
        ),
    > + Send {
        async move {
            let participant_factory = DomainParticipantFactoryAsync::get_instance();
            let participant = participant_factory
                .create_participant(domain_id, QosKind::Default, None, NO_STATUS)
                .await
                .unwrap();

            let writer_topic = participant
                .create_topic::<DataContainerType>(
                    writer_name,
                    writer_type,
                    QosKind::Default,
                    None,
                    NO_STATUS,
                )
                .await
                .unwrap();

            let reader_topic = participant
                .create_topic::<DataContainerType>(
                    reader_name,
                    reader_type,
                    QosKind::Default,
                    None,
                    NO_STATUS,
                )
                .await
                .unwrap();

            let publisher = participant
                .create_publisher(QosKind::Default, None, NO_STATUS)
                .await
                .unwrap();

            let writer_qos = DataWriterQos {
                reliability: ReliabilityQosPolicy {
                    kind: ReliabilityQosPolicyKind::Reliable,
                    max_blocking_time: DurationKind::Finite(Duration::new(1, 0)),
                },
                ..Default::default()
            };

            let writer = publisher
                .create_datawriter(
                    &writer_topic,
                    QosKind::Specific(writer_qos),
                    None,
                    NO_STATUS,
                )
                .await
                .unwrap();

            let subscriber = participant
                .create_subscriber(QosKind::Default, None, NO_STATUS)
                .await
                .unwrap();

            let reader_qos = DataReaderQos {
                reliability: ReliabilityQosPolicy {
                    kind: ReliabilityQosPolicyKind::Reliable,
                    max_blocking_time: DurationKind::Finite(Duration::new(1, 0)),
                },
                ..Default::default()
            };

            let reader = subscriber
                .create_datareader::<DataContainerType>(
                    &reader_topic,
                    QosKind::Specific(reader_qos),
                    None,
                    NO_STATUS,
                )
                .await
                .unwrap();

            (writer, reader)
        }
    }
}
