#![allow(dead_code)]

use super::command;
use super::param;
use super::params;
use super::transport;
use core::fmt;
use embedded_hal::digital::v2::{InputPin, OutputPin};

#[derive(Debug, Eq, PartialEq)]
pub struct Handler<E, GPIO0, BUSY, RESET, CS, DELAY> {
    transport: transport::Transport<E, GPIO0, BUSY, RESET, CS, DELAY>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Socket(u8);

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error<ESPI, EGPIO> {
    Transport(transport::Error<ESPI, EGPIO>),
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
    PinMode,
    DigitalWrite,
    AnalogWrite,
    BadConnectionStatus(num_enum::TryFromPrimitiveError<ConnectionStatus>),
    BadEncryptionType(num_enum::TryFromPrimitiveError<EncryptionType>),
    BadTcpState(num_enum::TryFromPrimitiveError<TcpState>),
    DataTooLong,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum ConnectionStatus {
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
    ip: no_std_net::Ipv4Addr,
    mask: no_std_net::Ipv4Addr,
    gateway: no_std_net::Ipv4Addr,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RemoteData {
    ip: no_std_net::Ipv4Addr,
    port: u32,
}

impl<EGPIO, GPIO0, BUSY, RESET, CS, DELAY> Handler<EGPIO, GPIO0, BUSY, RESET, CS, DELAY>
where
    GPIO0: InputPin<Error = EGPIO>,
    BUSY: InputPin<Error = EGPIO>,
    RESET: OutputPin<Error = EGPIO>,
    CS: OutputPin<Error = EGPIO>,
    DELAY: FnMut(core::time::Duration),
{
    pub fn start(
        gpio0: GPIO0,
        busy: BUSY,
        reset: RESET,
        cs: CS,
        delay: DELAY,
    ) -> Result<Self, EGPIO> {
        let transport = transport::Transport::start(gpio0, busy, reset, cs, delay)?;
        Ok(Self { transport })
    }

    pub fn get_connection_status<S, ESPI>(
        &mut self,
        spi: &mut S,
    ) -> Result<ConnectionStatus, Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        use core::convert::TryFrom;

        let send_params = ();
        let mut recv_params = (0u8,);

        self.handle_cmd(
            spi,
            command::Command::GetConnStatusCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;
        let status = ConnectionStatus::try_from(status).map_err(Error::BadConnectionStatus)?;

        Ok(status)
    }

    pub fn get_firmware_version<S, ESPI>(
        &mut self,
        spi: &mut S,
        buf: &mut [u8],
    ) -> Result<usize, Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (0u8,);
        let mut recv_params = (param::Short::new(buf),);

        self.handle_cmd(
            spi,
            command::Command::GetFwVersionCmd,
            &send_params,
            &mut recv_params,
        )?;

        Ok(recv_params.0.recv_len as usize)
    }

    pub fn get_network_data<S, ESPI>(
        &mut self,
        spi: &mut S,
    ) -> Result<NetworkData, Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (0u8,);
        let mut recv_params = (0u32, 0u32, 0u32);

        self.handle_cmd(
            spi,
            command::Command::GetIpaddrCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (ip, mask, gateway) = recv_params;
        let ip = ip.into();
        let mask = mask.into();
        let gateway = gateway.into();

        Ok(NetworkData { ip, mask, gateway })
    }

    pub fn get_remote_data<S, ESPI>(
        &mut self,
        spi: &mut S,
        socket: Socket,
    ) -> Result<RemoteData, Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (socket.0,);
        let mut recv_params = (0u32, 0u32);

        self.handle_cmd(
            spi,
            command::Command::GetRemoteDataCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (ip, port) = recv_params;
        let ip = ip.into();

        Ok(RemoteData { ip, port })
    }

    pub fn set_network<S, ESPI>(
        &mut self,
        spi: &mut S,
        ssid: &str,
    ) -> Result<(), Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (param::Short::new(ssid.as_bytes()),);
        let mut recv_params = (0u8,);

        self.handle_cmd(
            spi,
            command::Command::SetNetCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(Error::SetNetwork)
        }
    }

    pub fn set_passphrase<S, ESPI>(
        &mut self,
        spi: &mut S,
        ssid: &str,
        passphrase: &str,
    ) -> Result<(), Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (
            param::Short::new(ssid.as_bytes()),
            param::Short::new(passphrase.as_bytes()),
        );
        let mut recv_params = (0u8,);

        self.handle_cmd(
            spi,
            command::Command::SetPassphraseCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(Error::SetPassphrase)
        }
    }

