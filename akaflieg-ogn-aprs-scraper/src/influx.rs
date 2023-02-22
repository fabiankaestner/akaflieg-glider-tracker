use std::sync::Arc;

use tokio::sync::mpsc::Receiver;
use tokio_stream::Stream;

use influxdb2::models::DataPoint;
use influxdb2::Client;

use log::{error, warn, info};

use ogn_aprs_parser::model::ogn_status_message::OGNStatusMessage;

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
    let mut builder = DataPoint::builder("aprs").field("message", aprs_msg.clone());
    let parsed =OGNStatusMessage::from_str(&aprs_msg, None);
    if let Ok(p) = parsed  {
        builder = builder.field("timestamp", p.timestamp.timestamp());
        builder = builder
            .tag("aprs_callsign", p.aprs_callsign)
            .tag("aprs_path", p.aprs_path)
            .tag("aprs_type", format!("{:?}", p.aprs_type))
            .tag("aircraft_type", format!("{:?}", p.aircraft_type))
            .tag("parsed", "true");
        builder = builder
            .field("lat", p.position.latitude as f64)
            .field("long", p.position.longitude as f64)
            .field("heading", p.position.heading as f64)
            .field("altitude", p.position.altitude as f64);
        builder = builder.field("vel_horiz", p.velocity.horizontal as f64);
        if let Some(vel) = p.velocity.vertical {
            builder = builder.field("vel_vert", vel as f64);
        }
        if let Some(rot) = p.velocity.rotation {
            builder = builder.field("rot", rot as f64);
        }
        if let Some(id) = p.aircraft_id {
            builder = builder.tag("aircraft_id", id);
        }
        if let Some(flags) = p.ogn_flags {
            builder = builder
                .tag("stealth", flags.stealth_mode.to_string())
                .tag("no_track", flags.no_tracking_mode.to_string());
        }
        if let Some(addr_t) = p.address_type {
            builder = builder.tag("addr_type", format!("{:?}", addr_t));
        }
    } else {
        println!("FAILED PARSE: {}", aprs_msg);
        info!("Parse failed.");
        println!("{:?}", parsed);
        builder = builder.tag("parsed", "false")
    }

    let dp = match builder.build() {
        Ok(dp) => dp,
        Err(err) => {
            error!(
                "error creating datapoint from aprs message <{:?}>: {:?}",
                aprs_msg, err
            );
            return None;
        }
    };

    Some(tokio_stream::once(dp))
}
