//use std::fmt;
use std::net;
use std::io::*;
use std::sync::*;
use std::thread;
use std::time;

use crossbeam_channel as cbc;

/// Enum representing a button
#[derive(Copy, Debug, Clone)]
#[repr(u8)]
pub enum ButtonType{
    Up,
    Down,
    Cab
}

/// Enum for motor direction
#[repr(u8)]
pub enum MotorDirection {
    Stop,
    Up,
    Down = u8::MAX,
}

/// Enum for return value of elevator sensors
pub enum ElevRet {
    OrderButton(u8, ButtonType),
    FloorSensor(u8),
    StopButton(bool),
    Obstuction(bool)
}

/** Elevator Input output type
 * The number of floors is stored in `num_floors`
 * The `socket` is have a Arc-Mutex arround so multiple threads can use the same socket.*/ 
pub struct ElevIO{
    socket: Arc<Mutex<net::TcpStream>>,
    num_floors: u8,
}

impl ElevIO {

    /// The init function initialize the `ElevIO` type.
    /// `addr` is a `std::net::SocketAddr` which contais the IP-address.
    /// `num_floors` contains the number of floors in the elevator.
    pub fn init(addr: net::SocketAddr, num_floors: u8) -> Result<ElevIO> {
        Ok(Self {
            socket: Arc::new(Mutex::new(net::TcpStream::connect(addr)?)),
            num_floors
        })
    }

    fn set(&self, message: [u8; 4]) -> Result<()>{
        let mut sock = self.socket.lock().expect("Could not lock elevator socket mutex!");
        sock.write_all(&message)?;
        
        Ok(())
    }

    fn get(&self,message: [u8; 4]) -> Result<[u8; 4]> {
        let mut sock = self.socket.lock().expect("Could not lock elevator socket mutex!");
        let mut read_elev: [u8; 4] = [0;4];
        
        sock.write_all(&message)?;
        sock.read_exact(&mut read_elev)?;
        Ok(read_elev)
    }

    pub fn set_motor_dir(&self, dir: MotorDirection)                                -> Result<()> { self.set([1, dir as u8, 0, 0]) }
    pub fn set_call_button_light(&self, button: ButtonType, floor: u8, on: bool)    -> Result<()> { self.set([2, button as u8, floor as u8, on as u8]) }
    pub fn set_floor_indicator_light(&self, floor: u8)                              -> Result<()> { self.set([3, floor , 0, 0]) }
    pub fn set_door_open_light(&self, on: bool)                                     -> Result<()> { self.set([4, on as u8, 0, 0]) }
    pub fn set_stop_button_light(&self, on: bool)                                   -> Result<()> { self.set([5, on as u8, 0, 0]) }

    pub fn get_order_button(&self, button: ButtonType, floor: u8) -> Result<bool> { 
        Ok(self.get([6, button as u8, floor, 0])?[1] != 0)
    }

    pub fn get_stop_button(&self) -> Result<bool> { 
        Ok(self.get([8, 0, 0, 0])?[1] != 0)
    }

    pub fn get_is_obstuction(&self) -> Result<bool> { 
        Ok(self.get([8, 0, 0, 0])?[1] != 0)
    }

    pub fn get_floor_sensor(&self) -> Result<Option<u8>> {
        let message = self.get([7, 0, 0, 0])?;

        Ok( if message[1] != 0 {
            Some(message[2])
        } else {
            None
        })
    }

    pub fn poll_order_buttons(&self, ch: cbc::Sender<ElevRet>, poll_period: time::Duration) {
        let mut prev = vec![[false; 3]; self.num_floors.into()];
        loop {
            for floor in 0..self.num_floors {
                for button in &[ButtonType::Up, ButtonType::Down, ButtonType::Cab]{
                    if let Ok(on) = self.get_order_button(*button, floor) {
                        if on && !prev[floor as usize][*button as usize] {
                            ch.send(ElevRet::OrderButton(floor, *button))
                                .expect("Could not send OrderButton over channel")
                        }
                        prev[floor as usize][*button as usize] = on;
                    }
                }
            }
            thread::sleep(poll_period)
        }
    }
}

/*
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
    
}*/