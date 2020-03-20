use crate::types;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error<E> {
    Transport(E),
    SetNetwork,
    SetPassphrase,
    SetKey,
    SetIpConfig,
    SetDnsConfig,
    SetHostname,
    Disconnect,
    StartScanNetworks,
    StartClientByIp,
    StopClient,
    CheckDataSent,
    PinMode,
    DigitalWrite,
    AnalogWrite,
    ConnectionFailure(types::ConnectionState),
    BadConnectionStatus(num_enum::TryFromPrimitiveError<types::ConnectionState>),
    BadEncryptionType(num_enum::TryFromPrimitiveError<types::EncryptionType>),
    BadTcpState(num_enum::TryFromPrimitiveError<types::TcpState>),
    DataTooLong,
}
