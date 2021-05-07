use crossbeam_channel as cbc;

use std::time;

pub enum ButtonCall{
    Up,
    Down,
    Cab
}


enum DriverMessage {
    CallButton(u8, ButtonCall),
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