use crate::configs::AppConfigs;
use aws_config::SdkConfig;
use aws_sdk_timestreamwrite::{
    types::{MagneticStoreWriteProperties, RetentionProperties},
    Client,
};
use log::{debug, error};

pub struct Migrator<'m> {
    client: Client,
    app_configs: &'m AppConfigs,
}

impl<'m> Migrator<'m> {
    pub async fn new(aws_sdk_config: &SdkConfig, app_configs: &'m AppConfigs) -> Result<Self, ()> {
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

impl<'m> Migrator<'m> {
    pub async fn up(&self) -> Result<(), ()> {
        match self
            .client
            .create_database()
            .set_database_name(Some(self.app_configs.database.clone()))
            .send()
            .await
        {
            Ok(_out) => {
                debug!("database created!");
                Ok(())
            }
            Err(err) => {
                error!("failure to create database");
                error!("{}", err);
                Err(())
            }
        }?;

        match self
            .client
            .create_table()
            .set_database_name(Some(self.app_configs.database.clone()))
            .set_table_name(Some(self.app_configs.table.clone()))
            .set_magnetic_store_write_properties(Some(
                MagneticStoreWriteProperties::builder()
                    .set_enable_magnetic_store_writes(Some(true))
                    .build()
                    .unwrap(),
            ))
            .set_retention_properties(Some(
                RetentionProperties::builder()
                    .set_magnetic_store_retention_period_in_days(Some(360))
                    .set_memory_store_retention_period_in_hours(Some(24))
                    .build()
                    .unwrap(),
            ))
            .send()
            .await
        {
            Ok(_out) => {
                debug!("table created!");
                Ok(())
            }
            Err(err) => {
                error!("failure to create table");
                error!("{:?}", err);

                Err(())
            }
        }
    }
}
