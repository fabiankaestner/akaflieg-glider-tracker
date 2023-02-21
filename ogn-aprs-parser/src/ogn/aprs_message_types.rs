#[derive(Debug, PartialEq, Eq)]
pub enum APRSMessageType {
    Status,
    PositionWithTimestamp,
    Unknown,
}

impl APRSMessageType {
    pub fn from(c: char) -> Self {
        use APRSMessageType::*;
        match c {
            '>' => Status,
            '/' => PositionWithTimestamp,
            _ => Unknown
        }
    }
}
