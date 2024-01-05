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

    info!("reading aws configurations...");
    let aws_sdk_configs = aws_config::load_from_env().await;
    let app_configs = configs::AppConfigs::default();

    let writable = write::Writable::new(&aws_sdk_configs, &app_configs)
        .await
        .unwrap();

    let reader = reader::Reader::new(&aws_sdk_configs, &app_configs)
        .await
        .unwrap();

    info!("writing...");
    writable.write().await.unwrap();
    info!("wrote!");

    info!("reading...");
    reader.read().await.unwrap();
    info!("read!");

    info!("application finished successful!");
    Ok(())
}
