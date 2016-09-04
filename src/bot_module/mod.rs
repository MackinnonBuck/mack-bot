extern crate discord;

use discord::Discord;
use discord::Connection;
use discord::model::Event;

pub mod connect4;
pub mod everett;

pub trait BotModule {
	fn handle(&mut self, discord: &Discord, connection: &Connection, event: &Event);
}