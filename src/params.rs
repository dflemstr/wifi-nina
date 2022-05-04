use super::full_duplex::FullDuplexExt as _;
use super::param;
use crate::param::SendParam;

pub trait SendParams {
    fn len(&self, long: bool) -> usize {
        self.param_len(long) + 1
    }

    fn param_len(&self, long: bool) -> usize;

    fn send<S>(&self, spi: &mut S, long: bool) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>;
}

pub trait RecvParams {
    fn recv<S>(&mut self, spi: &mut S, long: bool) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>;
}

impl SendParams for () {
    fn param_len(&self, _long: bool) -> usize {
        0
    }

    fn send<S>(&self, spi: &mut S, _long: bool) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        spi.send_exchange(0)?;
        Ok(())
    }
}

impl RecvParams for () {
    fn recv<S>(&mut self, spi: &mut S, _long: bool) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        assert_eq!(0, spi.recv_exchange()?);
        Ok(())
    }
}

impl<A> SendParams for (A,)
where
    A: param::SendParam,
{
    fn param_len(&self, long: bool) -> usize {
        let (a,) = self;
        a.len_length_delimited(long)
    }

    fn send<S>(&self, spi: &mut S, long: bool) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a,) = self;
        spi.send_exchange(1)?;
        log::trace!("param 0");
        a.send_length_delimited(spi, long)?;
        log::trace!("end");
        Ok(())
    }
}

impl<A> RecvParams for (A,)
where
    A: param::RecvParam,
{
    fn recv<S>(&mut self, spi: &mut S, long: bool) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a,) = self;
        assert_eq!(1, spi.recv_exchange()?);
        log::trace!("param 0");
        a.recv_length_delimited(spi, long)?;
        log::trace!("end");
        Ok(())
    }
}

impl<A, B> SendParams for (A, B)
where
    A: param::SendParam,
    B: param::SendParam,
{
    fn param_len(&self, long: bool) -> usize {
        let (a, b) = self;
        a.len_length_delimited(long) + b.len_length_delimited(long)
    }
    fn send<S>(&self, spi: &mut S, long: bool) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a, b) = self;
        spi.send_exchange(2)?;
        log::trace!("param 0");
        a.send_length_delimited(spi, long)?;
        log::trace!("param 1");
        b.send_length_delimited(spi, long)?;
        log::trace!("end");
        Ok(())
    }
}

impl<A, B> RecvParams for (A, B)
where
    A: param::RecvParam,
    B: param::RecvParam,
{
    fn recv<S>(&mut self, spi: &mut S, long: bool) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a, b) = self;
        assert_eq!(2, spi.recv_exchange()?);
        log::trace!("param 0");
        a.recv_length_delimited(spi, long)?;
        log::trace!("param 1");
        b.recv_length_delimited(spi, long)?;
        log::trace!("end");
        Ok(())
    }
}

impl<A, B, C> SendParams for (A, B, C)
where
    A: param::SendParam,
    B: param::SendParam,
    C: param::SendParam,
{
    fn param_len(&self, long: bool) -> usize {
        let (a, b, c) = self;
        a.len_length_delimited(long) + b.len_length_delimited(long) + c.len_length_delimited(long)
    }

    fn send<S>(&self, spi: &mut S, long: bool) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a, b, c) = self;
        spi.send_exchange(3)?;
        log::trace!("param 0");
        a.send_length_delimited(spi, long)?;
        log::trace!("param 1");
        b.send_length_delimited(spi, long)?;
        log::trace!("param 2");
        c.send_length_delimited(spi, long)?;
        log::trace!("end");
        Ok(())
    }
}

impl<A, B, C> RecvParams for (A, B, C)
where
    A: param::RecvParam,
    B: param::RecvParam,
    C: param::RecvParam,
{
    fn recv<S>(&mut self, spi: &mut S, long: bool) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a, b, c) = self;
        assert_eq!(3, spi.recv_exchange()?);
        log::trace!("param 0");
        a.recv_length_delimited(spi, long)?;
        log::trace!("param 1");
        b.recv_length_delimited(spi, long)?;
        log::trace!("param 2");
        c.recv_length_delimited(spi, long)?;
        log::trace!("end");
        Ok(())
    }
}

