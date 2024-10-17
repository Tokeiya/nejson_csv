#![feature(format_args_nl)]
#![allow(dead_code)]
mod gen_sample;

use chrono::prelude::*;
use chrono::Local;
use chrono::TimeZone;

fn main() {
	let time = Utc::now();
	let local = time.with_timezone(&Local);
}
