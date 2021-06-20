use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

// importing generated gRPC code
use pong_grpc::*;
// importing types for messages
use pong::*;
mod pong;
mod pong_grpc;

use std::env;
use std::sync::Arc;

use futures::executor;
use grpc::ClientStub;
use grpc::ClientStubExt;


// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::pong::{Paddle, Side, Ball};

#[derive(SystemDesc)]
pub struct NetworkPaddleSystem;

impl<'s> System<'s> for NetworkPaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
		ReadStorage<'s, Ball>,
    );

    fn run(&mut self, (mut transforms, paddles, mut balls): Self::SystemData) {
		let left = (&paddles, &transforms).join().filter(|(p, _)| p.side == Side::Left).next().unwrap();

		let name = "k";
		let port = 50051;
		let client_conf = Default::default();
		// create a client
		let client = SayClient::new_plain("192.168.1.189", port, client_conf).unwrap();
		// create request
		let mut req = SayRequest::new();
		req.set_name(name.to_string());
		req.set_networkname(left.1.translation().y);
		if let Some(ball) = (&balls, &mut transforms).join().next() {
			req.set_networkname2(ball.1.translation().y);
			req.set_networkname3(ball.1.translation().x);
		}
		// send the request
		let resp = client
			.send(grpc::RequestOptions::new(), req)
			.drop_metadata();
		// wait for response
		// println!("REsULT: {:?}", executor::block_on(resp).unwrap());
		let response = executor::block_on(resp).unwrap();

		if let Some(ball) = (&balls, &mut transforms).join().next() {
			ball.1.set_translation_y(response.get_message2());
			ball.1.set_translation_x(response.get_message3());
		}

		let right = (&paddles, &mut transforms).join().filter(|(p, _)| p.side == Side::Right).next().unwrap();
		right.1.set_translation_y(response.get_message());
	}
}
