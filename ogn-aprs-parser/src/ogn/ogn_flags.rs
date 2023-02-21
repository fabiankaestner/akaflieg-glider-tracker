#[derive(Debug, PartialEq, Eq)]
pub struct OGNFlags {
    stealth_mode: bool,     // should never be true
    no_tracking_mode: bool, // request from airplane not to be tracked
}

// taken from: http://wiki.glidernet.org/wiki:ogn-flavoured-aprs
//
// STttttaa
//
// S, T, tttt, aa stand for 8 bits from most to least significant. Note that no messages with
// the no-tracking set flag should ever appear on the public APRS network.
// Sample: id02DF0A52
// 0x06 = 0b00000110: sender details (2 digit hex number encoding 8 bits)
// 0b0 = false: stealth mode boolean (should never be "1")
// 0b0 = false: no-tracking boolean (must ignore if "1")
// 0b0001 = 1: sender (aircraft) type ("GLIDER")
// 0b10 = 2: address type ("FLARM")
// DF0A52: sender address
impl OGNFlags {
    pub fn from(meta: usize) -> Self {
        let no_tracking_mode = ((meta >> 6) & 0b0001) != 0;
        let stealth_mode = ((meta >> 7) & 0b0001) != 0;
        OGNFlags {
            stealth_mode,
            no_tracking_mode,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ogn_flags_are_decoded_correctly() {
        let both = 0b11011100;
        let track = 0b01011110;
        let stealth = 0b10111110;
        let none = 0b00000011;
        assert_eq!(OGNFlags::from(both), OGNFlags{no_tracking_mode: true, stealth_mode: true});
        assert_eq!(OGNFlags::from(track), OGNFlags{no_tracking_mode: true, stealth_mode: false});
        assert_eq!(OGNFlags::from(stealth), OGNFlags{no_tracking_mode: false, stealth_mode: true});
        assert_eq!(OGNFlags::from(none), OGNFlags{no_tracking_mode: false, stealth_mode: false});
    }
}
