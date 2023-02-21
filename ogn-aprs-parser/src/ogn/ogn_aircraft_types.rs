use crate::parser::position::ParsedSymbol;

pub enum AircraftType {
    Other,
    Glider,
    TowPlane,
    Helicopter,
    Parachute,
    DropPlane,
    HangGlider,
    ParaGlider,
    PoweredAircraft,
    JetAircraft,
    UFO,
    Balloon,
    Airship,
    UAV,
    GroundSupport,
    StaticObject,
    Unknown,
}

impl AircraftType {
    // from: http://wiki.glidernet.org/wiki:ogn-flavoured-aprs
    //
    // Hexadecimal value. Range: from 0 to F.
    // Aircraft types as assigned by FLARM:
    // 0 = (reserved)
    // 1 = glider/motor glider (turbo, self-launch, jet) / TMG
    // 2 = tow plane/tug plane
    // 3 = helicopter/gyrocopter/rotorcraft
    // 4 = skydiver, parachute (Do not use for drop plane!)
    // 5 = drop plane for skydivers
    // 6 = hang glider (hard)
    // 7 = paraglider (soft)
    // 8 = aircraft with reciprocating engine(s)
    // 9 = aircraft with jet/turboprop engine(s)
    // A = unknown
    // B = balloon (hot, gas, weather, static)
    // C = airship, blimp, zeppelin
    // D = unmanned aerial vehicle (UAV, RPAS, drone)
    // E = (reserved)
    // F = static obstacle
    //
    // bit layout: STttttaa
    //
    // S, T, tttt, aa stand for 8 bits from most to least significant.
    // tttt: FLARM Aircraft Type

    pub fn from_meta(meta: usize) -> Self {
        use AircraftType::*;
        let masked = (meta >> 2) & 0x0F;
        match masked {
            0 => Other,
            1 => Glider,
            2 => TowPlane,
            3 => Helicopter,
            4 => Parachute,
            5 => DropPlane,
            6 => HangGlider,
            7 => ParaGlider,
            8 => PoweredAircraft,
            9 => JetAircraft,
            10 => UFO, // Taken from the other spec on the same site
            11 => Balloon,
            12 => Airship,
            13 => UAV,
            14 => GroundSupport, // Taken from the other spec on the same site
            15 => StaticObject,
            _ => Unknown,
        }
    }

    // from: http://wiki.glidernet.org/wiki:ogn-flavoured-aprs
    //
    //  "/z",  //  0 = ?
    //  "/'",  //  1 = (moto-)glider    (most frequent)
    //  "/'",  //  2 = tow plane        (often)
    //  "/X",  //  3 = helicopter       (often)
    //  "/g" , //  4 = parachute        (rare but seen - often mixed with drop plane)
    //  "\\^", //  5 = drop plane       (seen)
    //  "/g" , //  6 = hang-glider      (rare but seen)
    //  "/g" , //  7 = para-glider      (rare but seen)
    //  "\\^", //  8 = powered aircraft (often)
    //  "/^",  //  9 = jet aircraft     (rare but seen)
    //  "/z",  //  A = UFO              (people set for fun)
    //  "/O",  //  B = balloon          (seen once)
    //  "/O",  //  C = airship          (seen once)
    //  "/'",  //  D = UAV              (drones, can become very common)
    //  "/z",  //  E = ground support   (ground vehicles at airfields)
    //  "\\n"  //  F = static object    (ground relay ?)
    pub fn from_aprs_symbol(symbol: ParsedSymbol) -> Self {
        use AircraftType::*;

        let concat = format!("{}{}", symbol.0, symbol.1);

        match concat.as_str() {
            // as this is not injective, we choose the first listed type.
            "/z" => Other,
            "/'" => Glider,
            "/X" => Helicopter,
            "/g" => Parachute,
            "\\^" => PoweredAircraft,
            "/^" => JetAircraft,
            "/O" => Balloon,
            "\\n" => StaticObject,
            _ => Unknown,
        }
    }
}
