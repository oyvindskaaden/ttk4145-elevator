use crossbeam_channel as cbc;

use std::time;

pub enum ButtonType{
    Up,
    Down,
    Cab
}


enum DriverMessage {
    CallButton(u8, ButtonType),
    FloorSensor(u8),
    StopButton(bool),
    Obstruction(bool),
}

pub fn poll_elevator(
    elevator:               elev::Elevator,
    elevator_poll_sender:   cbc::Sender<DriverMessage>,
    poll_period:            time::Duration
){
    
}