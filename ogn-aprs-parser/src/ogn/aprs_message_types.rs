#[derive(Debug, PartialEq, Eq)]
pub enum APRSMessageType {
    Status,
    PositionWithTimestamp,
}

impl APRSMessageType {
    pub fn from(c: char) -> Self {
        use APRSMessageType::*;
        match c {
            '>' => Status,
            '/' => PositionWithTimestamp,
        }
    }
}
