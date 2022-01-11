use std::sync::{Arc, Mutex};
use std::io::Read;
use iron::{status, AfterMiddleware, Handler, IronResult, Request, Response};
use iron::headers::ContentType;
use rustc_serialize::json;

use database::Database;
use uuid::Uuid;
use router::Router;
use models::Post;
use std::error:Error;

macro_rules! try_handler {
    ($e: expr) => {
        match $e{
            Ok(x) => x,
            Err(e) => return Ok(Response::with((status::InternalServerError, e.description())))
        }
    };

    ($e: expr, $error: expr) => {
        match $e{
            Ok(x) => x,  
            Err(e) => return Ok(Response::with(($error, e.description())))
        }
    }
}

macro_rules! lock {
    ($e: expr) => { e.lock().unwrap() };    
}

macro_rules! get_http_param {
    ($r: expr, $e: expr) => {
        
    };
}