impl<A, B, C, D> SendParams for (A, B, C, D)
where
    A: param::SendParam,
    B: param::SendParam,
    C: param::SendParam,
    D: param::SendParam,
{
    fn param_len(&self, long: bool) -> usize {
        let (a, b, c, d) = self;
        a.len_length_delimited(long)
            + b.len_length_delimited(long)
            + c.len_length_delimited(long)
            + d.len_length_delimited(long)
    }

    fn send<S>(&self, spi: &mut S, long: bool) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a, b, c, d) = self;
        spi.send_exchange(4)?;
        log::trace!("param 0");
        a.send_length_delimited(spi, long)?;
        log::trace!("param 1");
        b.send_length_delimited(spi, long)?;
        log::trace!("param 2");
        c.send_length_delimited(spi, long)?;
        log::trace!("param 3");
        d.send_length_delimited(spi, long)?;
        log::trace!("end");
        Ok(())
    }
}

impl<A, B, C, D> RecvParams for (A, B, C, D)
where
    A: param::RecvParam,
    B: param::RecvParam,
    C: param::RecvParam,
    D: param::RecvParam,
{
    fn recv<S>(&mut self, spi: &mut S, long: bool) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a, b, c, d) = self;
        assert_eq!(4, spi.recv_exchange()?);
        log::trace!("param 0");
        a.recv_length_delimited(spi, long)?;
        log::trace!("param 1");
        b.recv_length_delimited(spi, long)?;
        log::trace!("param 2");
        c.recv_length_delimited(spi, long)?;
        log::trace!("param 3");
        d.recv_length_delimited(spi, long)?;
        log::trace!("end");
        Ok(())
    }
}

impl<A, B, C, D, E> SendParams for (A, B, C, D, E)
where
    A: param::SendParam,
    B: param::SendParam,
    C: param::SendParam,
    D: param::SendParam,
    E: param::SendParam,
{
    fn param_len(&self, long: bool) -> usize {
        let (a, b, c, d, e) = self;
        a.len_length_delimited(long)
            + b.len_length_delimited(long)
            + c.len_length_delimited(long)
            + d.len_length_delimited(long)
            + e.len_length_delimited(long)
    }

    fn send<S>(&self, spi: &mut S, long: bool) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a, b, c, d, e) = self;
        spi.send_exchange(5)?;
        log::trace!("param 0");
        a.send_length_delimited(spi, long)?;
        log::trace!("param 1");
        b.send_length_delimited(spi, long)?;
        log::trace!("param 2");
        c.send_length_delimited(spi, long)?;
        log::trace!("param 3");
        d.send_length_delimited(spi, long)?;
        log::trace!("param 4");
        e.send_length_delimited(spi, long)?;
        log::trace!("end");
        Ok(())
    }
}

impl<A, B, C, D, E> RecvParams for (A, B, C, D, E)
where
    A: param::RecvParam,
    B: param::RecvParam,
    C: param::RecvParam,
    D: param::RecvParam,
    E: param::RecvParam,
{
    fn recv<S>(&mut self, spi: &mut S, long: bool) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a, b, c, d, e) = self;
        assert_eq!(5, spi.recv_exchange()?);
        log::trace!("param 0");
        a.recv_length_delimited(spi, long)?;
        log::trace!("param 1");
        b.recv_length_delimited(spi, long)?;
        log::trace!("param 2");
        c.recv_length_delimited(spi, long)?;
        log::trace!("param 3");
        d.recv_length_delimited(spi, long)?;
        log::trace!("param 4");
        e.recv_length_delimited(spi, long)?;
        log::trace!("end");
        Ok(())
    }
}

impl<A> SendParams for arrayvec::ArrayVec<A>
where
    A: arrayvec::Array,
    A::Item: param::SendParam,
{
    fn param_len(&self, long: bool) -> usize {
        self.iter().map(|p| p.len_length_delimited(long)).sum()
    }

    fn send<S>(&self, spi: &mut S, long: bool) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        use core::convert::TryFrom;

        spi.send_exchange(u8::try_from(self.len()).unwrap())?;
        for (i, item) in self.iter().enumerate() {
            log::trace!("param {}", i);
            item.send_length_delimited(spi, long)?;
        }
        log::trace!("end");
        Ok(())
    }
}

impl<A> RecvParams for arrayvec::ArrayVec<A>
where
    A: arrayvec::Array,
    A::Item: param::RecvParam + Default,
{
    fn recv<S>(&mut self, spi: &mut S, long: bool) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        use crate::param::RecvParam;

        let len = spi.recv_exchange()?;
        for i in 0..len {
            log::trace!("param {}", i);
            let mut item: <A as arrayvec::Array>::Item = Default::default();
            item.recv_length_delimited(spi, long)?;
            self.push(item);
        }
        log::trace!("end");
        Ok(())
    }
}
