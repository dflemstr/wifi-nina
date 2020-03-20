pub trait FullDuplexExt<Word>: embedded_hal::spi::FullDuplex<Word>
where
    Word: core::fmt::LowerHex + Copy + Default,
{
    #[inline]
    fn send_exchange(&mut self, word: Word) -> Result<(), Self::Error> {
        nb::block!(self.send(word))?;
        log::trace!("send {:#04x}", word);
        nb::block!(self.read())?;
        Ok(())
    }

    #[inline]
    fn recv_exchange(&mut self) -> Result<Word, Self::Error> {
        nb::block!(self.send(Word::default()))?;
        let byte = nb::block!(self.read())?;
        log::trace!("recv {:#04x}", byte);
        Ok(byte)
    }
}

impl<T, Word> FullDuplexExt<Word> for T
where
    T: embedded_hal::spi::FullDuplex<Word>,
    Word: core::fmt::LowerHex + Copy + Default,
{
}
