#[derive(Debug, PartialEq, Eq)]
pub enum AddressType {
    Unknown,
    ICAO,
    FLARM,
    OGN,
}

impl AddressType {
    // from: http://wiki.glidernet.org/wiki:ogn-flavoured-aprs
    // Address Type: 0b00 -> Unkown, 0b01 -> ICAO, 0b10 -> Flarm, 0b11 -> OGN tracker
    pub fn from(meta: usize) -> Self {
        let masked = meta & 0b0011;
        match masked {
            0 => AddressType::Unknown,
            1 => AddressType::ICAO,
            2 => AddressType::FLARM,
            3 => AddressType::OGN,
            _ => AddressType::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_type_is_decoded_properly() {
        let unknown = 0b11011100;
        let icao = 0b11111101;
        let flarm = 0b10111110;
        let ogn = 0b00000011;
        assert_eq!(AddressType::from(unknown), AddressType::Unknown);
        assert_eq!(AddressType::from(icao), AddressType::ICAO);
        assert_eq!(AddressType::from(flarm), AddressType::FLARM);
        assert_eq!(AddressType::from(ogn), AddressType::OGN);
    }
}
