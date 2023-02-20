use std::sync::Arc;

use async_stream::stream;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio_stream::Stream;

use influxdb2::models::DataPoint;
use influxdb2::Client;

use log::{error,info};

pub async fn write_aprs(client: Arc<Client>, aprs_bucket: &str, tx: Sender<String>) {
    loop {
        match client
            .write(aprs_bucket, transform_aprs(tx.subscribe()))
            .await
        {
            Ok(_) => continue,
            Err(err) => {
                error!("error writing to influx: {:?}", err);
                continue;
            }
        }
    }
}

pub fn transform_aprs(mut rx: Receiver<String>) -> impl Stream<Item = DataPoint> {
    stream! {
        loop {
            let aprs_msg = rx.recv().await;
            info!("got message");
            let aprs_str = match aprs_msg {
                Ok(aprs_msg) => aprs_msg,
                Err(err) => {error!("receive error in APRS channel, messages might have been missed: {:?}", err); continue;}
            };
            let datapoint = match DataPoint::builder("aprs").field("message", aprs_str.clone()).build() {
                Ok(dp) => dp,
                Err(err) => {error!("error creating datapoint from aprs message <{:?}>: {:?}", aprs_str, err); continue;}
            };
            yield datapoint
        }
    }
}
