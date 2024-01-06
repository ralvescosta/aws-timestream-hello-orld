mod configs;
mod migrator;
mod reader;
mod write;

use std::error::Error;

use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    info!("staging application...");

    info!("reading aws cli configurations...");
    let aws_sdk_configs = aws_config::load_from_env().await;
    let app_configs = configs::AppConfigs::default();
    info!("aws configurations read successfully");

    let migrator = migrator::Migrator::new(&aws_sdk_configs, &app_configs)
        .await
        .unwrap();

    let writable = write::Writable::new(&aws_sdk_configs, &app_configs)
        .await
        .unwrap();

    let reader = reader::Reader::new(&aws_sdk_configs, &app_configs)
        .await
        .unwrap();

    info!("migrating...");
    migrator.up().await.unwrap();
    info!("migrated!");

    info!("writing...");
    writable.write().await.unwrap();
    info!("written successfully!");

    info!("reading...");
    reader.read().await.unwrap();
    info!("read successfully!");

    info!("application finished successful!");
    Ok(())
}
