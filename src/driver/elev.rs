use std::fmt;
use std::net;
use std::io::*;
use std::sync::*;

use crate::driver::poll::ButtonCall;

struct Elevator{
    socket: Arc<Mutex<net::TcpStream>>,
    pub num_floors: u8,
}

#[repr(u8)]
enum MotorDirection {
    Stop,
    Up,
    Down = u8::MAX,
}


impl Elevator {
    pub fn init(addr: net::SocketAddr, num_floors: u8) -> Result<Elevator> {
        Ok(Self {
            socket: Arc::new(Mutex::new(net::TcpStream::connect(addr)?)),
            num_floors: num_floors
        })
    }

    pub fn set_motor_direction(&self, dir: MotorDirection) {
        let buf = [1, dir as u8, 0, 0];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&buf).unwrap();
    }

    pub fn set_call_button_light(&self, floor: u8, call: ButtonCall, on: bool) {
        let buf = [2, call as u8, floor, on as u8];
        let mut sock = self.socket.lock().unwrap();
        sock.write(&buf).unwrap();
    }
}