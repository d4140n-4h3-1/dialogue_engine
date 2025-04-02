use std::{
	io::stdin,
	thread::sleep,			// Use "thread" and "Duration" for NPCs talking to each other. 
	time::Duration,
};

use ears::{
	AudioController,
//	Music,
	Sound,
};

use inline_colorization::*; // I originally modified this crate for my own convenience. There's nothing too signifant. 
use rand::Rng;

fn main() {
	troll();	
}

// Sound effects that may be reused. Used as separate functions for now. 

fn neckpop() {
	let mut neckpop = Sound::new("examples/troll_data/neck_crack.mp3").unwrap();

	let mut repeat = 0; // Start from 0

	while repeat < 2 { // Number of times you want to loop
		neckpop.play();
		sleep(Duration::from_secs(1));
		repeat += 1; // Add 1
	}
	
}

fn punch() {
	let mut punch = Sound::new("examples/troll_data/face-punch.mp3").unwrap();

	punch.play();
	sleep(Duration::from_secs(1));
}

// I am working on persuasion in "npc_template". It's labeled as a coin flip, and for now I am using the "rand" crate. 

fn begin_combat() {
	println!("Troll: Alright. ");
		let mut troll_line_alright = Sound::new("examples/troll_data/troll-line_alright.flac").unwrap();
		troll_line_alright.play();
		sleep(Duration::from_secs(1));

	println!("       *{style_italics}Cracks neck{style_reset}*");
		let mut neckpop = Sound::new("examples/troll_data/neck_crack.mp3").unwrap();
		let mut repeat = 0; // Start from 0
		while repeat < 2 { // Number of times you want to loop
			neckpop.play();
			sleep(Duration::from_secs(1));
			repeat += 1; // Add 1
		}

	println!("       Let's dance. ");
		let mut troll_line_lets_dance = Sound::new("examples/troll_data/troll-line_lets_dance.flac").unwrap();
		troll_line_lets_dance.play();
		sleep(Duration::from_secs(2));
}

