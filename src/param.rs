use super::full_duplex::FullDuplexExt as _;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Short<A>
where
    A: ?Sized,
{
    pub recv_len: u8,
    pub value: A,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Long<A>
where
    A: ?Sized,
{
    pub recv_len: u16,
    pub value: A,
}

pub trait SendParam {
    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>;
}

pub trait RecvParam {
    fn recv<S>(&mut self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>;
}

impl SendParam for u8 {
    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        spi.send_exchange(1)?;
        spi.send_exchange(*self)?;
        Ok(())
    }
}

impl RecvParam for u8 {
    fn recv<S>(&mut self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        assert_eq!(spi.recv_exchange()?, 1);
        *self = spi.recv_exchange()?;
        Ok(())
    }
}

impl SendParam for u16 {
    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        spi.send_exchange(2)?;
        spi.send_exchange((*self >> 8) as u8)?;
        spi.send_exchange(*self as u8)?;
        Ok(())
    }
}

impl RecvParam for u16 {
    fn recv<S>(&mut self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        assert_eq!(spi.recv_exchange()?, 2);
        *self = (spi.recv_exchange()? as u16) << 8 | (spi.recv_exchange()? as u16);
        Ok(())
    }
}

impl SendParam for u32 {
    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        spi.send_exchange(4)?;
        spi.send_exchange((*self >> 24) as u8)?;
        spi.send_exchange((*self >> 16) as u8)?;
        spi.send_exchange((*self >> 8) as u8)?;
        spi.send_exchange(*self as u8)?;
        Ok(())
    }
}

impl RecvParam for u32 {
    fn recv<S>(&mut self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        assert_eq!(spi.recv_exchange()?, 4);
        *self = (spi.recv_exchange()? as u32) << 32
            | (spi.recv_exchange()? as u32) << 16
            | (spi.recv_exchange()? as u32) << 8
            | (spi.recv_exchange()? as u32);
        Ok(())
    }
}

impl SendParam for Short<&[u8]> {
    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        use core::convert::TryFrom;

        spi.send_exchange(u8::try_from(self.value.len()).unwrap())?;
        for &byte in self.value {
            spi.send_exchange(byte)?;
        }
        Ok(())
    }
}

impl RecvParam for Short<&mut [u8]> {
    fn recv<S>(&mut self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let len = spi.recv_exchange()?;
        for i in 0..len {
            self.value[i as usize] = spi.recv_exchange()?;
        }
        self.recv_len = len;
        Ok(())
    }
}

impl SendParam for Long<&[u8]> {
    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        for &byte in self.value {
            spi.send_exchange(byte)?;
        }
        Ok(())
    }
}

impl RecvParam for Long<&mut [u8]> {
    fn recv<S>(&mut self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let len = (spi.recv_exchange()? as u16) << 8 | (spi.recv_exchange()? as u16);
        for i in 0..len {
            self.value[i as usize] = spi.recv_exchange()?;
        }
        self.recv_len = len;
        Ok(())
    }
}

impl<A> Short<A> {
    pub fn new(value: A) -> Self {
        let recv_len = 0;
        Self { recv_len, value }
    }
}

impl<A> Long<A> {
    pub fn new(value: A) -> Self {
        let recv_len = 0;
        Self { recv_len, value }
    }
}

impl<A> core::ops::Deref for Short<A> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<A> core::ops::Deref for Long<A> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<A> core::ops::DerefMut for Short<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<A> core::ops::DerefMut for Long<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
