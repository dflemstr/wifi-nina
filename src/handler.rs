#![allow(dead_code)]

use crate::command;
use crate::error;
use crate::param;
use crate::params;
use crate::transport;
use crate::types;
use core::fmt;
use core::time;

#[derive(Debug)]
pub struct Handler<T> {
    transport: T,
}

impl<T> Handler<T>
where
    T: transport::Transport,
{
    pub fn new(transport: T) -> Self {
        Self { transport }
    }

    pub fn get_connection_state(
        &mut self,
    ) -> Result<types::ConnectionState, error::Error<T::Error>> {
        use core::convert::TryFrom;

        let mut recv_params = (0u8,);

        self.handle_cmd(command::Command::GetConnStatusCmd, &(), &mut recv_params)?;

        let (status,) = recv_params;
        let status =
            types::ConnectionState::try_from(status).map_err(error::Error::BadConnectionStatus)?;

        Ok(status)
    }

    pub fn delay(&mut self, duration: time::Duration) -> Result<(), error::Error<T::Error>> {
        self.transport
            .delay(duration)
            .map_err(error::Error::Transport)
    }

    pub fn get_firmware_version(
        &mut self,
    ) -> Result<arrayvec::ArrayVec<[u8; 16]>, error::Error<T::Error>> {
        let send_params = (0u8,);
        let mut recv_params = (param::NullTerminated::new(arrayvec::ArrayVec::new()),);

        self.handle_cmd(
            command::Command::GetFwVersionCmd,
            &send_params,
            &mut recv_params,
        )?;

        let result = recv_params.0.into_inner();

        Ok(result)
    }

    pub fn get_mac_address(&mut self) -> Result<[u8; 6], error::Error<T::Error>> {
        let send_params = (0u8,);
        let mut recv_params = (arrayvec::ArrayVec::new(),);

        self.handle_cmd(
            command::Command::GetMacaddrCmd,
            &send_params,
            &mut recv_params,
        )?;

        Ok(recv_params.0.into_inner().unwrap())
    }

    pub fn start_scan_networks(&mut self) -> Result<(), error::Error<T::Error>> {
        let mut recv_params = (0u8,);

        self.handle_cmd(command::Command::StartScanNetworks, &(), &mut recv_params)?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(error::Error::StartScanNetworks)
        }
    }

    pub fn get_scanned_networks(
        &mut self,
    ) -> Result<arrayvec::ArrayVec<[arrayvec::ArrayVec<[u8; 32]>; 16]>, error::Error<T::Error>>
    {
        let mut recv_params: arrayvec::ArrayVec<[arrayvec::ArrayVec<[u8; 32]>; 16]> =
            arrayvec::ArrayVec::new();

        self.handle_cmd(command::Command::ScanNetworks, &(), &mut recv_params)?;

        Ok(recv_params)
    }

    pub fn get_scanned_network_rssi(&mut self, network: u8) -> Result<i32, error::Error<T::Error>> {
        let send_params = (network,);
        let mut recv_params = (param::Scalar::be(0u32),);

        self.handle_cmd(
            command::Command::GetIdxRssiCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (rssi,) = recv_params;

        Ok(rssi.into_inner() as i32)
    }

    pub fn get_scanned_network_encryption_type(
        &mut self,
        network: u8,
    ) -> Result<types::EncryptionType, error::Error<T::Error>> {
        use core::convert::TryFrom;

        let send_params = (network,);
        let mut recv_params = (0u8,);

        self.handle_cmd(
            command::Command::GetIdxEnctCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (encryption_type,) = recv_params;

        let encryption_type = types::EncryptionType::try_from(encryption_type)
            .map_err(error::Error::BadEncryptionType)?;

        Ok(encryption_type)
    }

    pub fn get_scanned_network_bssid(
        &mut self,
        network: u8,
    ) -> Result<[u8; 6], error::Error<T::Error>> {
        let send_params = (network,);
        let mut recv_params = (arrayvec::ArrayVec::new(),);

        self.handle_cmd(
            command::Command::GetIdxBssid,
            &send_params,
            &mut recv_params,
        )?;

        let (bssid,) = recv_params;

        Ok(bssid.into_inner().unwrap())
    }

    pub fn get_scanned_network_channel(
        &mut self,
        network: u8,
    ) -> Result<u8, error::Error<T::Error>> {
        let send_params = (network,);
        let mut recv_params = (0u8,);

        self.handle_cmd(
            command::Command::GetIdxChannelCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (channel,) = recv_params;

        Ok(channel)
    }

    pub fn get_network_data(&mut self) -> Result<types::NetworkData, error::Error<T::Error>> {
        let send_params = (0u8,);
        let mut recv_params = (
            param::Scalar::be(0u32),
            param::Scalar::be(0u32),
            param::Scalar::be(0u32),
        );

        self.handle_cmd(
            command::Command::GetIpaddrCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (ip, mask, gateway) = recv_params;
        let ip = ip.into_inner().into();
        let mask = mask.into_inner().into();
        let gateway = gateway.into_inner().into();

        Ok(types::NetworkData { ip, mask, gateway })
    }

    pub fn get_remote_data(
        &mut self,
        socket: types::Socket,
    ) -> Result<types::RemoteData, error::Error<T::Error>> {
        let send_params = (socket.0,);
        let mut recv_params = (param::Scalar::be(0u32), param::Scalar::be(0u32));

        self.handle_cmd(
            command::Command::GetRemoteDataCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (ip, port) = recv_params;
        let ip = ip.into_inner().into();
        let port = port.into_inner();

        Ok(types::RemoteData { ip, port })
    }

    pub fn set_network(&mut self, ssid: &[u8]) -> Result<(), error::Error<T::Error>> {
        let send_params = (param::NullTerminated::new(ssid),);
        let mut recv_params = (0u8,);

        self.handle_cmd(command::Command::SetNetCmd, &send_params, &mut recv_params)?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(error::Error::SetNetwork)
        }
    }

    pub fn set_passphrase(
        &mut self,
        ssid: &[u8],
        passphrase: &[u8],
    ) -> Result<(), error::Error<T::Error>> {
        let send_params = (
            param::NullTerminated::new(ssid),
            param::NullTerminated::new(passphrase),
        );
        let mut recv_params = (0u8,);

        self.handle_cmd(
            command::Command::SetPassphraseCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(error::Error::SetPassphrase)
        }
    }

    pub fn set_key(
        &mut self,
        ssid: &str,
        key_idx: u8,
        key: &[u8],
    ) -> Result<(), error::Error<T::Error>> {
        let send_params = (
            param::NullTerminated::new(ssid.as_bytes()),
            key_idx,
            // TODO: null terminate?
            key,
        );
        let mut recv_params = (0u8,);

        self.handle_cmd(command::Command::SetKeyCmd, &send_params, &mut recv_params)?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(error::Error::SetKey)
        }
    }

    pub fn config(
        &mut self,
        valid_params: u8,
        local_ip: no_std_net::Ipv4Addr,
        gateway: no_std_net::Ipv4Addr,
        subnet: no_std_net::Ipv4Addr,
    ) -> Result<(), error::Error<T::Error>> {
        let send_params = (
            valid_params,
            param::Scalar::be(u32::from(local_ip)),
            param::Scalar::be(u32::from(gateway)),
            param::Scalar::be(u32::from(subnet)),
        );
        let mut recv_params = (0u8,);

        self.handle_cmd(
            command::Command::SetIpConfigCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(error::Error::SetIpConfig)
        }
    }

    pub fn set_dns(
        &mut self,
        valid_params: u8,
        dns_server1: no_std_net::Ipv4Addr,
        dns_server2: no_std_net::Ipv4Addr,
    ) -> Result<(), error::Error<T::Error>> {
        let send_params = (
            valid_params,
            param::Scalar::be(u32::from(dns_server1)),
            param::Scalar::be(u32::from(dns_server2)),
        );
        let mut recv_params = (0u8,);

        self.handle_cmd(
            command::Command::SetDnsConfigCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(error::Error::SetDnsConfig)
        }
    }

    pub fn set_hostname(&mut self, hostname: &str) -> Result<(), error::Error<T::Error>> {
        let send_params = (param::NullTerminated::new(hostname.as_bytes()),);
        let mut recv_params = (0u8,);

        self.handle_cmd(
            command::Command::SetHostnameCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(error::Error::SetHostname)
        }
    }

    pub fn disconnect(&mut self) -> Result<(), error::Error<T::Error>> {
        let send_params = (0u8,);
        let mut recv_params = (0u8,);

        self.handle_cmd(
            command::Command::DisconnectCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(error::Error::Disconnect)
        }
    }

    pub fn get_current_ssid(
        &mut self,
    ) -> Result<arrayvec::ArrayVec<[u8; 32]>, error::Error<T::Error>> {
        let send_params = (0u8,);
        let mut recv_params = (arrayvec::ArrayVec::new(),);

        self.handle_cmd(
            command::Command::GetCurrSsidCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (ssid,) = recv_params;

        Ok(ssid)
    }

    pub fn get_current_bssid(
        &mut self,
    ) -> Result<arrayvec::ArrayVec<[u8; 6]>, error::Error<T::Error>> {
        let send_params = (0u8,);
        let mut recv_params = (arrayvec::ArrayVec::new(),);

        self.handle_cmd(
            command::Command::GetCurrBssidCmd,
            &send_params,
            &mut recv_params,
        )?;

        Ok(recv_params.0)
    }

    pub fn get_current_rssi(&mut self) -> Result<i32, error::Error<T::Error>> {
        let send_params = (0u8,);
        let mut recv_params = (param::Scalar::be(0u32),);

        self.handle_cmd(
            command::Command::GetCurrRssiCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (rssi,) = recv_params;

        Ok(rssi.into_inner() as i32)
    }

    pub fn get_current_encryption_type(
        &mut self,
    ) -> Result<types::EncryptionType, error::Error<T::Error>> {
        use core::convert::TryFrom;

        let send_params = (0u8,);
        let mut recv_params = (0u8,);

        self.handle_cmd(
            command::Command::GetCurrEnctCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (encryption_type,) = recv_params;

        let encryption_type = types::EncryptionType::try_from(encryption_type)
            .map_err(error::Error::BadEncryptionType)?;

        Ok(encryption_type)
    }

    pub fn start_client_by_ip(
        &mut self,
        ip: no_std_net::Ipv4Addr,
        port: u16,
        socket: types::Socket,
        protocol_mode: types::ProtocolMode,
    ) -> Result<(), error::Error<T::Error>> {
        let send_params = (
            param::Scalar::be(u32::from(ip)),
            param::Scalar::be(port),
            socket.0,
            u8::from(protocol_mode),
        );
        let mut recv_params = (0u8,);

        self.handle_cmd(
            command::Command::StartClientTcpCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(error::Error::StartClientByIp)
        }
    }

    pub fn stop_client(&mut self, socket: types::Socket) -> Result<(), error::Error<T::Error>> {
        let send_params = (socket.0,);
        let mut recv_params = (0u8,);

        self.handle_cmd(
            command::Command::StopClientTcpCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(error::Error::StopClient)
        }
    }

    pub fn get_client_state(
        &mut self,
        socket: types::Socket,
    ) -> Result<types::TcpState, error::Error<T::Error>> {
        use core::convert::TryFrom;

        let send_params = (socket.0,);
        let mut recv_params = (0u8,);

        self.handle_cmd(
            command::Command::GetClientStateTcpCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (state,) = recv_params;
        let state = types::TcpState::try_from(state).map_err(error::Error::BadTcpState)?;

        Ok(state)
    }

    pub fn avail_data(&mut self, socket: types::Socket) -> Result<u16, error::Error<T::Error>> {
        let send_params = (socket.0,);
        let mut recv_params = (param::Scalar::le(0u16),);

        self.handle_cmd(
            command::Command::AvailDataTcpCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (data,) = recv_params;

        Ok(data.into_inner())
    }

    pub fn get_data_buf(
        &mut self,
        socket: types::Socket,
        buf: &mut [u8],
    ) -> Result<usize, error::Error<T::Error>> {
        use core::convert::TryFrom;
        let send_params = (
            socket.0,
            param::Scalar::le(u16::try_from(buf.len()).unwrap()),
        );
        let mut recv_params = (buf,);

        self.handle_long_send_long_recv_cmd(
            command::Command::GetDatabufTcpCmd,
            &send_params,
            &mut recv_params,
        )?;

        Ok(recv_params.0.len())
    }

    pub fn send_data(
        &mut self,
        socket: types::Socket,
        data: &[u8],
    ) -> Result<usize, error::Error<T::Error>> {
        let send_params = (socket.0, &data);
        let mut recv_params = (param::Scalar::le(0u16),);

        self.handle_long_send_cmd(
            command::Command::SendDataTcpCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (len,) = recv_params;

        Ok(len.into_inner() as usize)
    }

    pub fn check_data_sent(&mut self, socket: types::Socket) -> Result<(), error::Error<T::Error>> {
        let send_params = (socket.0,);
        let mut recv_params = (0u8,);

        self.handle_cmd(
            command::Command::DataSentTcpCmd,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(error::Error::CheckDataSent)
        }
    }

    pub fn get_socket(&mut self) -> Result<types::Socket, error::Error<T::Error>> {
        let mut recv_params = (0u8,);

        self.handle_cmd(command::Command::GetSocketCmd, &(), &mut recv_params)?;

        let (socket,) = recv_params;
        let socket = types::Socket(socket);

        Ok(socket)
    }

    pub fn pin_mode(
        &mut self,
        pin: u8,
        mode: types::PinMode,
    ) -> Result<(), error::Error<T::Error>> {
        let send_params = (pin, u8::from(mode));
        let mut recv_params = (0u8,);

        self.handle_cmd(command::Command::SetPinMode, &send_params, &mut recv_params)?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(error::Error::PinMode)
        }
    }

    pub fn digital_write(&mut self, pin: u8, value: u8) -> Result<(), error::Error<T::Error>> {
        let send_params = (pin, value);
        let mut recv_params = (0u8,);

        self.handle_cmd(
            command::Command::SetDigitalWrite,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(error::Error::DigitalWrite)
        }
    }

    pub fn analog_write(&mut self, pin: u8, value: u8) -> Result<(), error::Error<T::Error>> {
        let send_params = (pin, value);
        let mut recv_params = (0u8,);

        self.handle_cmd(
            command::Command::SetAnalogWrite,
            &send_params,
            &mut recv_params,
        )?;

        let (status,) = recv_params;

        if status == 1 {
            Ok(())
        } else {
            Err(error::Error::AnalogWrite)
        }
    }

    fn handle_cmd<SP, RP>(
        &mut self,
        command: command::Command,
        send_params: &SP,
        recv_params: &mut RP,
    ) -> Result<(), error::Error<T::Error>>
    where
        SP: params::SendParams + fmt::Debug,
        RP: params::RecvParams + fmt::Debug,
    {
        self.transport
            .handle_cmd(command, send_params, recv_params, false, false)
            .map_err(error::Error::Transport)
    }

    fn handle_long_send_cmd<SP, RP>(
        &mut self,
        command: command::Command,
        send_params: &SP,
        recv_params: &mut RP,
    ) -> Result<(), error::Error<T::Error>>
    where
        SP: params::SendParams + fmt::Debug,
        RP: params::RecvParams + fmt::Debug,
    {
        self.transport
            .handle_cmd(command, send_params, recv_params, true, false)
            .map_err(error::Error::Transport)
    }

    fn handle_long_send_long_recv_cmd<SP, RP>(
        &mut self,
        command: command::Command,
        send_params: &SP,
        recv_params: &mut RP,
    ) -> Result<(), error::Error<T::Error>>
    where
        SP: params::SendParams + fmt::Debug,
        RP: params::RecvParams + fmt::Debug,
    {
        self.transport
            .handle_cmd(command, send_params, recv_params, true, true)
            .map_err(error::Error::Transport)
    }
}
