extern crate discord;

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use bot_module::BotModule;

use discord::Discord;
use discord::Connection;
use discord::model::Event;

pub struct Everett {
	swears: Vec<String>
}

impl Everett {
	pub fn new() -> Everett {
		Everett {
			swears: match Everett::load("res/swears.txt".to_string()) {
				Ok(words) => {
					words
				}
				Err(error) => {
					println!("Error loading words: {}", error);
					Vec::new()
				}
			}
		}
	}
	
	fn load(file_name: String) -> Result<Vec<String>, io::Error> {
		let file = try!(File::open(file_name));
		let reader = BufReader::new(&file);
		
		let mut words = Vec::new();
		
		for word in reader.lines() {
			match word {
				Ok(val) => {
					words.push(val);
				}
				_ => { }
			}
		}
		
		Result::Ok(words)
	}
}

impl BotModule for Everett {
	fn handle(&mut self, discord: &Discord, _: &Connection, event: &Event) {
		match *event {
			Event::MessageCreate(ref message) => {
				let lower = message.content.to_lowercase();
				for swear in &self.swears {
					if lower.contains(swear) {
						let _ = discord.delete_message(&message.channel_id, &message.id);
						let _ = discord.send_message(&message.channel_id,
							&*format!("{} Hey! Expand your vocabulary!", message.author.mention()), "", false);
						break;
					}
				}
			}
			_ => {}
		}
	}
}
