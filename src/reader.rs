use crate::configs::AppConfigs;
use aws_config::SdkConfig;
use aws_sdk_timestreamquery::client::Client;
use log::{debug, error};

pub struct Reader<'r> {
    client: Client,
    app_configs: &'r AppConfigs,
}

impl<'r> Reader<'r> {
    pub async fn new(aws_sdk_configs: &SdkConfig, app_configs: &'r AppConfigs) -> Result<Self, ()> {
        debug!("creating read client...");
        // You MUST call `with_endpoint_discovery_enabled` to produce a working client for this service.
        match aws_sdk_timestreamquery::Client::new(&aws_sdk_configs)
            .with_endpoint_discovery_enabled()
            .await
        {
            Ok((client, _endpoint)) => {
                debug!("read cliente created successfully!");

                Ok(Self {
                    client,
                    app_configs,
                })
            }
            Err(_err) => {
                //
                Err(())
            }
        }
    }
}

impl<'r> Reader<'r> {
    pub async fn read(&self) -> Result<(), ()> {
        debug!("preparing query statement...");

        let prepare = match self
            .client
            .prepare_query()
            .set_query_string(Some(format!(
                "SELECT * FROM \"{}\".{} ORDER BY time asc",
                self.app_configs.database, self.app_configs.table,
            )))
            .validate_only(true)
            .send()
            .await
        {
            Ok(p) => {
                debug!("query prepared successfully!");

                Ok(p)
            }
            Err(err) => {
                error!("error to prepare query - {:?}", err);

                Err(())
            }
        }?;

        match self
            .client
            .query()
            .set_query_string(Some(prepare.query_string))
            .send()
            .await
        {
            Ok(_r) => {
                //
                Ok(())
            }
            Err(_err) => {
                //
                Err(())
            }
        }
    }
}
