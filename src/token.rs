pub enum Token {
    Other(String),
    Literal(String),
    Float(String),
    Int(String),
    HexString(String),
    MAC(String),
    IPv4(String),
    IPv6(String)
}
