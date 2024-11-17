use std::{
	io::stdin,
	thread::sleep,			// Use "thread" and "Duration" for NPCs talking to each other. 
	time::Duration,
};

use ears::{ 			// If you wish to utilize voices or sound effects, disable "thread" and "Duration" via "//". 
	AudioController,	// They will not be necessary as Ears will automatically set a countdown based on an audio file's length. 
//	Music,
	Sound,
};

// use inline_colorization::*; 	// I originally modified this crate for my own convenience. There's nothing too signifant. 
				// Although, I probably should rename this crate while giving the original author credit. 
use rand::Rng;

fn main() {
	npc_template();
//	shimmer();
}

// I am working on persuasion in "npc_template". It's labeled as a coin_flip, and for now I am using the "rand" crate. 
fn npc_template() {
	// Could have used "if" statements, but "match" expressions take up a little less code. 
	// It's pretty much the same effect until you get more advanced. 

	// This is a tuple. 
	let mut choice = (
		String::new(), // choice.0, 0 is equal to 1 in this context. 
		String::new(), // choice.1
		String::new(), // choice.2
		// Repeat "String::new()," as many times as needed. 
	);

	println!("A) Choice 1");
	println!("B) Choice 2");
	println!("C) Choice 3");
	println!(); // It's wise to create an empty line between blocks. Better readability in the command line. 
	stdin().read_line(&mut choice.0).ok();
	match &choice.0[..1] { 	// It seems character count is static rather than dynamic by default. 
				// If I tried [..2] rather than [..1], and then type "a", the compiler panics.
				// Maybe there is a workaround somewhere... 
		"a" | "A" => {
			println!("You chose A. Pick another? ");
			println!();

			println!("A) Choice 4");
			println!("B) Choice 5");
			println!("C) Choice 6");
			println!("D) Choice 7");
			println!();

			stdin().read_line(&mut choice.1).ok(); 
			match &choice.1[..1] {
				"a" | "A" => println!("Alpha"), // Replace ";" with "," as single lines do not need to be placed in additional brackets. 
				"b" | "B" => println!("Bravo"), // Should you require multiple lines, surround them with "{}" after "=>", then use ";" again. 
				"c" | "C" => println!("Charlie"),
				"d" | "D" => println!("Delta"),

				_ => { 
					println!("error");
					// find a way to remain in "choice.1" if incorrect, 
					// then still move on after inputing a valid character.  
				}
			} 
		}
		"b" | "B" => {
			println!("You chose B. Pick another? ");
			println!();

			println!("E) Choice 4");
			println!("F) Choice 5");
			println!("G) Choice 6");
			println!("H) Choice 7");
			println!();

			stdin().read_line(&mut choice.1).ok();
			match &choice.1[..1] {
				"e" | "E" => { print!("Echo: ");    coin_flip();  },
				"f" | "F" => { print!("Foxtrot: "); dice();       },
				"g" | "G" => { print!("Gulf: ");    eight_ball(); },
				"h" | "H" => { print!("Hotel: ");   counter();    },

				_ =>         { println!("error"); }
			}
		}
		"c" | "C" => { // Same as B, but with a voice. 
			println!("You chose C. Pick another? ");
			println!();

			println!("I) Choice 4");
			println!("J) Choice 5");
			println!("K) Choice 6");
			println!("L) Choice 7"); // Coin flip referred here
			println!();

			stdin().read_line(&mut choice.1).ok();
			match &choice.1[..1] { 
				"i" | "I" => { print!("India: ");  coin_flip_via_ears();  },
				"j" | "J" => { print!("Juliet: "); dice_via_ears();       },
				"k" | "K" => { print!("Kilo: ");   eight_ball_via_ears(); },
				"l" | "L" => { print!("Lima: ");   counter_via_ears();    },

				_ =>         { println!("error"); }
			}
		}
		_ => println!("error"),
	}
}

fn coin_flip() {
	println!("COIN FLIP!");

	let toss = match rand::thread_rng().gen_range(1..=2) {
		1 => "Heads",
		2 => "Tails",
		_ => "Error",
	};
	println!("YOU HAVE: {toss}");
}

fn coin_flip_via_ears() { // called by "India"
	println!("COIN FLIP, W/SOUND!");

	let delay = Duration::from_secs(1);
	
	let mut head = Sound::new("data/main_audio/heads-shimmer.mp3").unwrap();
	let mut tail = Sound::new("data/main_audio/tails-shimmer.mp3").unwrap();

	match rand::thread_rng().gen_range(1..=2) {
		1 => { println!("Heads"); head.play(); sleep(delay); }
		2 => { println!("Tails"); tail.play(); sleep(delay); }

 		_ => println!("Error"),
	};
}

