use log::debug;
use ogn_aprs_parser::model::ogn_status_message::OGNStatusMessage;
use sqlx::{Pool, Postgres};
use tokio::sync::mpsc::Receiver;

pub async fn write_aprs(pool: Pool<Postgres>, mut rx: Receiver<String>) {
    // loop while channel is still alive
    while let Some(aprs_msg) = rx.recv().await {
        let parsed = OGNStatusMessage::from_str(&aprs_msg, None);
        match parsed {
            Ok(msg) => {
                debug!("inserting {:?}", msg);
                let (stealth_mode, no_tracking_mode) = if let Some(flags) = msg.ogn_flags {
                    (Some(flags.stealth_mode), Some(flags.no_tracking_mode))
                } else {
                    (None, None)
                };
                // Insert geometry
                let _ = sqlx::query!(
                    r#"
                    INSERT INTO tracks (
                        timestamp, aircraft_id, aprs_callsign, aprs_path, aprs_type,
                        position, velocity_horizontal, velocity_vertical, velocity_rotation,
                        aircraft_type, ogn_flag_stealth_mode, ogn_flag_no_tracking_mode,
                        address_type, raw_string
                    )
                    VALUES($1, $2, $3, $4, $5,
                      ST_GeogFromText('SRID=4326;POINT(' || $6 || ' ' || $7 || ' ' || $8 || ' ' || $9 || ')'),
                      $10, $11, $12,
                      $13, $14, $15, $16, $17
                    )
                    "#,
                    msg.timestamp,
                    msg.aircraft_id,
                    msg.aprs_callsign,
                    msg.aprs_path,
                    Some(msg.aprs_type as i32),
                    msg.position.longitude.to_string(),
                    msg.position.latitude.to_string(),
                    msg.position.altitude.to_string(),
                    msg.position.heading.to_string(),
                    msg.velocity.horizontal,
                    msg.velocity.vertical,
                    msg.velocity.rotation,
                    Some(msg.aircraft_type as i32),
                    stealth_mode,
                    no_tracking_mode,
                    msg.address_type.map_or(None, |f| Some(f as i32)),
                    aprs_msg
                )
                .execute(&pool)
                .await;
            }
            Err(_) => {}
        }
        // write the datapoint to db, log any errors but continue anyway.
    }

    // this should really only happen when quitting the program.
    println!("quitting write loop")
}
