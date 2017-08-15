use std::str::SplitWhitespace;

pub trait Command {
	fn command<'a>(&'a self) -> (&'a str, Option<(SplitWhitespace<'a>, &'a str)>);
}

impl Command for String {
	fn command<'a>(&'a self) -> (&'a str, Option<(SplitWhitespace<'a>, &'a str)>) {
		let index = self.chars().enumerate()
			.find(|&(_, ch)| ch.is_whitespace())
			.map(|(idx, _)| idx).unwrap_or(self.len());

		let command = &self[0..index];

		let args = if index < self.len() {
			let mut splitter = self.split_whitespace();
			splitter.next();

			Some((splitter, &self[index..self.len()]))
		}
		else {
			None
		};

		(command, args)
	}
}
