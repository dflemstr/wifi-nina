use crate::command;
use crate::params;
use core::fmt;
use core::time;

mod spi;

pub use spi::SpiError;
pub use spi::SpiTransport;

pub trait Transport {
    type Error;

    fn reset(&mut self) -> Result<(), Self::Error>;

    fn delay(&mut self, duration: time::Duration) -> Result<(), Self::Error>;

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
        RP: params::RecvParams + fmt::Debug;
}
