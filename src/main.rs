use futures::{
    future::{self, FutureResult},
    Future,
};
use rand::prelude::*;
use std::thread;
use std::time::Duration;
use tokio_core::reactor::Core;
use tokio_service::Service;

use tokio_modbus::prelude::*;

struct MbServer {
    value: u16,
}

impl Service for MbServer {
    type Request = Request;
    type Response = Response;
    type Error = std::io::Error;
    type Future = FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        println!("Request: {:?}", req);

        match req {
//            Request::ReadCoils(_addr, cnt) => {
//                let mut registers = vec![0; cnt as usize];
//                registers[3] = 0x8;
//                let rsp = Response::ReadCoils(registers);
//                future::ok(rsp)
//            }
//            Request::ReadDiscreteInputs(_addr, cnt) => {
//                let mut registers = vec![0; cnt as usize];
//                registers[3] = 0x8;
//                let rsp = Response::ReadDiscreteInputs(registers);
//                future::ok(rsp)
//            }
            Request::ReadHoldingRegisters(addr, cnt) => {
                let length: usize = cnt as usize - 1;
                let mut registers = vec![0; cnt as usize];
                match addr {
                    100 => {
                        let mut rng = rand::thread_rng();
                        let n100: u16 = rng.gen_range(0, 100);
                        registers[length] = 100 + n100;
                        println!("registers 100 : {:?}", registers);
                        let rsp = Response::ReadHoldingRegisters(registers);
                        future::ok(rsp)
                    }
                    200 => {
                        let mut rng = rand::thread_rng();
                        let n200: u16 = rng.gen_range(0, 100);
                        registers[length] = 200 + n200;
                        println!("registers 200: {:?}", registers);
                        let rsp = Response::ReadHoldingRegisters(registers);
                        future::ok(rsp)
                    }
                    _ => {
                        let mut rng = rand::thread_rng();
                        let n1000: u16 = rng.gen_range(0, 100);
                        registers[length] = 1000 + n1000;
                        println!("registers 1000: {:?}", registers);
                        let rsp = Response::ReadHoldingRegisters(registers);
                        future::ok(rsp)
                    }
                }
            }
            Request::ReadInputRegisters(addr, cnt) => {
                let length: usize = cnt as usize - 1;
                let mut registers = vec![0; cnt as usize];
                match addr {
                    100 => {
                        let mut rng = rand::thread_rng();
                        let n100: u16 = rng.gen_range(0, 100);
                        registers[length] = 100 + n100;
                        println!("registers 100 : {:?}", registers);
                        let rsp = Response::ReadHoldingRegisters(registers);
                        future::ok(rsp)
                    }
                    200 => {
                        let mut rng = rand::thread_rng();
                        let n200: u16 = rng.gen_range(0, 100);
                        registers[length] = 200 + n200;
                        println!("registers 200: {:?}", registers);
                        let rsp = Response::ReadHoldingRegisters(registers);
                        future::ok(rsp)
                    }
                    _ => {
                        let mut rng = rand::thread_rng();
                        let n1000: u16 = rng.gen_range(0, 100);
                        registers[length] = 1000 + n1000;
                        println!("registers 1000: {:?}", registers);
                        let rsp = Response::ReadHoldingRegisters(registers);
                        future::ok(rsp)
                    }
                }
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(feature = "tcp")]
fn main() {
    let socket_addr = "0.0.0.0:5502".parse().unwrap();

    println!("Starting up server..., {}", socket_addr);
    tcp::Server::new(socket_addr).serve(|| Ok(MbServer { value: 0 }));
//    let _server = thread::spawn(move || {
//        tcp::Server::new(socket_addr).serve(|| Ok(MbServer));
//    });
    // Give the server some time for stating up
//    thread::sleep(Duration::from_secs(1));
//
//    let mut core = Core::new().unwrap();
//    let handle = core.handle();
//
//    println!("Connecting client...");
//    let task = tcp::connect(&handle, socket_addr).and_then(|ctx| {
//        println!("Reading input registers...");
//        ctx.read_input_registers(0x00, 7).and_then(move |rsp| {
//            println!("The result is '{:?}'", rsp);
//            Ok(())
//        })
//    });
//
//    core.run(task).unwrap();
}

#[cfg(not(feature = "tcp"))]
pub fn main() {
    println!("feature `tcp` is required to run this example");
    std::process::exit(1);
}