fn dice() {
	println!("DICE!"); // where 'print!("Juliet: ");' begins, this line finishes as 'Juliet: DICE!'. 
	let roll = (
		rand::thread_rng().gen_range(1..=6), // roll.0
		rand::thread_rng().gen_range(1..=6), // roll.1
	);

	let die = (
		match roll.0 {
			01 => "One",  02 => "Two",  03 => "Three",
			04 => "Four", 05 => "Five", 06 => "Six",

			_ => "Error"
		},
		 match roll.1 {
			01 => "One",  02 => "Two",  03 => "Three", 
			04 => "Four", 05 => "Five", 06 => "Six",

			_ => "Error"
		},
	);

	println!("FIRST DIE:  {}", die.0);
	println!("SECOND DIE: {}", die.1);

	let total = match roll.0 + roll.1 {
		// '1' does not exist here. While this code will work 
		// with it just fine, it will never be printed even 
		// if written as real traditional dice do not contain '0'.  
		02 => "Two", 03 => "Three",  04 => "Four",  05 => "Five", 
		06 => "Six", 07 => "Seven",  08 => "Eight", 09 => "Nine", 
		10 => "Ten", 11 => "Eleven", 12 => "Twelve",

		_  => "Error"
	};
	println!("TOTAL:      {total}");
}

