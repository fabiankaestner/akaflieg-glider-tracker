-- Add up migration script here
CREATE TABLE
  public.tracks (
    "timestamp" timestamp with time zone NOT NULL,
    aircraft_id text NULL,
    aprs_callsign text NOT NULL,
    aprs_path text NOT NULL,
    aprs_type integer NOT NULL,
    "position" geography NOT NULL,
    velocity_horizontal real NOT NULL,
    velocity_vertical real NULL,
    velocity_rotation real NULL,
    aircraft_type integer NOT NULL,
    address_type integer NULL,
    ogn_flag_stealth_mode boolean NULL,
    ogn_flag_no_tracking_mode boolean NULL,
    raw_string text NOT NULL
  );
