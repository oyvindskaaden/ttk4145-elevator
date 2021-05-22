
mod modules {
    pub mod elevio;
}

use modules::elevio::*;
use std::net;
use crossbeam_channel as cbc;
use std::thread::{spawn, sleep};
use std::time::Duration;
use std::sync::Arc;

fn main() {
    let num_floors = 4;
    let localhost = net::IpAddr::V4(net::Ipv4Addr::new(127,0,0,1));
    let server_port = 15657;
    let poll_period = Duration::from_millis(15);


    let elev_ip = net::SocketAddr::new(localhost, server_port);
    
    let elev_io = Arc::new(ElevIO::init(elev_ip, num_floors).unwrap());

    let (elevio_tx, elevio_rx) = cbc::unbounded::<ElevRet>();

    {
        let (elev_io, elevio_sender) = (elev_io.clone(), elevio_tx.clone());
        spawn(move || {
            elev_io.poll_order_buttons(elevio_sender, poll_period);
        });
    }

    {
        let (elev_io, elevio_sender) = (elev_io.clone(), elevio_tx.clone());
        spawn(move || {
            elev_io.poll_floor_sensors(elevio_sender, poll_period);
        });
    }

    {
        let (elev_io, elevio_sender) = (elev_io.clone(), elevio_tx.clone());
        spawn(move || {
            elev_io.poll_stop_button(elevio_sender, poll_period);
        });
    }

    {
        let (elev_io, elevio_sender) = (elev_io.clone(), elevio_tx.clone());
        spawn(move || {
            elev_io.poll_is_obstructed(elevio_sender, poll_period);
        });
    }

    let mut dirn = MotorDirection::Down;

    if elev_io.get_floor_sensor().unwrap().is_none() {
        elev_io.set_motor_dir(dirn).unwrap();
    }

    loop {
        cbc::select! {
            recv(elevio_rx) -> elev_sensor => {
                match elev_sensor.unwrap() {
                    ElevRet::OrderButton(floor, button) => {
                        println!("Button {:?}, pressed at floor {}", button, floor);
                        elev_io.set_call_button_light(button, floor, true).unwrap();
                    },
                    ElevRet::FloorSensor(floor) => {
                        println!("Floor: {:#?}", floor);
                        dirn = if floor == 0 {
                            MotorDirection::Up
                        } else if floor == num_floors-1 {
                            MotorDirection::Down
                        } else {
                            dirn
                        };
                        elev_io.set_motor_dir(dirn).unwrap();
                    },
                    ElevRet::StopButton(on) => {
                        println!("Stop button: {}", on);
                    },
                    ElevRet::Obstuction(obstructed) => {
                        println!("Obstruction: {}", obstructed);
                    }
                }
            }
        }
    }
}
