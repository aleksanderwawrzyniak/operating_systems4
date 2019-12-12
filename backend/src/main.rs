#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod connections;
mod frames;

use connections::{OutputRequest, Request};
use frames::FrameSimulator;

use rocket::Data;
use rocket_contrib::json::Json;
use std::io::Read;
// use serde_json::{Json, Value};

#[post(
    "/equal",
    format = "text/plain; charset=UTF-8",
    data = "<plain_request>"
)]
fn equal(plain_request: Data) -> Json<OutputRequest> {
    let string_request = match data_to_string(plain_request) {
        Ok(s) => s,
        Err(_) => {
            return Json(OutputRequest::error(String::from(
                "Error, cannot read the request",
            )))
        }
    };
    println!("{}", string_request);
    let request: Request = serde_json::from_str(&string_request).unwrap();

    let mut sim = FrameSimulator::from_request(&request);
    let algorithm = match request.get_algorithm() {
        Ok(alg) => alg,
        Err(s) => {
            return Json(OutputRequest::new_error(s, &request));
        }
    };

    sim.simulate_equal(algorithm);

    let output = OutputRequest::new_good(&sim, &request);
    println!("{:?}", output);

    Json(output)
}

#[post(
    "/proportional",
    format = "text/plain; charset=UTF-8",
    data = "<plain_request>"
)]
fn proportional(plain_request: Data) -> Json<OutputRequest> {
    let string_request = match data_to_string(plain_request) {
        Ok(s) => s,
        Err(_) => {
            return Json(OutputRequest::error(String::from(
                "Error, cannot read the request",
            )))
        }
    };
    println!("{}", string_request);
    let request: Request = serde_json::from_str(&string_request).unwrap();

    let mut sim = FrameSimulator::from_request(&request);
    let algorithm = match request.get_algorithm() {
        Ok(alg) => alg,
        Err(s) => {
            return Json(OutputRequest::new_error(s, &request));
        }
    };

    sim.simulate_proportional(algorithm);

    let output = OutputRequest::new_good(&sim, &request);
    println!("{:?}", output);

    Json(output)
}

#[post(
    "/random",
    format = "text/plain; charset=UTF-8",
    data = "<plain_request>"
)]
fn random(plain_request: Data) -> Json<OutputRequest> {
    let string_request = match data_to_string(plain_request) {
        Ok(s) => s,
        Err(_) => {
            return Json(OutputRequest::error(String::from(
                "Error, cannot read the request",
            )))
        }
    };
    println!("{}", string_request);
    let request: Request = serde_json::from_str(&string_request).unwrap();

    let mut sim = FrameSimulator::from_request(&request);
    let algorithm = match request.get_algorithm() {
        Ok(alg) => alg,
        Err(s) => {
            return Json(OutputRequest::new_error(s, &request));
        }
    };

    sim.simulate_rand(algorithm);

    let output = OutputRequest::new_good(&sim, &request);
    println!("{:?}", output);

    Json(output)
}

#[post("/pff", format = "text/plain; charset=UTF-8", data = "<plain_request>")]
fn pff(plain_request: Data) -> Json<OutputRequest> {
    let string_request = match data_to_string(plain_request) {
        Ok(s) => s,
        Err(_) => {
            return Json(OutputRequest::error(String::from(
                "Error, cannot read the request",
            )))
        }
    };
    println!("{}", string_request);
    let request: Request = serde_json::from_str(&string_request).unwrap();

    let mut sim = FrameSimulator::from_request(&request);
    let algorithm = match request.get_algorithm() {
        Ok(alg) => alg,
        Err(s) => {
            return Json(OutputRequest::new_error(s, &request));
        }
    };

    sim.simulate_pff(algorithm);

    let output = OutputRequest::new_good(&sim, &request);
    println!("{:?}", output);

    Json(output)
}

#[post("/wsa", format = "text/plain; charset=UTF-8", data = "<plain_request>")]
fn wsa(plain_request: Data) -> Json<OutputRequest> {
    let string_request = match data_to_string(plain_request) {
        Ok(s) => s,
        Err(_) => {
            return Json(OutputRequest::error(String::from(
                "Error, cannot read the request",
            )))
        }
    };
    println!("{}", string_request);
    let request: Request = serde_json::from_str(&string_request).unwrap();

    let mut sim = FrameSimulator::from_request(&request);
    let algorithm = match request.get_algorithm() {
        Ok(alg) => alg,
        Err(s) => {
            return Json(OutputRequest::new_error(s, &request));
        }
    };

    sim.simulate_wsa(algorithm);

    let output = OutputRequest::new_good(&sim, &request);
    println!("{:?}", output);

    Json(output)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![equal, proportional, random, pff, wsa])
        .launch();
}

fn data_to_string(data: Data) -> std::io::Result<String> {
    let mut buffer = String::new();
    data.open().read_to_string(&mut buffer)?;

    Ok(buffer)
}
