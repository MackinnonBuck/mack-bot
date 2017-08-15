extern crate discord;

use bot_module::BotModule;

use discord::Discord;
use discord::Connection;
use discord::model::{Event, Game, Message};

const WIDTH: usize = 7;
const HEIGHT: usize = 6;
const SEARCH_DEPTH: i32 = 4;

static NONE: &'static str = "None";

pub struct Connect4 {
	enemy: Option<String>,
	msg: Option<Message>,
	grid: [[char; HEIGHT]; WIDTH]
}

impl Connect4 {
	pub fn new() -> Connect4 {
		Connect4 {
			enemy: Option::None,
			msg: Option::None,
			grid: [[' '; HEIGHT]; WIDTH]
		}
	}
	
	fn place(grid: &mut [[char; HEIGHT]; WIDTH], player: char, position: usize) -> bool {
		if grid[position][0] != ' ' {
			return false
		} else if grid[position][HEIGHT - 1] == ' ' {
			grid[position][HEIGHT - 1] = player;
		} else {
			for i in 1..HEIGHT {
				if grid[position][i] != ' ' {
					grid[position][i - 1] = player;
					break;
				}
			}
		}
		true
	}
	
	fn check_winner(grid: [[char; HEIGHT]; WIDTH]) -> Option<char> {
		for i in 0..WIDTH {
			for j in 0..HEIGHT {
				let c = grid[i][j];
				
				if c == ' ' {
					continue
				}
				
				let mut l = 1;
				
				if i < WIDTH - 1 && j > 0 {
					let (mut x, mut y) = (i as i8 + 1, j as i8 - 1);
					
					while x < WIDTH as i8 && y as i8 >= 0 && grid[x as usize][y as usize] == c {
						l += 1;
						x += 1;
						y -= 1;
					}
					
					if l >= 4 {
						return Option::Some(c);
					}
				}
				
				if i < WIDTH - 1 {
					l = 1;
					let mut x = i + 1;
					
					while x < WIDTH && grid[x][j] == c {
						l += 1;
						x += 1;
					}
					
					if l >= 4 {
						return Option::Some(c);
					}
				}
				
				if i < WIDTH - 1 && j < HEIGHT - 1 {
					l = 1;
					let (mut x, mut y) = (i + 1, j + 1);
					
					while x < WIDTH && y < HEIGHT && grid[x][y] == c {
						l += 1;
						x += 1;
						y += 1;
					}
					
					if l >= 4 {
						return Option::Some(c);
					}
				}
				
				if j < HEIGHT - 1 {
					l = 1;
					let mut y = j + 1;
					
					while y < HEIGHT && grid[i][y] == c {
						l += 1;
						y += 1;
					}
					
					if l >= 4 {
						return Option::Some(c);
					}
				}
			}
		}
		Option::None
	}
	
	fn handle_message(&mut self, discord: &Discord, connection: &Connection, message: &Message) {
		let lower = message.content.to_lowercase();
		let mut splitter = lower.split_whitespace();
		
		if let Some(command) = splitter.next() {
			let enemy = match self.enemy {
				Option::Some(ref x) => x.clone(),
				Option::None => NONE.to_string()
			};
			
			let check_enemy_valid = || {
				if enemy == NONE {
					let _ = discord.send_message(message.channel_id, "Bro, who said we were playing a game?", "", false);
					false
				} else if *enemy == *message.author.name {
					true
				} else {
					let _ = discord.send_message(message.channel_id, &*format!("Hey, it's {} who's playing, not you.", enemy), "", false);
					false
				}
			};
			
			match command {
				"!play" => {
					if enemy == NONE {
						self.start_game(discord, connection, message);
					} else {
						let _ = discord.send_message(message.channel_id,
							&*format!("Shut up, I'm already playing with {}!", enemy), "", false);
					}
				}
				"!quit" => {
					if check_enemy_valid() {
						let _ = discord.send_message(message.channel_id, "Wow. Sore loser.", "", false);
						self.quit_game(connection);
					}
				}
				"!ragequit" => {
					if check_enemy_valid() {
						let _ = discord.send_message(message.channel_id, "(\u{256F}\u{00B0}\u{25A1}\u{00B0}\u{FF09}\u{256F}\u{FE35} \u{253B}\u{2501}\u{253B}", "", false);

						self.quit_game(connection);
					}
				}
				"!place" => {
					if check_enemy_valid() {
						if let Some(index) = splitter.next() {
							self.take_turn(discord, connection, message, index);
						} else {
							let _ = discord.send_message(message.channel_id, "Invalid command, index not specified.", "", false);
						}
					}
				}
				_ => {}
			}
		}
	}
	
