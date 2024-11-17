use rand::Rng;

fn main() {
	enemy();
}

fn enemy() {
	let druggie = (
		rand::thread_rng().gen_range(1..=2),
	);

	let voice = (
		rand::thread_rng().gen_range(1..=3),
		rand::thread_rng().gen_range(1..=3),
	);

	match druggie.0 {
		1 => {
			println!("Druggie: What the fuck are you looking at?");
			match voice.0 {
				1 => { println!("voice one"); }, 
				2 => { println!("voice two"); }, 
				3 => { println!("voice three"); },

				_ => {},
			}

			println!("Druggie: I am looking. AT YOU!");
			match voice.1 {
				1 => { println!("voice one"); },
				2 => { println!("voice two"); },
				3 => { println!("voice three"); },

				_ => {},
			}
		}

		2 => {
			println!("Druggie: Get the fuck out of my face. ");
			match voice.0 {
				1 => { println!("voice one"); },
				2 => { println!("voice two"); },
				3 => { println!("voice three"); },

				_ => {},
			}

			println!("Druggie: ...");
			match voice.1 {
				1..3 => println!("no voice"),

				_ => {},
			}
		}
		_ => println!(),
	}
}

fn enemy_advance () {
	// Here, we need to dictate who is male or female.
}
