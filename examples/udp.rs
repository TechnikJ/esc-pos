use std::{net::UdpSocket, io::{Write, Result}};

use esc_pos::{Printer, type_state::T88IV};

struct UdpSocketWrapper(UdpSocket);

impl Write for UdpSocketWrapper {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.send(buf)
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:3400").unwrap();
    socket.connect("172.28.0.50:4210").unwrap();
    let mut printer = Printer::new(UdpSocketWrapper(socket), T88IV);
    
    
    printer.paper_cut(0x41, 0x05);
}