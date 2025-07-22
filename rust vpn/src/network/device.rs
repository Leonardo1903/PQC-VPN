use std::io;
use tokio::io::{AsyncRead, AsyncWrite};
use tun::platform::Device as PlatformDevice;
use tun::{Configuration, TunPacket};

pub struct TunDevice {
    device: PlatformDevice,
}

impl TunDevice {
    pub fn new(name: &str, mtu: i32) -> io::Result<Self> {
        let mut config = Configuration::default();
        config
            .name(name)
            .mtu(mtu)
            .up()
            .platform(|config| {
                config.packet_information(true);
            });

        let device = tun::create(&config)?;
        Ok(Self { device })
    }

    pub async fn read_packet(&mut self) -> io::Result<TunPacket> {
        let mut buf = vec![0u8; 1504]; // MTU + headers
        let size = self.device.read(&mut buf).await?;
        buf.truncate(size);
        Ok(TunPacket::new(buf))
    }

    pub async fn write_packet(&mut self, packet: TunPacket) -> io::Result<()> {
        self.device.write_all(packet.get_bytes()).await
    }
}

impl AsyncRead for TunDevice {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        // Delegate to inner device
        AsyncRead::poll_read(std::pin::Pin::new(&mut self.get_mut().device), cx, buf)
    }
}

impl AsyncWrite for TunDevice {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<io::Result<usize>> {
        AsyncWrite::poll_write(std::pin::Pin::new(&mut self.get_mut().device), cx, buf)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        AsyncWrite::poll_flush(std::pin::Pin::new(&mut self.get_mut().device), cx)
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        AsyncWrite::poll_shutdown(std::pin::Pin::new(&mut self.get_mut().device), cx)
    }
}
