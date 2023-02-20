use std::sync::Arc;

use tokio::sync::mpsc::Receiver;
use tokio_stream::Stream;

use influxdb2::models::DataPoint;
use influxdb2::Client;

use log::{error, warn};

pub async fn write_aprs(client: Arc<Client>, aprs_bucket: &str, mut rx: Receiver<String>) {
    // loop while channel is still alive
    while let Some(aprs_msg) = rx.recv().await {
        // transform received string to influx datapoint
        let transformed = match transform_aprs(aprs_msg) {
            Some(datapoint) => datapoint,
            None => continue,
        };

        // write the datapoint to db, log any errors but continue anyway.
        match client.write(aprs_bucket, transformed).await {
            Ok(_) => continue,
            Err(err) => {
                error!("error writing to influx: {:?}", err);
                continue;
            }
        }
    }

    // this should really only happen when quitting the program.
    warn! {"quit influx aprs write loop"}
}

pub fn transform_aprs(aprs_msg: String) -> Option<impl Stream<Item = DataPoint>> {
    let datapoint = match DataPoint::builder("aprs")
        .field("message", aprs_msg.clone())
        .build()
    {
        Ok(dp) => dp,
        Err(err) => {
            error!(
                "error creating datapoint from aprs message <{:?}>: {:?}",
                aprs_msg, err
            );
            return None;
        }
    };
    Some(tokio_stream::once(datapoint))
}