	fn start_game(&mut self, discord: &Discord, connection: &Connection, message: &Message) {
		connection.set_game(Option::Some(Game::playing("Connect4".to_string())));
		
		self.enemy = Option::Some(message.author.name.clone());
		let _ = discord.send_message(message.channel_id,
			&*format!("Starting Connect4 with {} (I'm '@' and you're '#').", message.author.name),
			"", false);
		
		self.place_computer();
		self.update_grid(discord, message);
	}
	
	fn quit_game(&mut self, connection: &Connection) {
		connection.set_game(Option::None);
		*self = Connect4::new();
	}
	
	fn take_turn(&mut self, discord: &Discord, connection: &Connection, message: &Message, index: &str) {
		match index.parse::<usize>() {
			Ok(val) => {
				if val < WIDTH {
					if self.place_enemy(discord, message, val) {
						let _ = discord.delete_message(message.channel_id, message.id);
						
						if Connect4::check_winner(self.grid) == Option::Some('#') {
							self.update_grid(discord, message);
							let _ = discord.send_message(message.channel_id,
								"Well, you won. But that's only because you cheated.", "", false);
							self.quit_game(connection);
						} else {
							self.place_computer();
							self.update_grid(discord, message);
							if Connect4::check_winner(self.grid) == Option::Some('@') {
								let _ = discord.send_message(message.channel_id,
								"Ha! Loser. You really thought you could beat me?", "", false);
								self.quit_game(connection);
							}
						}
					}
				} else {
					let _ = discord.send_message(message.channel_id, "Index is out of range.", "", false);
				}
			}
			Err(_) => {
				let _ = discord.send_message(message.channel_id, "Index is not a valid number.", "", false);
			}
		}
	}
	
	fn update_grid(&mut self, discord: &Discord, message: &Message) {
		let mut sgrid = "```\n".to_string();
		
		for i in 0..WIDTH  {
			sgrid.push_str(&*format!(" {}", i));
		}
		
		sgrid.push('\n');
		
		for i in 0..HEIGHT {
			for j in 0..WIDTH {
				sgrid.push_str(&*format!("|{}", self.grid[j][i]));
			}
			
			sgrid.push_str("|\n");
		}
		
		for _ in 0..WIDTH * 2 + 1 {
			sgrid.push('-');
		}
		
		sgrid.push_str("\n```");
		
		if let Some(ref msg) = self.msg {
			let _ = discord.delete_message(msg.channel_id, msg.id);
		}
		
		if let Ok(msg) = discord.send_message(message.channel_id, &sgrid, "", false) {
			self.msg = Option::Some(msg);
		}
	}
	
	fn place_enemy(&mut self, discord: &Discord, message: &Message, position: usize) -> bool {
		if Connect4::place(&mut self.grid, '#', position) {
			true
		} else {
			let _ = discord.send_message(message.channel_id, "Invalid position ya dweeb. The column is full.", "", false);
			false
		}
	}

	fn find_best_placement(&mut self, grid: [[char; HEIGHT]; WIDTH], scores: &mut [(i32, i32); WIDTH], depth: i32, mut column: i8) {
		if depth == 0 {
			return
		}
		
		let original = column;
		
		for i in 0..WIDTH {
			if original == -1 {
				column = i as i8;
			}
			
			let mut copy = grid;
			if !Connect4::place(&mut copy, '@', i) {
				scores[column as usize] = (depth + 1, 0);
				continue
			}
			
			if let None = Connect4::check_winner(copy) {
				let mut copy2 = copy;
				for j in 0..WIDTH {				
					Connect4::place(&mut copy2, '#', j);
					
					if let Some(_) = Connect4::check_winner(copy2) {
						scores[column as usize] = (depth, scores[column as usize].1 + 1);
					}
				}
				
				if scores[column as usize].0 == -1 {
					self.find_best_placement(copy2, scores, depth - 1, column);
				}
			}
		}
	}
	
	fn place_computer(&mut self) {
		let mut scores = [(-1, 0); WIDTH];
		let grid = self.grid;
		
		self.find_best_placement(grid, &mut scores, SEARCH_DEPTH, -1);
		
		let (mut val, mut index) = (scores[0], 0);
		
		for i in 0..WIDTH {
			println!("{}: {}, {}", i, scores[i].0, scores[i].1);
			if scores[i] < val {
				val = scores[i];
				index = i;
			}
		}
		
		println!("{} is the best choice with a depth rating of {} and a possibility count of {}.", index, val.0, val.1);
		
		Connect4::place(&mut self.grid, '@', index);
	}
}

impl BotModule for Connect4 {
	fn handle(&mut self, discord: &Discord, connection: &Connection, event: &Event) {
		match *event {
			Event::MessageCreate(ref message) => {
				self.handle_message(discord, connection, message);
			}
			_ => {}
		}
	}
}