fn dice_via_ears() { // called by Juliet

	println!("DICE W/SOUND!");

	// Shimmer's Voice
	let mut one    = Sound::new("data/main_audio/integers/01-shimmer.mp3").unwrap();
	let mut two    = Sound::new("data/main_audio/integers/02-shimmer.mp3").unwrap();
	let mut three  = Sound::new("data/main_audio/integers/03-shimmer.mp3").unwrap();
	let mut four   = Sound::new("data/main_audio/integers/04-shimmer.mp3").unwrap();
	let mut five   = Sound::new("data/main_audio/integers/05-shimmer.mp3").unwrap();
	let mut six    = Sound::new("data/main_audio/integers/06-shimmer.mp3").unwrap();
	let mut seven  = Sound::new("data/main_audio/integers/07-shimmer.mp3").unwrap();
	let mut eight  = Sound::new("data/main_audio/integers/08-shimmer.mp3").unwrap();
	let mut nine   = Sound::new("data/main_audio/integers/09-shimmer.mp3").unwrap();
	let mut ten    = Sound::new("data/main_audio/integers/10-shimmer.mp3").unwrap();
	let mut eleven = Sound::new("data/main_audio/integers/11-shimmer.mp3").unwrap();
	let mut twelve = Sound::new("data/main_audio/integers/12-shimmer.mp3").unwrap();

	let mut first  = Sound::new("data/main_audio/integers/01st-shimmer.mp3").unwrap();
	let mut second = Sound::new("data/main_audio/integers/02nd-shimmer.mp3").unwrap();

	let mut die    = Sound::new("data/main_audio/die-shimmer.mp3").unwrap();
	let mut error  = Sound::new("data/main_audio/error-shimmer.mp3").unwrap();
//	let mut point  = Sound::new("data/main_audio/point-shimmer.mp3").unwrap();
//	let mut points = Sound::new("data/main_audio/points-shimmer.mp3").unwrap(); 
	let mut total  = Sound::new("data/main_audio/total-shimmer.mp3").unwrap();

	let delay = Duration::from_secs(1);

	let roll = (
		rand::thread_rng().gen_range(1..=6),
		rand::thread_rng().gen_range(1..=6),
	);

	let fd = "FIRST DIE:  ";
	match roll.0 {
		01 => { 
			println!("{fd} One");
			first.play();  sleep(delay); 
			die.play();    sleep(delay); 
			one.play();    sleep(delay); 
		}, 
		02 => { 
			println!("{fd} Two");
			first.play();  sleep(delay); 
			die.play();    sleep(delay); 
			two.play();    sleep(delay); 
		}, 
		03 => { 
			println!("{fd} Three"); 
			first.play();  sleep(delay);
			die.play();    sleep(delay);
			three.play();  sleep(delay); 
		},
		04 => { 
			println!("{fd} Four");
			first.play();  sleep(delay);
			die.play();    sleep(delay);
			four.play();   sleep(delay); 
		}, 
		05 => { 
			println!("{fd} Five");
			first.play();  sleep(delay);
			die.play();    sleep(delay);
			five.play();   sleep(delay); 
		}, 
		06 => { 
			println!("{fd} Six");
			first.play();  sleep(delay);
			die.play();    sleep(delay);
			six.play();    sleep(delay); 
		},

		_  => { 
			println!("Error");
			error.play();  sleep(delay); 
		},
	};

	let sd = "SECOND DIE: ";
	match roll.1 {
		01 => { 
			println!("{sd} One"); 
			second.play(); sleep(delay); 
			die.play();    sleep(delay); 
			one.play();    sleep(delay); 
		},
		02 => { 
			println!("{sd} Two"); 
			second.play(); sleep(delay); 
			die.play();    sleep(delay); 
			two.play();    sleep(delay); 
		}, 
		03 => { 
			println!("{sd} Three"); 
			second.play(); sleep(delay); 
			die.play();    sleep(delay); 
			three.play();  sleep(delay); 
		},
		04 => { 
			println!("{sd} Four"); 
			second.play(); sleep(delay); 
			die.play();    sleep(delay); 
			four.play();   sleep(delay); 
		}, 
		05 => { 
			println!("{sd} Five"); 
			second.play(); sleep(delay); 
			die.play();    sleep(delay); 
			five.play();   sleep(delay); 
		}, 
		06 => { 
			println!("{sd} Six"); 
			second.play(); sleep(delay); 
			die.play();    sleep(delay); 
			six.play();    sleep(delay); 
		},

		_  => { 
			println!("Error"); 
			error.play();  sleep(delay); 
		},
	};

	let t = "TOTAL:      ";
	match roll.0 + roll.1 {
	// The first block will work if uncommented, but will never be printed 
	// as the lowest possible value here is two. 
	/*	01 => { 
			println!("{t} One"); 
			one.play();    sleep(delay); 
		}, 
	*/
		02 => { 
			println!("{t} Two"); 
			total.play();  sleep(delay); 
			two.play();    sleep(delay); 
		}, 
		03 => { 
			println!("{t} Three"); 
			total.play();  sleep(delay); 
			three.play();  sleep(delay); 
		}, 
		04 => { 
			println!("{t} Four"); 
			total.play();  sleep(delay); 
			four.play();   sleep(delay); 
		}, 
		05 => { 
			println!("{t} Five"); 
			total.play();  sleep(delay); 
			five.play();   sleep(delay); 
		},
		06 => { 
			println!("{t} Six"); 
			total.play();  sleep(delay); 
			six.play();    sleep(delay); 
		}, 
		07 => { 
			println!("{t} Seven"); 
			total.play();  sleep(delay); 
			seven.play();  sleep(delay); 
		}, 
		08 => { 
			println!("{t} Eight"); 
			total.play();  sleep(delay); 
			eight.play();  sleep(delay); 
		}, 
		09 => { 
			println!("{t} Nine"); 
			total.play();  sleep(delay); 
			nine.play();   sleep(delay); 
		},
		10 => { 
			println!("{t} Ten"); 
			total.play();  sleep(delay); 
			ten.play();    sleep(delay); 
		}, 
		11 => { 
			println!("{t} Eleven"); 
			total.play();  sleep(delay); 
			eleven.play(); sleep(delay); 
		}, 
		12 => { 
			println!("{t} Twelve"); 
			total.play();  sleep(delay); 
			twelve.play(); sleep(delay); 
		},

		_  => { 
			println!("Error");      
			error.play();  sleep(delay); 
		},
	};
}

fn eight_ball() {
	println!("EIGHT BALL!");

	let message = (
		rand::thread_rng().gen_range(1..=3), // message.0 
		rand::thread_rng().gen_range(1..=3), // message.1
	);

	let ball = match message.0 {
		1 => match message.1 {
			1 => "Yes", 
			2 => "No", 
			3 => "Maybe",

			_ => "Error from 'message.1'",
		}

		2 => match message.1 {
			1 => "Certainly", 
			2 => "Certainly not", 
			3 => "Uncertain", 

			_ => "Error from 'message.1'",
		}

		3 => match message.1 {
			1 => "Absolutely",
			2 => "Absolutely not",
			3 => "Don't count on it",

			_ => "Error from 'message.1'",
		}
		_ => "Error from 'message.0'",
	};
	println!("{ball}");
}

