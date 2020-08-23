use super::full_duplex::FullDuplexExt as _;
use crate::encoding;
use core::marker;

pub trait SendParam {
    fn len(&self) -> usize;

    fn len_length_delimited(&self, long: bool) -> usize {
        self.len() + if long { 2 } else { 1 }
    }

    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>;

    fn send_length_delimited<S>(&self, spi: &mut S, long: bool) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        encoding::send_len(spi, long, self.len())?;
        self.send(spi)
    }
}

pub trait RecvParam {
    fn recv<S>(&mut self, spi: &mut S, len: usize) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>;

    fn recv_length_delimited<S>(&mut self, spi: &mut S, long: bool) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let len = encoding::recv_len(spi, long)?;
        self.recv(spi, len)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct NullTerminated<A>(A)
where
    A: ?Sized;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct Scalar<O, A>
where
    A: ?Sized,
{
    phantom: marker::PhantomData<O>,
    value: A,
}

impl<A> SendParam for &A
where
    A: SendParam + ?Sized,
{
    fn len(&self) -> usize {
        (*self).len()
    }

    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        (*self).send(spi)
    }
}

impl<A> RecvParam for &mut A
where
    A: RecvParam + ?Sized,
{
    fn recv<S>(&mut self, spi: &mut S, len: usize) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        (*self).recv(spi, len)
    }
}

impl SendParam for u8 {
    fn len(&self) -> usize {
        1
    }

    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        spi.send_exchange(*self)?;
        Ok(())
    }
}

impl RecvParam for u8 {
    fn recv<S>(&mut self, spi: &mut S, len: usize) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        assert_eq!(1, len);
        *self = spi.recv_exchange()?;
        Ok(())
    }
}

impl<O> SendParam for Scalar<O, u16>
where
    O: byteorder::ByteOrder,
{
    fn len(&self) -> usize {
        2
    }

    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let mut buf = [0; 2];
        O::write_u16(&mut buf, self.value);
        spi.send_exchange(buf[0])?;
        spi.send_exchange(buf[1])?;
        Ok(())
    }
}

impl<O> RecvParam for Scalar<O, u16>
where
    O: byteorder::ByteOrder,
{
    fn recv<S>(&mut self, spi: &mut S, len: usize) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        assert_eq!(2, len);
        let mut buf = [0; 2];
        buf[0] = spi.recv_exchange()?;
        buf[1] = spi.recv_exchange()?;
        self.value = O::read_u16(&buf);
        Ok(())
    }
}

impl<O> SendParam for Scalar<O, u32>
where
    O: byteorder::ByteOrder,
{
    fn len(&self) -> usize {
        4
    }

    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let mut buf = [0; 4];
        O::write_u32(&mut buf, self.value);
        spi.send_exchange(buf[0])?;
        spi.send_exchange(buf[1])?;
        spi.send_exchange(buf[2])?;
        spi.send_exchange(buf[3])?;
        Ok(())
    }
}

impl<O> RecvParam for Scalar<O, u32>
where
    O: byteorder::ByteOrder,
{
    fn recv<S>(&mut self, spi: &mut S, len: usize) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        assert_eq!(4, len);
        let mut buf = [0; 4];
        buf[0] = spi.recv_exchange()?;
        buf[1] = spi.recv_exchange()?;
        buf[2] = spi.recv_exchange()?;
        buf[3] = spi.recv_exchange()?;
        self.value = O::read_u32(&buf);
        Ok(())
    }
}

impl SendParam for [u8] {
    fn len(&self) -> usize {
        self.len()
    }

    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        for &byte in self.iter() {
            spi.send_exchange(byte)?;
        }

        Ok(())
    }
}

impl<A> SendParam for arrayvec::ArrayVec<A>
where
    A: arrayvec::Array<Item = u8>,
{
    fn len(&self) -> usize {
        self.len()
    }

    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        SendParam::send(self.as_slice(), spi)
    }
}

impl RecvParam for &mut [u8] {
    fn recv<S>(&mut self, spi: &mut S, len: usize) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        use core::mem;

        for i in 0..len {
            self[i] = spi.recv_exchange()?;
        }

        let slice = mem::replace(self, &mut []);
        *self = &mut slice[..len as usize];

        Ok(())
    }
}

impl<A> RecvParam for arrayvec::ArrayVec<A>
where
    A: arrayvec::Array<Item = u8>,
{
    fn recv<S>(&mut self, spi: &mut S, len: usize) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        for _ in 0..len {
            self.push(spi.recv_exchange()?);
        }

        Ok(())
    }
}

impl<A> SendParam for NullTerminated<A>
where
    A: SendParam,
{
    fn len(&self) -> usize {
        self.0.len() + 1
    }

    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        self.0.send(spi)?;
        spi.send_exchange(0)?;
        Ok(())
    }
}

impl<A> RecvParam for NullTerminated<A>
where
    A: RecvParam,
{
    fn recv<S>(&mut self, spi: &mut S, len: usize) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        self.0.recv(spi, len - 1)?;
        assert_eq!(0, spi.recv_exchange()?);
        Ok(())
    }
}

impl<A> NullTerminated<A> {
    pub fn new(value: A) -> Self {
        Self(value)
    }

    pub fn into_inner(self) -> A {
        self.0
    }
}

impl<A> Scalar<byteorder::BigEndian, A> {
    pub fn be(value: A) -> Self {
        let phantom = marker::PhantomData;
        Self { value, phantom }
    }
}

impl<A> Scalar<byteorder::LittleEndian, A> {
    pub fn le(value: A) -> Self {
        let phantom = marker::PhantomData;
        Self { value, phantom }
    }
}

impl<O, A> Scalar<O, A> {
    pub fn into_inner(self) -> A {
        self.value
    }
}

impl<A> core::ops::Deref for NullTerminated<A> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<A> core::ops::DerefMut for NullTerminated<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<O, A> core::ops::Deref for Scalar<O, A> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<O, A> core::ops::DerefMut for Scalar<O, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