    pub fn set_key<S, ESPI>(
        &mut self,
        spi: &mut S,
        ssid: &str,
        key_idx: u8,
        key: &[u8],
    ) -> Result<(), Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (
            param::Short::new(ssid.as_bytes()),
            key_idx,
            param::Short::new(key),
        );
        let mut recv_params = (0u8,);

        self.handle_cmd(
            spi,
            command::Command::SetKeyCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(Error::SetKey)
        }
    }

    pub fn config<S, ESPI>(
        &mut self,
        spi: &mut S,
        valid_params: u8,
        local_ip: no_std_net::Ipv4Addr,
        gateway: no_std_net::Ipv4Addr,
        subnet: no_std_net::Ipv4Addr,
    ) -> Result<(), Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (
            valid_params,
            u32::from(local_ip),
            u32::from(gateway),
            u32::from(subnet),
        );
        let mut recv_params = (0u8,);

        self.handle_cmd(
            spi,
            command::Command::SetIpConfigCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(Error::SetIpConfig)
        }
    }

    pub fn set_dns<S, ESPI>(
        &mut self,
        spi: &mut S,
        valid_params: u8,
        dns_server1: no_std_net::Ipv4Addr,
        dns_server2: no_std_net::Ipv4Addr,
    ) -> Result<(), Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (valid_params, u32::from(dns_server1), u32::from(dns_server2));
        let mut recv_params = (0u8,);

        self.handle_cmd(
            spi,
            command::Command::SetDnsConfigCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(Error::SetDnsConfig)
        }
    }

    pub fn set_hostname<S, ESPI>(
        &mut self,
        spi: &mut S,
        hostname: &str,
    ) -> Result<(), Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (param::Short::new(hostname.as_bytes()),);
        let mut recv_params = (0u8,);

        self.handle_cmd(
            spi,
            command::Command::SetHostnameCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(Error::SetHostname)
        }
    }

    pub fn disconnect<S, ESPI>(&mut self, spi: &mut S) -> Result<(), Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (0u8,);
        let mut recv_params = (0u8,);

        self.handle_cmd(
            spi,
            command::Command::DisconnectCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(Error::Disconnect)
        }
    }

    pub fn get_mac_address<S, ESPI>(
        &mut self,
        spi: &mut S,
        buf: &mut [u8],
    ) -> Result<usize, Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (0u8,);
        let mut recv_params = (param::Short::new(buf),);

        self.handle_cmd(
            spi,
            command::Command::GetMacaddrCmd,
            &send_params,
            &mut recv_params,
        )?;

        Ok(recv_params.0.recv_len as usize)
    }

    pub fn get_current_ssid<'a, S, ESPI>(
        &mut self,
        spi: &mut S,
        buf: &mut [u8],
    ) -> Result<usize, Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (0u8,);
        let mut recv_params = (param::Short::new(buf),);

        self.handle_cmd(
            spi,
            command::Command::GetCurrSsidCmd,
            &send_params,
            &mut recv_params,
        )?;

        Ok(recv_params.0.recv_len as usize)
    }

    pub fn get_current_bssid<S, ESPI>(
        &mut self,
        spi: &mut S,
        buf: &mut [u8],
    ) -> Result<usize, Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (0u8,);
        let mut recv_params = (param::Short::new(buf),);

        self.handle_cmd(
            spi,
            command::Command::GetCurrBssidCmd,
            &send_params,
            &mut recv_params,
        )?;

        Ok(recv_params.0.recv_len as usize)
    }

    pub fn get_current_rssi<S, ESPI>(&mut self, spi: &mut S) -> Result<i32, Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (0u8,);
        let mut recv_params = (0u32,);

        self.handle_cmd(
            spi,
            command::Command::GetCurrRssiCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (rssi,) = recv_params;

        Ok(rssi as i32)
    }

    pub fn get_current_encryption_type<S, ESPI>(
        &mut self,
        spi: &mut S,
    ) -> Result<EncryptionType, Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        use core::convert::TryFrom;

        let send_params = (0u8,);
        let mut recv_params = (0u8,);

        self.handle_cmd(
            spi,
            command::Command::GetCurrEnctCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (encryption_type,) = recv_params;

        let encryption_type =
            EncryptionType::try_from(encryption_type).map_err(Error::BadEncryptionType)?;

        Ok(encryption_type)
    }

    pub fn start_scan_networks<S, ESPI>(&mut self, spi: &mut S) -> Result<(), Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = ();
        let mut recv_params = (0u8,);

        self.handle_cmd(
            spi,
            command::Command::StartScanNetworks,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(Error::StartScanNetworks)
        }
    }

    pub fn start_client_by_ip<S, ESPI>(
        &mut self,
        spi: &mut S,
        ip: no_std_net::Ipv4Addr,
        port: u16,
        socket: Socket,
        protocol_mode: ProtocolMode,
    ) -> Result<(), Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (u32::from(ip), port, socket.0, u8::from(protocol_mode));
        let mut recv_params = (0u8,);

        self.handle_cmd(
            spi,
            command::Command::StartClientTcpCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(Error::StartClientByIp)
        }
    }

    pub fn stop_client<S, ESPI>(
        &mut self,
        spi: &mut S,
        socket: Socket,
    ) -> Result<(), Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (socket.0,);
        let mut recv_params = (0u8,);

        self.handle_cmd(
            spi,
            command::Command::StopClientTcpCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(Error::StopClient)
        }
    }

    pub fn get_client_state<S, ESPI>(
        &mut self,
        spi: &mut S,
        socket: Socket,
    ) -> Result<TcpState, Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        use core::convert::TryFrom;

        let send_params = (socket.0,);
        let mut recv_params = (0u8,);

        self.handle_cmd(
            spi,
            command::Command::GetClientStateTcpCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (state,) = recv_params;
        let state = TcpState::try_from(state).map_err(Error::BadTcpState)?;

        Ok(state)
    }

    pub fn avail_data<S, ESPI>(
        &mut self,
        spi: &mut S,
        socket: Socket,
    ) -> Result<u16, Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (socket.0,);
        let mut recv_params = (0u16,);

        self.handle_cmd(
            spi,
            command::Command::AvailDataTcpCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (data,) = recv_params;

        Ok(data)
    }

    pub fn get_data_buf<S, ESPI>(
        &mut self,
        spi: &mut S,
        socket: Socket,
        data: &mut [u8],
    ) -> Result<usize, Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (socket.0, data.len() as u16);
        let mut recv_params = (param::Long::new(data),);

        self.handle_cmd(
            spi,
            command::Command::GetDatabufTcpCmd,
            &send_params,
            &mut recv_params,
        )?;

        Ok(recv_params.0.recv_len as usize)
    }

    pub fn send_data<'a, S, ESPI>(
        &mut self,
        spi: &mut S,
        socket: Socket,
        data: &[u8],
    ) -> Result<usize, Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (socket.0, param::Long::new(data));
        let mut recv_params = (0u16,);

        self.handle_cmd(
            spi,
            command::Command::SendDataTcpCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (len,) = recv_params;

        Ok(len as usize)
    }

    pub fn get_socket<S, ESPI>(&mut self, spi: &mut S) -> Result<Socket, Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = ();
        let mut recv_params = (0u8,);

        self.handle_cmd(
            spi,
            command::Command::GetSocketCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (socket,) = recv_params;
        let socket = Socket(socket);

        Ok(socket)
    }

    pub fn pin_mode<S, ESPI>(
        &mut self,
        spi: &mut S,
        pin: u8,
        mode: PinMode,
    ) -> Result<(), Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (pin, u8::from(mode));
        let mut recv_params = (0u8,);

        self.handle_cmd(
            spi,
            command::Command::SetPinMode,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(Error::PinMode)
        }
    }

    pub fn digital_write<S, ESPI>(
        &mut self,
        spi: &mut S,
        pin: u8,
        value: u8,
    ) -> Result<(), Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (pin, value);
        let mut recv_params = (0u8,);

        self.handle_cmd(
            spi,
            command::Command::SetDigitalWrite,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(Error::DigitalWrite)
        }
    }

    pub fn analog_write<S, ESPI>(
        &mut self,
        spi: &mut S,
        pin: u8,
        value: u8,
    ) -> Result<(), Error<ESPI, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8, Error = ESPI>,
    {
        let send_params = (pin, value);
        let mut recv_params = (0u8,);

        self.handle_cmd(
            spi,
            command::Command::SetAnalogWrite,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(Error::AnalogWrite)
        }
    }

    fn handle_cmd<S, SP, RP>(
        &mut self,
        spi: &mut S,
        command: command::Command,
        send_params: &SP,
        recv_params: &mut RP,
    ) -> Result<(), Error<S::Error, EGPIO>>
    where
        SP: params::SendParams + fmt::Debug,
        RP: params::RecvParams + fmt::Debug,
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        self.transport
            .handle_cmd(spi, command, send_params, recv_params)
            .map_err(Error::Transport)
    }
}