fn troll() {
	let mut player_answer = (
		String::new(), // player_answer.0
		String::new(), // player_answer.1
		String::new(), // player_answer.2
	);

	let persuasion = rand::thread_rng().gen_range(1..=2);

	println!("{style_bold}Troll:{style_reset} Hey! Who the hell are you? ");
		let mut troll_line_one = Sound::new("examples/troll_data/troll-line_1.flac").unwrap();
		troll_line_one.play();
		sleep(Duration::from_secs(1));
	println!();

	println!("A) IDENTIFY AND EXPLAIN ");
	println!("B) SCOLD CHOICE LANGUAGE ");
	println!("C) REMAIN SILENT ");
	println!();

	stdin().read_line(&mut player_answer.0).ok();
	match &player_answer.0[..1] {

		// Line 2.1
		"a" | "A" => {
			println!("{style_bold}Player:{style_reset} I am just a traveler. I could use some directions. ");
			println!("{style_bold}Troll:{style_reset} Sure, thing. It's back the way you came. ");
				let mut troll_line_2p1 = Sound::new("examples/troll_data/troll-line_2.1.flac").unwrap();
				troll_line_2p1.play();
				sleep(Duration::from_secs(3));
			println!();

			println!("A) EXPLAIN FURTHER ");
			println!("B) QUESTION DEMEANOR ");
			println!("C) LEAVE");
			println!();

			stdin().read_line(&mut player_answer.1).ok();
			match &player_answer.1[..1] {
				// Line 3.1.1
				"a" | "A" => {
					println!("{style_bold}Player:{style_reset} I am on a quest. I'll be out of your hair if you let me pass. ");
					println!("{style_bold}Troll:{style_reset} Hmm. *{style_italics}purses lips{style_reset}*"); 
					println!("       I could let you through, if you have the coin. ");
						let mut troll_line_3p1p1 = Sound::new("examples/troll_data/troll-line_3.1.1.flac").unwrap();
						troll_line_3p1p1.play();
						sleep(Duration::from_secs(5));
					println!();

					println!("A) PAY X_AMOUNT COINS "); 	// write a money counter first
					println!("B) BARTER ");			// write an inventory first
					println!("C) NEGOTIATE "); 		// using rand crate first
					println!("D) ASK FOR WORK");		// write a side quest first
					println!("E) LEAVE");
					println!();

					stdin().read_line(&mut player_answer.2).ok();
					match &player_answer.2[..1] {
						// Line 4.1 
						"a" | "A" => {
							println!("{style_bold}Player:{style_reset} Sure, here you go. ");
							println!("{style_bold}Troll:{style_reset} Get on through before I change my mind. ");
								let mut troll_line_4p1 = Sound::new("examples/troll_data/troll-line_4.1.flac").unwrap();
								troll_line_4p1.play();
								sleep(Duration::from_secs(3));
							println!();
						}
						// Line 4.2
						"b" | "B" => {
							println!("{style_bold}Player:{style_reset} How about a trade? ");
							println!("{style_bold}Troll:{style_reset} I guess it's better than nothing. ");
								let mut troll_line_4p2 = Sound::new("examples/troll_data/troll-line_4.2.flac").unwrap();
								troll_line_4p2.play();
								sleep(Duration::from_secs(5));
							println!();
						//	troll_barter();
						}
						// Line 4.3
						"c" | "C" => {
							println!("{style_bold}Player:{style_reset} Is this amount okay? ");
							match persuasion { // This block may evolve into an "if" statement when further enhanced. 
									   // We'll need to input an x amount of coins provided the player has them 
									   // within range.  
								1 => {
									println!("{style_bold}Troll:[FAILED]{style_reset} What do you take me for? A beggar? ");
										let mut troll_line_4p3a = Sound::new("examples/troll_data/troll-line_4.3a.flac").unwrap();
										troll_line_4p3a.play();
										sleep(Duration::from_secs(3));
								},

								2 => {
									println!("{style_bold}Troll:[SUCCESS]{style_reset} I guess it's better than nothing. ");
										let mut troll_line_4p3b = Sound::new("examples/troll_data/troll-line_4.2.flac").unwrap();
										troll_line_4p3b.play();
										sleep(Duration::from_secs(1));
								},

								3 => {
									println!("{style_bold}Troll:{style_reset} You ain't got that much. ");
								//	troll_line_4p3c.play(); // There is no sound yet
								//	sleep(Duration::from_secs(1));
								}
								_ => println!(),
							}
							println!();
						}
						// Line 4.4
						"d" | "D" => {
							println!("{style_bold}Player:{style_reset} Is there another option? ");
							println!("{style_bold}Troll:{style_reset} Hmm...");
							println!("       Tell you what. You go find these ingredients, and you can cross this bridge anytime you like. ");
								let mut troll_line_4p4 = Sound::new("examples/troll_data/troll-line_4.4.flac").unwrap();
								troll_line_4p4.play(); 
								sleep(Duration::from_secs(8));
							println!();
						//	troll_sidequest(); // activate sidequest here, echo notes in a text file
						}
						// Line 4.5
						"e" | "E" => {
							println!("{style_bold}Player:{style_reset} Hold on. I'll return. ");
							println!("{style_bold}Troll:{style_reset} Don't forget to bring payment next time. ");
								let mut troll_line_4p5 = Sound::new("examples/troll_data/troll-line_4.5.flac").unwrap();
								troll_line_4p5.play();
								sleep(Duration::from_secs(1));
							println!();
						}
						_ => println!(),
					}
				}
				// Line 3.1.2
				"b" | "B" => {
					println!("{style_bold}Player:{style_reset} Why are you such a jerk? ");
					println!("{style_bold}Troll:{style_reset} Probably because you are wasting my oxygen. ");
						let mut troll_line_3p1p2 = Sound::new("examples/troll_data/troll-line_3.1.2.flac").unwrap();
						troll_line_3p1p2.play();
						sleep(Duration::from_secs(3));
					println!();

				}
				// Line 3.1.3
				"c" | "C" => {
					println!("{style_bold}Player:{style_reset} I have to go now. ");
					println!("{style_bold}Troll:{style_reset} Yeah. Beat it. ");
						let mut troll_line_3p1p3 = Sound::new("examples/troll_data/troll-line_3.1.3.flac").unwrap();
						troll_line_3p1p3.play();
						sleep(Duration::from_secs(2));
					println!();
				}
				_ => println!(),
			}
		}
		// Line 2.2
		"b" | "B" => {
			println!("{style_bold}Player:{style_reset} Do you talk to your mother that mouth? ");
			println!("{style_bold}Troll:{style_reset} Are you looking for a fight? ");
				let mut troll_line_2p2 = Sound::new("examples/troll_data/troll-line_2.2.flac").unwrap();
				troll_line_2p2.play();
				sleep(Duration::from_secs(1));
			println!();

			println!("A) YES");
			println!("B) ASSAULT");
			println!("C) RETRACT");
			println!();

			stdin().read_line(&mut player_answer.1).ok();
			match &player_answer.1[..1] {
				"a" | "A" | "y" | "Y" => {
					begin_combat();
					println!();
				//	troll_combat();
				}
				"b" | "B" => {
					punch();
					begin_combat();
					println!();
				//	troll_combat();
				}
				"c" | "C" | "n" | "N" => {
					println!("{style_bold}Player:{style_reset} No. I sense I made a mistake. ");
					println!("{style_bold}Troll:{style_reset} Yeah, that's what I thought. ");
						// Sound here
					println!();
				}
				_ => println!(),
			}
		}
		// Line 2.3
		"c" | "C" => {
			println!("{style_bold}Player:{style_reset} *{style_italics}Remain silent{style_reset}*");
			println!("{style_bold}Troll:{style_reset} Well? You got anything to say?");
				let mut troll_line_2p3 = Sound::new("examples/troll_data/troll-line_2.3.flac").unwrap();
				troll_line_2p3.play();
				sleep(Duration::from_secs(1));
			println!();

			println!("A) STARE");
			println!("B) ASSAULT");
			println!("C) LEAVE");
			println!();

			stdin().read_line(&mut player_answer.1).ok();
			match &player_answer.1[..1] {
				"a" | "A" => {
					println!("{style_bold}Player:{style_reset} *{style_italics}stare{style_reset}*");
					println!("{style_bold}Troll:{style_reset} You are starting to make me mad...");
						// Sound here
					println!();
				}
				"b" | "B" => {
					println!("{style_bold}Player:{style_reset} *{style_italics}kicked the testicles{style_reset}*"); punch();
					println!("{style_bold}Troll:{style_reset} Alright. "); 
						let mut troll_line_alright_nutcracker = Sound::new("examples/troll_data/troll-line_alright_nutcracker.flac").unwrap();
						troll_line_alright_nutcracker.play();
						sleep(Duration::from_secs(1));
					println!("       *{style_italics}cracks neck{style_reset}*"); 		neckpop();
					println!("       Let's dance");
						let mut troll_line_lets_dance_nutcracker = Sound::new("examples/troll_data/troll-line_lets_dance_nutcracker.flac").unwrap();
						troll_line_lets_dance_nutcracker.play();
						sleep(Duration::from_secs(2));
					println!();
				//	troll_combat();
				}
				"c" | "C" => {
					println!("{style_bold}Player:{style_reset} *{style_italics}leave{style_reset}*");
					println!("{style_bold}Troll:{style_reset} Yeah. Beat it. ");
					println!();
				}
				_ => println!(),
			}
		}
		_ => println!(),
	}
}

/*
fn troll_barter() {}
*/

/*
fn troll_combat() {
	match {
		"a" => { println!("Battleaxe"); },
		"b" => { println!("Bow"); },
		"c" => { println!("Dagger"); },
		"d" => { println!("Fist"); }
	}
}
*/

/*
fn troll_sidequest() {}
*/
