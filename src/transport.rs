use super::command;
use super::full_duplex::FullDuplexExt as _;
use super::params;
use core::fmt;
use core::marker;
use embedded_hal::digital::v2::{InputPin, OutputPin};

#[derive(Debug, Eq, PartialEq)]
pub struct Transport<E, GPIO0, BUSY, RESET, CS, DELAY> {
    gpio0: GPIO0,
    busy: BUSY,
    reset: RESET,
    cs: CS,
    delay: DELAY,
    phantom: marker::PhantomData<E>,
}

const START_CMD: u8 = 0xe0;
const END_CMD: u8 = 0xee;
const ERR_CMD: u8 = 0xef;
const REPLY_FLAG: u8 = 1 << 7;
const WAIT_REPLY_TIMEOUT_BYTES: usize = 1000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error<ESPI, EGPIO> {
    Gpio(EGPIO),
    Spi(ESPI),
    Timeout,
    ErrorResponse,
    UnexpectedReplyByte(u8),
}

impl<EGPIO, GPIO0, BUSY, RESET, CS, DELAY> Transport<EGPIO, GPIO0, BUSY, RESET, CS, DELAY>
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
        let phantom = marker::PhantomData;

        let mut this = Self {
            gpio0,
            busy,
            reset,
            cs,
            phantom,
            delay,
        };

        this.reset()?;

        Ok(this)
    }

    pub fn reset(&mut self) -> Result<(), EGPIO> {
        self.cs.set_high()?;

        self.reset.set_low()?;
        (self.delay)(core::time::Duration::from_millis(10));
        self.reset.set_high()?;
        (self.delay)(core::time::Duration::from_millis(750));

        Ok(())
    }

    pub fn handle_cmd<S, SP, RP>(
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
        self.transaction(|| {
            Self::send_byte(spi, START_CMD)?;
            Self::send_byte(spi, u8::from(command) & !REPLY_FLAG)?;
            Self::send_byte(spi, SP::LEN)?;
            send_params.send(spi).map_err(Error::Spi)?;
            Self::send_byte(spi, END_CMD)?;
            log::debug!("send {:?} {:?}", command, send_params);
            Ok(())
        })?;
        self.transaction(|| {
            Self::await_start_cmd(spi)?;
            Self::expect_byte(spi, u8::from(command) | REPLY_FLAG)?;
            Self::expect_byte(spi, RP::LEN)?;
            recv_params.recv(spi).map_err(Error::Spi)?;
            Self::expect_byte(spi, END_CMD)?;
            log::debug!("recv {:?} {:?} -> {:?}", command, send_params, recv_params);
            Ok(())
        })?;
        Ok(())
    }

    fn await_start_cmd<S>(spi: &mut S) -> Result<(), Error<S::Error, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        for _ in 0..=WAIT_REPLY_TIMEOUT_BYTES {
            let byte = Self::recv_byte(spi)?;
            if byte == ERR_CMD {
                return Err(Error::ErrorResponse);
            }
            if byte == START_CMD {
                return Ok(());
            }
        }
        Err(Error::Timeout)
    }

    fn expect_byte<S>(spi: &mut S, expected_byte: u8) -> Result<(), Error<S::Error, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let byte = Self::recv_byte(spi)?;
        if byte == expected_byte {
            Ok(())
        } else {
            Err(Error::UnexpectedReplyByte(byte))
        }
    }

    fn send_byte<S>(spi: &mut S, byte: u8) -> Result<(), Error<S::Error, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        spi.send_exchange(byte).map_err(Error::Spi)
    }

    fn recv_byte<S>(spi: &mut S) -> Result<u8, Error<S::Error, EGPIO>>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        spi.recv_exchange().map_err(Error::Spi)
    }

    fn transaction<R, S>(
        &mut self,
        func: impl FnOnce() -> Result<R, Error<S, EGPIO>>,
    ) -> Result<R, Error<S, EGPIO>> {
        while self.busy.is_high().map_err(Error::Gpio)? {}

        self.cs.set_low().map_err(Error::Gpio)?;

        while self.busy.is_low().map_err(Error::Gpio)? {}

        let result = func();

        self.cs.set_high().map_err(Error::Gpio)?;

        result
    }
}
