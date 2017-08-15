extern crate discord;

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;
use std::fs::File;

use bot_module::BotModule;
use command::Command;

use discord::Discord;
use discord::Connection;
use discord::model::Event;

use regex::{self, Regex};

pub struct Everett {
	swears: Vec<(String, Regex)>
}

impl Everett {
	pub fn new() -> Everett {
		let mut everett = Everett {
			swears: Vec::new()
		};
		if let Err(error) = everett.load("res/swears.txt") {
			println!("Error loading words: {}", error);
		}
		everett
	}

	fn register(&mut self, word: &str) {
		let regex = Regex::new(
			&format!(r"(^|\s){}($|\s)", regex::escape(&word[..]))[..]).unwrap();
		self.swears.push((word.to_string(), regex));
	}

	fn unregister(&mut self, word: &str) {
		if let Some(index) = self.swears.iter().enumerate()
			.find(|&(_, &(ref w, _))| word == w).map(|(idx, _)| idx)
		{
			self.swears.swap_remove(index);
		}
	}
	
	fn load(&mut self, file_name: &str) -> Result<(), io::Error> {
		let file = try!(File::open(file_name));
		let reader = BufReader::new(&file);
		
		for word in reader.lines() {
			match word {
				Ok(val) => {
					self.register(&val[..]);
				}
				_ => { }
			}
		}
		Ok(())
	}

	fn save(&self, file_name: &str) -> Result<(), io::Error> {
		let file = try!(File::create(file_name));
		let mut writer = BufWriter::new(&file);
		for &(ref word, _) in &self.swears {
			try!(writeln!(writer, "{}", word));
		}
		Ok(())
	}
}

impl BotModule for Everett {
	fn handle(&mut self, discord: &Discord, _: &Connection, event: &Event) {
		match *event {
			Event::MessageCreate(ref message) => {
				let lower = message.content.to_lowercase();
				//check for the register command
				match lower.command() {
					("!register", Some((_, word))) => {
						self.register(word.trim());
						if let Err(err) = self.save("res/swears.txt") {
							println!("error saving swears: {}", err);
						}
						let _ = discord.send_message(message.channel_id, "Registered", "", false);
					},
					("!register", None) => {
						let _ = discord.send_message(message.channel_id, "Did you mean to actually give me a word?", "", false);
					},
					("!unregister", Some((_, word))) => {
						self.unregister(word.trim());
						if let Err(err) = self.save("res/swears.txt") {
							println!("error saving swears: {}", err);
						}
						let _ = discord.send_message(message.channel_id, "Unregistered", "", false);
					},
					("!unregister", None) => {
						let _ = discord.send_message(message.channel_id, "Did you mean to actually give me a word?", "", false);
					},
					//check for swearing
					_ => for &(_, ref swear) in &self.swears {
						if swear.is_match(&lower[..]) {
							let _ = discord.delete_message(message.channel_id, message.id);
							let _ = discord.send_message(message.channel_id,
														 &*format!("{} Hey! Expand your vocabulary!", message.author.mention()), "", false);
							break;
						}
					}
				}
			}
			_ => {}
		}
	}
}
