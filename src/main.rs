extern crate discord;
extern crate regex;

mod bot_module;
mod command;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use bot_module::BotModule;
use bot_module::connect4::Connect4;
use bot_module::everett::Everett;
use discord::Discord;

fn load_token() -> Result<String, io::Error> {
	let mut file = try!(File::open("res/token.txt"));
	let mut token = String::new();
	
	try!(file.read_to_string(&mut token));
	
	Result::Ok(token)
}

fn main() {
	let discord = match load_token() {
		Ok(token) => {
			Discord::from_bot_token(&token).expect("Invalid token.")
		}
		Err(error) => {
			panic!("{}", error)
		}
	};
	
	let (mut connection, _) = discord.connect().expect("Connection failed.");
	
	let mut modules: Vec<Box<BotModule>> = Vec::new();
	modules.push(Box::new(Connect4::new()));
	modules.push(Box::new(Everett::new()));
	
	loop {
		if let Ok(event) = connection.recv_event() {
			for m in &mut modules {
				m.handle(&discord, &connection, &event);
			}
		} else {
			println!("Could not receive event.");
		}
	}
}
