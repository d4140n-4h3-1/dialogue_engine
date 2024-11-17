use std::io::stdin;

fn main() {
	let mut choice = (
		String::new(), // choice.0
		String::new(), // choice.1
		String::new(), // choice.2
	);

	println!("Your options are A, B and  C. ");

	stdin().read_line(&mut choice.0).ok();

	match &choice.0[..1] {
		"a" => { 
			println!("You Chose A");
			println!("Step 1"); 

			stdin().read_line(&mut choice.1).ok();
			match &choice.1[..1] {
				"a" => { 
					println!("Step 1.1");

					stdin().read_line(&mut choice.2).ok();
					match &choice.2[..1] {
						"a" => { println!(""); },
						"b" => { println!(""); },
						"c" => { println!(""); },

						_ => { println!(""); },
					} 
				},
				"b" => { 
					println!("Step 1.2");

					stdin().read_line(&mut choice.2).ok();
					match &choice.2[..1] {
						"a" => { println!(""); },
						"b" => { println!(""); },
						"c" => { println!(""); },

						_ => { println!(""); },
					}
				},
				"c" => { 
					println!("Step 1.3");

					stdin().read_line(&mut choice.2).ok();
					match &choice.2[..1] {
						"a" => { println!(""); },
						"b" => { println!(""); },
						"c" => { println!(""); },

						_ => { println!(""); },
					}
				},

				_ => { 
					println!("Error. Returning. "); 
				},
			}
		},
		"b" => { 
			println!("You chose B");
			println!("Step 1"); 

			stdin().read_line(&mut choice.1).ok();
			match &choice.1[..1] {
				"a" => { 
					println!("Step 1.1"); 

					stdin().read_line(&mut choice.2).ok();
					match &choice.2[..1] {
						"a" => { println!(""); },
						"b" => { println!(""); },
						"c" => { println!(""); },

						_ => { println!(""); },
					}
				},
				"b" => { 
					println!("Step 1.2"); 

					stdin().read_line(&mut choice.2).ok();
					match &choice.2[..1] {
						"a" => { println!(""); },
						"b" => { println!(""); },
						"c" => { println!(""); },

						_ => { println!(""); },
					}
				},
				"c" => { 
					println!("Step 1.3"); 

					stdin().read_line(&mut choice.2).ok();
					match &choice.2[..1] {
						"a" => { println!(""); },
						"b" => { println!(""); },
						"c" => { println!(""); },

						_ => { println!(""); },
					}
				},

				_ => { 
					println!("Error. Returning. "); 
				},
			}
		},
		"c" => { 
			println!("You chose C");
			println!("Step 1");

			stdin().read_line(&mut choice.1).ok();
			match &choice.1[..1] {
				"a" => { 
					println!("Step 1.1"); 

					stdin().read_line(&mut choice.2).ok();
					match &choice.2[..1] {
						"a" => { println!(""); },
						"b" => { println!(""); },
						"c" => { println!(""); },

						_ => { println!(""); },
					}
				},
				"b" => { 
					println!("Step 1.2"); 

					stdin().read_line(&mut choice.2).ok();
					match &choice.2[..1] {
						"a" => { println!(""); },
						"b" => { println!(""); },
						"c" => { println!(""); },

						_ => { println!(""); },
					}
				},
				"c" => { 
					println!("Steop 1.3"); 

					stdin().read_line(&mut choice.2).ok();
					match &choice.2[..1] {
						"a" => { println!(""); },
						"b" => { println!(""); },
						"c" => { println!(""); },

						_ => { println!(""); },
					}
				},

				_ => { 
					println!("Error. Returning"); 
				},
			} 
		},

		_ => { 
			println!("Error. Returing. "); 
		},
	}
}