fn eight_ball_via_ears() {
	println!("EIGHT BALL W/SOUND!");
	println!();

	// Experiment with different kind of tuple later. 

	// Shimmer's Voice
	let mut positive = (
		Sound::new("data/main_audio/yes-shimmer.mp3").unwrap(),
		Sound::new("data/main_audio/absolutely-shimmer.mp3").unwrap(),
		Sound::new("data/main_audio/certainly-shimmer.mp3").unwrap(),
	);

	let mut negative = (
		Sound::new("data/main_audio/no-shimmer.mp3").unwrap(),
		Sound::new("data/main_audio/absolutely_not-shimmer.mp3").unwrap(),
		Sound::new("data/main_audio/certainly_not-shimmer.mp3").unwrap(),
	);

	let mut probable = (
		Sound::new("data/main_audio/maybe-shimmer.mp3").unwrap(),
		Sound::new("data/main_audio/uncertain-shimmer.mp3").unwrap(),
		Sound::new("data/main_audio/dont_count_on_it-shimmer.mp3").unwrap(),
	);

	let mut error = (
		Sound::new("data/main_audio/error_from_message_dot_zero-shimmer.mp3").unwrap(),
		Sound::new("data/main_audio/error_from_message_dot_one-shimmer.mp3").unwrap(),
	);

	let delay = Duration::from_secs(1);

	println!(" Ask a Yes or No Question Below...");
	println!("-----------------------------------");
	let mut question = String::new();
	stdin().read_line(&mut question).expect("Failed to read line");
	println!("-----------------------------------");

	let message = (
		rand::thread_rng().gen_range(1..=3), // message.0
		rand::thread_rng().gen_range(1..=3), // message.1
	);

	match message.0 {
		1 => match message.1 {
			1 => { println!("Yes");                  positive.0.play(); sleep(delay); },
			2 => { println!("No");                   negative.0.play(); sleep(delay); },
			3 => { println!("Maybe");                probable.0.play(); sleep(delay); },

			_ => { println!("Error from message.1"); error.1.play();    sleep(delay); },
		}

		2 => match message.1 {
			1 => { println!("Absolutely");           positive.1.play(); sleep(delay); },
			2 => { println!("Absolutely not");       negative.1.play(); sleep(delay); },
			3 => { println!("Uncertain");            probable.1.play(); sleep(delay); }, 

			_ => { println!("Error from message.1"); error.1.play();    sleep(delay); },

		}
		3 => match message.1 {
			1 => { println!("Certainly");            positive.2.play(); sleep(delay); },
			2 => { println!("Certainly not");        negative.2.play(); sleep(delay); }, 
			3 => { println!("Don't count on it");    probable.2.play(); sleep(delay); },

			_ => { println!("Error from message.1"); error.1.play();    sleep(delay); },
		}

		_ => { println!("Error from message.0");         error.0.play();    sleep(delay); },
	}
}

fn counter() {
	println!("COUNTER!");

//	let mut counter = 0; // Start from zero
	let mut counter = 1; // Start from one
//	let mut counter = 2; // Start from two

	while counter < 11 { 	// Although you may not see 11 in the match expression, 
				// you should add one more point here. 
		let count = match counter {
			// In English

			0 => "Zero",

			1 => "One", 2 => "Two", 3 => "Three", 4 => "Four", 5 => "Five", 
			6 => "Six", 7 => "Seven", 8 => "Eight", 9 => "Nine", 10 => "Ten",


			_ => "out of bounds", // DO NOT ALLOW TO PRINT

		/*
			// En Espanol

			0 => "Cero",

			1 => "Uno", 2 => "Dos", 3 => "Tres", 4 => "Cuatro", 5 => "Cinco", 
			6 => "Ses", 7 => "Siente", 8 => "Ocho", 9 => "Nueve", 10 ="??", 

			_ = "", // 
		*/
		};

		println!("{count} seconds passed. ");
	//	println!("{count} "); // En Espanol
		sleep(Duration::from_secs(1));
		counter += 1;
	//	counter += 2; 
	}
}

fn counter_via_ears() {
	// Based on what I learned so far, you need to modify your code should you include additional features. 
	// On the surface, this function is exactly the same as the last. If I didn't want to include Shimmer's 
	// voice here, this would be considered bloatware. 

	println!("COUNTER W/SOUND!");
	let mut counter = 0;
	let s = "seconds passed";

	while counter < 11 {
		match counter { // We don't need 'let count =' here. Each integer already prints independently. 
			00 => { println!("Zero {s}"); }, 

			01 => { println!("One {s}"); }, 
			02 => { println!("Two {s}"); }, 
			03 => { println!("Three {s}"); }, 
			04 => { println!("Four {s}"); }, 
			05 => { println!("Five {s}"); }, 
			06 => { println!("Six {s}"); }, 
			07 => { println!("Seven {s}"); }, 
			08 => { println!("Eight {s}"); }, 
			09 => { println!("Nine {s}"); }, 
			10 => { println!("Ten {s}"); },

			_ => { println!("Error"); },
		};

		sleep(Duration::from_secs(1));
		counter += 1;
	}
}
