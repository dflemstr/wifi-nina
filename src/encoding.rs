pub fn recv_len<S>(spi: &mut S, long: bool) -> Result<usize, S::Error>
where
    S: embedded_hal::spi::FullDuplex<u8>,
{
    use super::full_duplex::FullDuplexExt as _;
    use byteorder::ByteOrder as _;

    let len = if long {
        let mut buf = [0; 2];
        buf[0] = spi.recv_exchange()?;
        buf[1] = spi.recv_exchange()?;
        byteorder::BigEndian::read_u16(&buf) as usize
    } else {
        spi.recv_exchange()? as usize
    };

    Ok(len)
}

pub fn send_len<S>(spi: &mut S, long: bool, len: usize) -> Result<(), S::Error>
where
    S: embedded_hal::spi::FullDuplex<u8>,
{
    use super::full_duplex::FullDuplexExt as _;
    use byteorder::ByteOrder as _;
    use core::convert::TryFrom;

    if long {
        let len = u16::try_from(len).unwrap();
        let mut buf = [0; 2];
        byteorder::BigEndian::write_u16(&mut buf, len);
        spi.send_exchange(buf[0])?;
        spi.send_exchange(buf[1])?;
    } else {
        let len = u8::try_from(len).unwrap();
        spi.send_exchange(len)?;
    }

    Ok(())
}
