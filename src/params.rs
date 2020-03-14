use super::param;

pub trait SendParams {
    const LEN: u8;

    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>;
}

pub trait RecvParams {
    const LEN: u8;

    fn recv<S>(&mut self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>;
}

impl SendParams for () {
    const LEN: u8 = 0;

    fn send<S>(&self, _spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        Ok(())
    }
}

impl RecvParams for () {
    const LEN: u8 = 0;

    fn recv<S>(&mut self, _spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        Ok(())
    }
}

impl<A> SendParams for (A,)
where
    A: param::SendParam,
{
    const LEN: u8 = 1;

    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a,) = self;
        a.send(spi)?;
        Ok(())
    }
}

impl<A> RecvParams for (A,)
where
    A: param::RecvParam,
{
    const LEN: u8 = 1;

    fn recv<S>(&mut self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a,) = self;
        a.recv(spi)?;
        Ok(())
    }
}

impl<A, B> SendParams for (A, B)
where
    A: param::SendParam,
    B: param::SendParam,
{
    const LEN: u8 = 2;

    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a, b) = self;
        a.send(spi)?;
        b.send(spi)?;
        Ok(())
    }
}

impl<A, B> RecvParams for (A, B)
where
    A: param::RecvParam,
    B: param::RecvParam,
{
    const LEN: u8 = 2;

    fn recv<S>(&mut self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a, b) = self;
        a.recv(spi)?;
        b.recv(spi)?;
        Ok(())
    }
}

impl<A, B, C> SendParams for (A, B, C)
where
    A: param::SendParam,
    B: param::SendParam,
    C: param::SendParam,
{
    const LEN: u8 = 3;

    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a, b, c) = self;
        a.send(spi)?;
        b.send(spi)?;
        c.send(spi)?;
        Ok(())
    }
}

impl<A, B, C> RecvParams for (A, B, C)
where
    A: param::RecvParam,
    B: param::RecvParam,
    C: param::RecvParam,
{
    const LEN: u8 = 3;

    fn recv<S>(&mut self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a, b, c) = self;
        a.recv(spi)?;
        b.recv(spi)?;
        c.recv(spi)?;
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
    const LEN: u8 = 4;

    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a, b, c, d) = self;
        a.send(spi)?;
        b.send(spi)?;
        c.send(spi)?;
        d.send(spi)?;
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
    const LEN: u8 = 4;

    fn recv<S>(&mut self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a, b, c, d) = self;
        a.recv(spi)?;
        b.recv(spi)?;
        c.recv(spi)?;
        d.recv(spi)?;
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
    const LEN: u8 = 5;

    fn send<S>(&self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a, b, c, d, e) = self;
        a.send(spi)?;
        b.send(spi)?;
        c.send(spi)?;
        d.send(spi)?;
        e.send(spi)?;
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
    const LEN: u8 = 5;

    fn recv<S>(&mut self, spi: &mut S) -> Result<(), S::Error>
    where
        S: embedded_hal::spi::FullDuplex<u8>,
    {
        let (a, b, c, d, e) = self;
        a.recv(spi)?;
        b.recv(spi)?;
        c.recv(spi)?;
        d.recv(spi)?;
        e.recv(spi)?;
        Ok(())
    }
}
