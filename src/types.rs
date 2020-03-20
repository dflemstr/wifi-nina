#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Config<'a> {
    Station(StationConfig<'a>),
    AccessPoint(AccessPointConfig<'a>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StationConfig<'a> {
    pub network: NetworkConfig<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NetworkConfig<'a> {
    Open { ssid: &'a [u8] },
    Password { ssid: &'a [u8], password: &'a [u8] },
    // TODO: WPA2 enterprise etc
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccessPointConfig<'a> {
    pub ssid: &'a [u8],
    pub password: &'a [u8],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Socket(pub(crate) u8);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScannedNetwork {
    pub ssid: arrayvec::ArrayVec<[u8; 32]>,
    pub rssi: i32,
    pub encryption_type: EncryptionType,
    pub bssid: [u8; 6],
    pub channel: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum PinMode {
    Input = 0,
    Output = 1,
    InputPullup = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum ProtocolMode {
    Tcp = 0,
    Udp = 1,
    Tls = 2,
    UdpMulticast = 3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum ConnectionState {
    IdleStatus = 0,
    NoSsidAvail = 1,
    ScanCompleted = 2,
    Connected = 3,
    ConnectFailed = 4,
    ConnectionLost = 5,
    Disconnected = 6,
    ApListening = 7,
    ApConnected = 8,
    ApFailed = 9,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum TcpState {
    Closed = 0,
    Listen = 1,
    SynSent = 2,
    SynRcvd = 3,
    Established = 4,
    FinWait1 = 5,
    FinWait2 = 6,
    CloseWait = 7,
    Closing = 8,
    LastAck = 9,
    TimeWait = 10,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum EncryptionType {
    Invalid = 0,
    Auto = 1,
    OpenSystem = 2,
    SharedKey = 3,
    Wpa = 4,
    Wpa2 = 5,
    WpaPsk = 6,
    Wpa2Psk = 7,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NetworkData {
    pub ip: no_std_net::Ipv4Addr,
    pub mask: no_std_net::Ipv4Addr,
    pub gateway: no_std_net::Ipv4Addr,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RemoteData {
    pub ip: no_std_net::Ipv4Addr,
    pub port: u32,
}
