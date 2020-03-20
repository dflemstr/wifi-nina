use crate::command;
use crate::full_duplex::FullDuplexExt as _;
use crate::params;
use core::fmt;
use core::time;
use embedded_hal::digital::v2::{InputPin, OutputPin};

#[derive(Debug)]
pub struct SpiTransport<SPI, BUSY, RESET, CS, DELAY> {
    spi: SPI,
    busy: BUSY,
    reset: RESET,
    cs: CS,
    delay: DELAY,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpiError<SPI, BUSY, RESET, CS> {
    Spi(SPI),
    Busy(BUSY),
    Reset(RESET),
    ChipSelect(CS),
    Timeout,
    ErrorResponse,
    UnexpectedReplyByte(u8),
}

const START_CMD: u8 = 0xe0;
const END_CMD: u8 = 0xee;
const ERR_CMD: u8 = 0xef;
const REPLY_FLAG: u8 = 1 << 7;
const WAIT_REPLY_TIMEOUT_BYTES: usize = 1000;

impl<SPI, BUSY, RESET, CS, DELAY> super::Transport for SpiTransport<SPI, BUSY, RESET, CS, DELAY>
where
    SPI: embedded_hal::spi::FullDuplex<u8>,
    BUSY: InputPin,
    RESET: OutputPin,
    CS: OutputPin,
    DELAY: FnMut(time::Duration),
{
    type Error = SpiError<SPI::Error, BUSY::Error, RESET::Error, CS::Error>;

    #[inline]
    fn reset(&mut self) -> Result<(), Self::Error> {
        self.cs.set_high().map_err(SpiError::ChipSelect)?;

        self.reset.set_low().map_err(SpiError::Reset)?;
        self.delay(time::Duration::from_millis(10))?;
        self.reset.set_high().map_err(SpiError::Reset)?;
        self.delay(time::Duration::from_millis(750))?;

        Ok(())
    }

    #[inline]
    fn delay(&mut self, duration: time::Duration) -> Result<(), Self::Error> {
        (self.delay)(duration);
        Ok(())
    }

    #[inline]
    fn handle_cmd<SP, RP>(
        &mut self,
        command: command::Command,
        send_params: &SP,
        recv_params: &mut RP,
        long_send: bool,
        long_recv: bool,
    ) -> Result<(), Self::Error>
    where
        SP: params::SendParams + fmt::Debug,
        RP: params::RecvParams + fmt::Debug,
    {
        self.transaction(|spi| {
            Self::send_byte(spi, START_CMD)?;
            Self::send_byte(spi, u8::from(command) & !REPLY_FLAG)?;
            send_params.send(spi, long_send).map_err(SpiError::Spi)?;
            Self::send_byte(spi, END_CMD)?;

            // Pad to 4 byte boundary
            let mut total_len = send_params.len(long_send) + 3;
            while 0 != total_len % 4 {
                Self::send_byte(spi, 0)?;
                total_len += 1;
            }

            log::debug!("send {:?} {:?}", command, send_params);
            Ok(())
        })?;
        self.transaction(|spi| {
            Self::await_start_cmd(spi)?;
            Self::expect_byte(spi, u8::from(command) | REPLY_FLAG)?;
            recv_params.recv(spi, long_recv).map_err(SpiError::Spi)?;
            Self::expect_byte(spi, END_CMD)?;

            log::debug!("recv {:?} {:?} -> {:?}", command, send_params, recv_params);
            Ok(())
        })?;
        Ok(())
    }
}

impl<SPI, BUSY, RESET, CS, DELAY> SpiTransport<SPI, BUSY, RESET, CS, DELAY>
where
    SPI: embedded_hal::spi::FullDuplex<u8>,
    BUSY: InputPin,
    RESET: OutputPin,
    CS: OutputPin,
    DELAY: FnMut(time::Duration),
{
    #[inline]
    pub fn start(
        spi: SPI,
        busy: BUSY,
        reset: RESET,
        cs: CS,
        delay: DELAY,
    ) -> Result<Self, SpiError<SPI::Error, BUSY::Error, RESET::Error, CS::Error>> {
        let mut this = Self {
            spi,
            busy,
            reset,
            cs,
            delay,
        };

        super::Transport::reset(&mut this)?;

        Ok(this)
    }

    #[inline]
    fn await_start_cmd(
        spi: &mut SPI,
    ) -> Result<(), SpiError<SPI::Error, BUSY::Error, RESET::Error, CS::Error>> {
        for _ in 0..=WAIT_REPLY_TIMEOUT_BYTES {
            let byte = Self::recv_byte(spi)?;
            if byte == ERR_CMD {
                return Err(SpiError::ErrorResponse);
            }
            if byte == START_CMD {
                return Ok(());
            }
        }
        Err(SpiError::Timeout)
    }

    #[inline]
    fn expect_byte(
        spi: &mut SPI,
        expected_byte: u8,
    ) -> Result<(), SpiError<SPI::Error, BUSY::Error, RESET::Error, CS::Error>> {
        let byte = Self::recv_byte(spi)?;
        if byte == expected_byte {
            Ok(())
        } else {
            Err(SpiError::UnexpectedReplyByte(byte))
        }
    }

    #[inline]
    fn send_byte(
        spi: &mut SPI,
        byte: u8,
    ) -> Result<(), SpiError<SPI::Error, BUSY::Error, RESET::Error, CS::Error>> {
        spi.send_exchange(byte).map_err(SpiError::Spi)
    }

    #[inline]
    fn recv_byte(
        spi: &mut SPI,
    ) -> Result<u8, SpiError<SPI::Error, BUSY::Error, RESET::Error, CS::Error>> {
        spi.recv_exchange().map_err(SpiError::Spi)
    }

    #[inline]
    fn transaction<R>(
        &mut self,
        func: impl FnOnce(
            &mut SPI,
        )
            -> Result<R, SpiError<SPI::Error, BUSY::Error, RESET::Error, CS::Error>>,
    ) -> Result<R, SpiError<SPI::Error, BUSY::Error, RESET::Error, CS::Error>> {
        while self.busy.is_high().map_err(SpiError::Busy)? {}

        self.cs.set_low().map_err(SpiError::ChipSelect)?;

        while self.busy.is_low().map_err(SpiError::Busy)? {}

        let result = func(&mut self.spi);

        self.cs.set_high().map_err(SpiError::ChipSelect)?;

        result
    }
}
