use std::fmt;
use std::net;
use std::io::*;
use std::sync::*;

use crate::driver::poll::ButtonType;

struct Elevator{
    socket: Arc<Mutex<net::TcpStream>>,
    pub num_floors: u8,
}

/// Instructions to set values to the elevator
#[repr(u8)]
enum SetElevator {
    Reload,
    MotorDirection(MotorDirection),
    CallButtonLight(ButtonType, u8, bool),
    FloorIndicator(u8),
    DoorOpenLight(bool),
    StopButtonLight(bool)
}

#[repr(u8)]
enum GetElevator {
    OrderButton(ButtonType, u8),
    StopButton,
    Obstuction
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

    pub fn set(&self, inst: SetElevator) -> Result<()>{
        let message = match inst {
            SetElevator::Reload                             => [0, 0, 0, 0],
            SetElevator::MotorDirection(dir)                => [1, dir as u8, 0, 0],
            SetElevator::CallButtonLight(button, floor, on) => [2, button as u8, floor as u8, on as u8],
            SetElevator::FloorIndicator(floor)              => [3, floor as u8, 0, 0],
            SetElevator::DoorOpenLight(on)                  => [4, on as u8, 0, 0],
            SetElevator::StopButtonLight(on)                => [5, on as u8, 0, 0]
        };

        let mut sock = self.socket.lock().unwrap();
        sock.write(&message).unwrap();
        
        Ok(())
    }

    pub fn get_bool(&self, inst: GetElevator) -> Result<bool> {
        let mut message = match inst {
            GetElevator::OrderButton(button, floor) => [6, button as u8, floor, 0],
            GetElevator::StopButton                 => [8, 0, 0, 0],
            GetElevator::Obstuction                 => [9, 0, 0, 0]
        };

        let mut sock = self.socket.lock().unwrap();
        sock.write(&message).unwrap();
        sock.read(&mut message)?;//.unwrap()

        Ok(message[1] != 0)
    }

    pub fn get_floor_sensor(&self) -> Option<u8> {
        let mut message = [7, 0, 0, 0];

        let mut sock = self.socket.lock().unwrap();
        sock.write(&message).unwrap();
        sock.read(&mut message).unwrap();
        
        if message[1] != 0 {
            Some(message[1])
        } else {
            None
        }
    }
}