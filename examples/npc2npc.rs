use std::{
//	io::stdin,
	thread::sleep,			// Use "thread" and "Duration" for NPCs talking to each other. 
	time::Duration,
};


use ears::{
	AudioController,
//	Music,
	Sound,
};


//use inline_colorization::*; // I originally modified this crate for my own convenience. There's nothing too signifant. 
use rand::Rng;

fn main() {
	enemy();
//	enemy_advance();
}

fn enemy() {
	let druggie = (
		rand::thread_rng().gen_range(1..=2), // druggie.0
	);

	let speak = (
		rand::thread_rng().gen_range(1..=4), // voice.0
		rand::thread_rng().gen_range(1..=4), // voice.1
	);

	let diachoice = (
		rand::thread_rng().gen_range(1..=2), // diachoice.0
	);

	match druggie.0 {
		1 => {
			println!("Druggie: What the fuck are you looking at?"); // When we're working with the first interaction, "println!();" comes before "match"
			match speak.0 {
				1 => { 
					let mut voice_1p1 = Sound::new("examples/npc2npc_data/voice_1.1.flac").unwrap(); // voice 1.1
					voice_1p1.play();
					//sleep(Duration::from_secs(3));
					delay();
				}, 
				2 => { println!("voice two"); }, // voice 2.1
				3 => { println!("voice three"); }, // voice 3.1
				4 => { println!("voice four"); }, // voice 4.1

				_ => {},
			}

			println!("Druggie: I am looking. AT YOU!");
			match speak.1 {
				1 => { 
					let mut voice_1p2 = Sound::new("examples/npc2npc_data/voice_1.2.flac").unwrap(); // voice 1.2
					voice_1p2.play();
					delay();
				}, 
				2 => { println!("voice two"); }, // voice 2.2
				3 => { println!("voice three"); }, // voice 3.2
				4 => { println!("voice four"); }, // voice 4.2

				_ => {},
			}

			println!();
			match speak.0 {
				1 => {

					match diachoice.0 {
						1 => {
							println!("Well, fuck you then. "); // Now that we are giving NPC's a dialogue choice, "println!(); goes inside a "match" value. 
							let mut voice_1p3p1 = Sound::new("examples/npc2npc_data/voice_1.3.1.flac").unwrap(); // voice 1.3.1
							voice_1p3p1.play();
							delay();
						}
						2 => {
							println!("Whatever. ");
							let mut voice_1p3p2 = Sound::new("examples/npc2npc_data/voice_1.3.2.flac").unwrap(); // voice 1.3.2
							voice_1p3p2.play();
							delay();
						}

						_ => {},
					}
				},
				2 => {

					match diachoice.0 {
						1 => {
							println!("Well, fuck you then. "); // Now that we are giving NPC's a dialogue choice, "println!(); goes inside a "match" value. 
						//	let mut voice_2p3p1 = Sound::new("examples/npc2npc_data/voice_2.3.1.flac").unwrap(); // voice 2.3.1
						//	voice_2p3p1.play();
							delay();
						}
						2 => {
							println!("Whatever. ");
						//	let mut voice_2p3p2 = Sound::new("examples/npc2npc_data/voice_2.3.2.flac").unwrap(); // voice 2.3.2
						//	voice_2p3p2.play();
							delay();
						}

						_ => {},
					}
				}, 
				3 => {

					match diachoice.0 {
						1 => {
							println!("Well, fuck you then. "); // Now that we are giving NPC's a dialogue choice, "println!(); goes inside a "match" value. 
						//	let mut voice_3p3p1 = Sound::new("examples/npc2npc_data/voice_3.3.1.flac").unwrap(); // voice 3.3.1
						//	voice_3p3p1.play();
							delay();
						}
						2 => {
							println!("Whatever. ");
						//	let mut voice_3p3p2 = Sound::new("examples/npc2npc_data/voice_3.3.2.flac").unwrap(); // voice 3.3.2
						//	voice_3p3p2.play();
							delay();
						}

						_ => {},
					}
				}, 
				4 => {

					match diachoice.0 {
						1 => {
							println!("Well, fuck you then. "); // Now that we are giving NPC's a dialogue choice, "println!(); goes inside a "match" value. 
						//	let mut voice_4p3p1 = Sound::new("examples/npc2npc_data/voice_4.3.1.flac").unwrap(); // voice 4.3.1
						//	voice_4p3p1.play();
							delay();
						}
						2 => {
							println!("Whatever. ");
						//	let mut voice_4p3p2 = Sound::new("examples/npc2npc_data/voice_4.3.2.flac").unwrap(); // voice 4.3.2
						//	voice_4p3p2.play();
							delay();
						}

						_ => {},
					}
				}, 

				_ => {},
			}
		}

		2 => {
			println!("Druggie: Get the fuck out of my face. ");
			match speak.0 {
				1 => { 
					let mut voice_1p3 = Sound::new("examples/npc2npc_data/voice_1.3.flac").unwrap(); // voice 1.3
					voice_1p3.play();
					//sleep(Duration::from_secs(3));
					delay();
				},
				2 => { println!("voice two"); }, // voice 2.3
				3 => { println!("voice three"); }, // voice 3.3
				4 => { println!("voice four"); }, // voice 4.3

				_ => {},
			}

			println!("Druggie: ...");
			match speak.1 {
				1..3 => println!("no voice"),

				_ => {},
			}
		}
		_ => println!(),
	}
}

fn delay() {
	sleep(Duration::from_secs(2));
}
