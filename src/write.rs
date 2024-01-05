use crate::configs::AppConfigs;
use aws_config::SdkConfig;
use aws_sdk_timestreamwrite::{
    client,
    types::{Dimension, DimensionValueType, MeasureValue, MeasureValueType, Record, TimeUnit},
};
use log::{debug, error};
use std::vec;

pub struct Writable<'w> {
    client: client::Client,
    app_configs: &'w AppConfigs,
}

impl<'w> Writable<'w> {
    pub async fn new(aws_sdk_config: &SdkConfig, app_configs: &'w AppConfigs) -> Result<Self, ()> {
        debug!("creating write client...");

        match aws_sdk_timestreamwrite::Client::new(aws_sdk_config)
            .with_endpoint_discovery_enabled()
            .await
        {
            Ok((client, _endpoint)) => {
                debug!("write cliente created successfully!");

                Ok(Self {
                    client,
                    app_configs,
                })
            }
            Err(err) => {
                error!("something went wrong to create the write client",);
                error!("{:?}", err);

                Err(())
            }
        }
    }
}

impl<'w> Writable<'w> {
    pub async fn write(&self) -> Result<(), ()> {
        let record = Record::builder()
            .set_measure_name(Some(self.record_measure_name()))
            .set_measure_values(Some(self.measures()))
            .set_dimensions(Some(self.dimensions()))
            .set_measure_value_type(Some(MeasureValueType::Multi))
            .set_time(Some("1704463487395".into()))
            .set_time_unit(Some(TimeUnit::Milliseconds))
            .build();

        match self
            .client
            .write_records()
            .set_records(Some(vec![record]))
            .set_database_name(Some(self.app_configs.database.clone()))
            .set_table_name(Some(self.app_configs.table.clone()))
            .send()
            .await
        {
            Ok(_s) => Ok(()),
            Err(err) => {
                error!("failure to write messages");
                error!("{:?}", err);

                Err(())
            }
        }
    }

    fn record_measure_name(&self) -> String {
        "retrieve-key".into()
    }

    fn measures(&self) -> Vec<MeasureValue> {
        let name = MeasureValue::builder()
            .set_name(Some("name".into()))
            .set_type(Some(MeasureValueType::Varchar))
            .set_value(Some("configured_name".into()))
            .build()
            .unwrap();

        let signal = MeasureValue::builder()
            .set_name(Some("signal".into()))
            .set_type(Some(MeasureValueType::Double))
            .set_value(Some("-30".into()))
            .build()
            .unwrap();

        let data_type = MeasureValue::builder()
            .set_name(Some("data_type".into()))
            .set_type(Some(MeasureValueType::Bigint))
            .set_value(Some("3".into()))
            .build()
            .unwrap();

        let acquisition_frequency = MeasureValue::builder()
            .set_name(Some("acquisition_frequency".into()))
            .set_type(Some(MeasureValueType::Bigint))
            .set_value(Some("3".into()))
            .build()
            .unwrap();

        let data = MeasureValue::builder()
            .set_name(Some("data".into()))
            .set_type(Some(MeasureValueType::Varchar))
            .set_value(Some("xxxxxx".into()))
            .build()
            .unwrap();

        vec![name, signal, data_type, acquisition_frequency, data]
    }

    fn dimensions(&self) -> Vec<Dimension> {
        let organization = Dimension::builder()
            .set_name(Some("organization".into()))
            .set_value(Some("xxxxxx".into()))
            .set_dimension_value_type(Some(DimensionValueType::Varchar))
            .build()
            .unwrap();

        let device_id = Dimension::builder()
            .set_name(Some("device_id".into()))
            .set_value(Some("00000001".into()))
            .set_dimension_value_type(Some(DimensionValueType::Varchar))
            .build()
            .unwrap();

        let hardware_version = Dimension::builder()
            .set_name(Some("hardware_version".into()))
            .set_value(Some("0.0.0".into()))
            .set_dimension_value_type(Some(DimensionValueType::Varchar))
            .build()
            .unwrap();

        let firmware_version = Dimension::builder()
            .set_name(Some("firmware_version".into()))
            .set_value(Some("0.0.0".into()))
            .set_dimension_value_type(Some(DimensionValueType::Varchar))
            .build()
            .unwrap();

        vec![organization, device_id, hardware_version, firmware_version]
    }
}
