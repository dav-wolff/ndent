#![deny(unsafe_code)]

use std::io::{stdin, stdout, Read, Write, self, BufReader, BufRead};

const DEFAULT_INDENT_SIZE: u32 = 4;

fn main() {
	let mut args = std::env::args();
	let _ = args.next(); // ndent
	let indent_size = args.next()
		.map(|str| str.parse::<u32>().ok())
		.flatten()
		.unwrap_or(DEFAULT_INDENT_SIZE);
	
	indent(indent_size, stdin().lock(), stdout().lock())
		.unwrap();
}

fn indent(indent_size: u32, read: impl Read, mut write: impl Write) -> Result<(), io::Error> {
	let mut read = BufReader::new(read);
	
	let mut line = String::new();
	let mut previous_spaces = 0u32;
	
	while let Ok(length) = read.read_line(&mut line) {
		// EOF
		if length == 0 {
			break;
		}
		
		let (mut spaces, stripped_line) = extract_indentation(indent_size, &line);
		
		if line.is_empty() && spaces == 0 {
			spaces = previous_spaces;
		} else {
			previous_spaces = spaces;
		}
		
		let indented_line = indent_line(indent_size, spaces, &stripped_line);
		
		write!(write, "{indented_line}")?;
		
		line.clear();
	}
	
	Ok(())
}

fn extract_indentation(indent_size: u32, line: &str) -> (u32, String) {
	let (characters, spaces) = line.chars()
		.map_while(|char| match char {
			' ' => Some(1),
			
			
			'\t' => Some(indent_size),
			_ => None,
		})
		.fold((0, 0), |(count, sum), spaces|
			(count + 1, sum + spaces)
		);
	
	(spaces, line[characters..].to_owned())
}

fn indent_line(indent_size: u32, spaces: u32, line: &str) -> String {
	let indents = spaces / indent_size;
	let extra_spaces = spaces % indent_size;
	
	let mut result = String::with_capacity(indents as usize + extra_spaces as usize + line.len());
	result.extend((0..indents).map(|_| '\t'));
	result.extend((0..extra_spaces).map(|_| ' '));
	result.push_str(line);
	result
}