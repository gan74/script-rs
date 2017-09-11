use position::{Position};

use std::process::*;

pub fn fatal(msg: &str, pos: &Position) -> !{
	println!("at {}: {}", pos, msg);
	exit(1)
}