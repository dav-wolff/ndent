#![deny(unsafe_code)]

use std::io::stdin;

const INDENT_SIZE: u32 = 4;

fn main() {
	stdin().lines()
		.map(|line| line.unwrap())
		.map(|line| extract_indentation(&line))
		.scan(0, |previous_spaces, (spaces, line)|
			if line.is_empty() && spaces == 0 {
				Some((*previous_spaces, line))
			} else {
				*previous_spaces = spaces;
				Some((spaces, line))
			}
		)
		.map(|(spaces, line)| indent_line(spaces, &line))
		.for_each(|line| println!("{line}"));
}

fn extract_indentation(line: &str) -> (u32, String) {
	let (characters, spaces) = line.chars()
		.map_while(|char| match char {
			' ' => Some(1),
			
			
			'\t' => Some(INDENT_SIZE),
			_ => None,
		})
		.fold((0, 0), |(count, sum), spaces|
			(count + 1, sum + spaces)
		);
	
	(spaces, line[characters..].to_owned())
}

fn indent_line(spaces: u32, line: &str) -> String {
	let indents = spaces / INDENT_SIZE;
	let extra_spaces = spaces % INDENT_SIZE;
	
	let mut result = String::with_capacity(indents as usize + extra_spaces as usize + line.len());
	result.extend((0..indents).map(|_| '\t'));
	result.extend((0..extra_spaces).map(|_| ' '));
	result.push_str(line);
	result
}