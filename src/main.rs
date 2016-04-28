 #![feature(type_ascription)]

extern crate iron;
extern crate session;

use std::net::{SocketAddr, Ipv4Addr};
use session::{Sessions, SessionStore, HashSessionStore, Session};
use session::sessions::RequestSession;

use iron::{Request, Response, IronResult, Chain, Iron};
//use iron::prelude::*;
use std::boxed::Box;
use std::any::Any;
use iron::status::{Status};
//use iron::typemap;

fn hello_world(req: &mut Request) -> IronResult<Response> {
	// Retrieve our session from the store
    //let session = req.extensions.get_mut::<RequestSession<Box<u32>>>().unwrap();
    let mut count = 0;
    let session = req.extensions.get_mut::<RequestSession<Box<&str>>>();
    match session {
        None => {
        	println!("{}", "session is none")
        	//req.extensions.insert::<RequestSession<Box<&str>>>(Box::new(&*"localhost:3000"))

        	//session = Session<SocketAddr>::new
        },
        Some(v) => {
        	println!("{:?}", 2);
        	
            let countBox : Result<Box<u32>, _> = (v.upsert(Box::new(1u32), count_func)).downcast();
            match countBox {
            	Ok(v2) => count = *v2,
            	Err(e2) => println!("{}", "e2"),
            }
        },
    }
    // Store or increase the sessioned count
    // let count = session.upsert(1u32, |v: Box<u32>| { *v = *v + 1; } );

    //println!("{} hits from\t{}", count, req.remote_addr);

    //Ok(Response::with((iron::status::Ok, format!("Sessioned count: {}", count).as_slice())))
    Ok(Response::with((iron::status::Ok, format!("Sessioned count: {}", "test"))))
}

fn count_func(v: &mut Box<Any + 'static>){	
	*v = Box::new(1u32)
}
// fn count_func(v: &mut Box<Any + 'static>){	
// 	let t = &v;
// 	let vcast : Result<Box<i32>, _> = t.downcast();
// 	let mut val1 = 0;
// 	match vcast {
// 		Ok(val) => val1 = *val + 1,
// 		Err(e) => println!("result: Failed One"),
// 	}
// 	*v = Box::new(val1)
// }

fn id_generator(req: &Request) -> &'static str { "localhost" }

fn main() {
    let mut chain = Chain::new(hello_world);
    chain.link_before(Sessions::new(id_generator, HashSessionStore::<&str, Box<u32>>::new()));
    let mut server = Iron::new(chain);
    server.http("localhost:3000");
}