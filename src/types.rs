use core::fmt;

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

impl fmt::Display for ScannedNetwork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use itertools::Itertools;
        if let Ok(ssid) = core::str::from_utf8(&self.ssid[..]) {
            write!(
                f,
                "{:32} {:>8} {:3}dBm ch {:<2} [{:02x}]",
                ssid,
                self.encryption_type,
                self.rssi,
                self.channel,
                self.bssid.iter().format(":")
            )
        } else {
            write!(
                f,
                "{:64x} {:>8} {:3}dBm ch {:<2} [{:02x}]",
                self.ssid.iter().format(""),
                self.encryption_type,
                self.rssi,
                self.channel,
                self.bssid.iter().format(":")
            )
        }
    }
}

impl fmt::Display for EncryptionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match *self {
            EncryptionType::Invalid => "???",
            EncryptionType::Auto => "Auto",
            EncryptionType::OpenSystem => "Open",
            EncryptionType::SharedKey => "PSK",
            EncryptionType::Wpa => "WPA",
            EncryptionType::Wpa2 => "WPA2",
            EncryptionType::WpaPsk => "WPA PSK",
            EncryptionType::Wpa2Psk => "WPA2 PSK",
        };
        f.pad(string)
    }
}
