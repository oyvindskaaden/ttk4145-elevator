//use std::fmt;
use std::net;
use std::io::*;
use std::sync::*;
use std::thread;

use crossbeam_channel as cbc;

/// Enum representing a button
#[repr(u8)]
pub enum ButtonType{
    Up,
    Down,
    Cab
}

/// Instructions to set values to the elevator
#[repr(u8)]
pub enum SetElevIO {
    Reload,
    MotorDirection(MotorDirection),
    CallButtonLight(ButtonType, u8, bool),
    FloorIndicator(u8),
    DoorOpenLight(bool),
    StopButtonLight(bool)
}

pub enum GetElevIO {
    OrderButton(u8, ButtonType),
    FloorSensor(u8),
    StopButton(bool),
    Obstuction(bool)
}

#[repr(u8)]
enum MotorDirection {
    Stop,
    Up,
    Down = u8::MAX,
}

pub struct ElevIO{
    socket: Arc<Mutex<net::TcpStream>>,
    pub num_floors: u8,
}

impl ElevIO {
    pub fn init(addr: net::SocketAddr, num_floors: u8) -> Result<ElevIO> {
        Ok(Self {
            socket: Arc::new(Mutex::new(net::TcpStream::connect(addr)?)),
            num_floors
        })
    }

    pub fn set(&self, inst: SetElevIO) -> Result<()>{
        let message = match inst {
            SetElevIO::Reload                             => [0, 0, 0, 0],
            SetElevIO::MotorDirection(dir)                => [1, dir as u8, 0, 0],
            SetElevIO::CallButtonLight(button, floor, on) => [2, button as u8, floor as u8, on as u8],
            SetElevIO::FloorIndicator(floor)              => [3, floor as u8, 0, 0],
            SetElevIO::DoorOpenLight(on)                  => [4, on as u8, 0, 0],
            SetElevIO::StopButtonLight(on)                => [5, on as u8, 0, 0]
        };

        let mut sock = self.socket.lock().expect("Could not lock elevator socket mutex!");
        sock.write_all(&message)?;
        
        Ok(())
    }

    pub fn get_bool(&self, inst: GetElevIO) -> Result<bool> {
        let mut message = match inst {
            GetElevIO::OrderButton(floor, button) => [6, button as u8, floor, 0],
            GetElevIO::StopButton(_)              => [8, 0, 0, 0],
            GetElevIO::Obstuction(_)              => [9, 0, 0, 0]
        };

        let mut sock = self.socket.lock().expect("Could not lock elevator socket mutex!");
        sock.write_all(&message)?;
        sock.read_exact(&mut message)?;

        Ok(message[1] != 0)
    }

    pub fn get_floor_sensor(&self) -> Result<Option<ElevIOMessage>> {
        let mut message = [7, 0, 0, 0];

        let mut sock = self.socket.lock().expect("Could not lock elevator socket mutex!");
        sock.write_all(&message)?;
        sock.read_exact(&mut message)?;
        
        if message[1] != 0 {
            Ok(Some(ElevIOMessage::FloorSensor(message[2])))
        } else {
            Ok(None)
        }
    }

    pub fn pull(elevator_poll_sender: cbc::Sender<ElevIOMessage>, poll_period: time::Duration) {
        let mut pollers: Vec<thread::JoinHandle<_>> = Vec::new();
    }
}




use std::time;




pub enum ElevIOMessage {
    CallButton(u8, ButtonType),
    FloorSensor(u8),
    StopButton(bool),
    Obstruction(bool),
}

pub fn poll_elevator(
    elevator:               ElevIO,
    elevator_poll_sender:   cbc::Sender<ElevIOMessage>,
    poll_period:            time::Duration
){
    
}