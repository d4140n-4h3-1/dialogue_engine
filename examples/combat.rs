use std::io::stdin;
use rand::Rng;

fn main() {
	println!("YOU ARE IN COMBAT. ");
	println!();

	println!("HINT: Trolls are allergic to iron. If you happen to be weilding ");
	println!("      steel, their skin is likely to blister and soften. Pure iron ");
	println!("      blades or arrows with poison are most effective. "); 
	println!(); 

	println!("Choose an attack");
	let _troll_health = 100; 

	println!("A) SWORD"); 
	println!("B) BOW"); 
	println!("C) HEAL"); 
//	println!("D) Evade"); 
	println!();

	let mut choice = (
		String::new(),
		String::new(),
		String::new(),
	);

	stdin().read_line(&mut choice.0).ok();
	match &choice.0[..1] {
		"a" | "A" => { 
			let sword = rand::thread_rng().gen_range(1..=22);
			match sword {
				1..=18  => { println!("You slashed at the troll, negating -5 health points. "); },
				19..=21 => { println!("You stabbed the troll center mass. -15 health points. "); },
				22      => { println!("You struck the face, leaving him a scar. He isn't too happy about that. -5 health points.  "); },

				_ => { println!("error"); },
			}
		},
		"b" | "B" => { 
		//	Ask to poison iron arrows if available

			println!("A) Wood Arrow");
			println!("B) Steel Arrow");
			println!("C) Iron Arrow");
			println!();
			let bow = rand::thread_rng().gen_range(1..37);
			match bow {
				1..=25  => {
					stdin().read_line(&mut choice.1).ok();
					match &choice.1[..1] {
						"a" | "A" => {
							println!("You struck the troll with a non-lethal blow. -2 health points.  ");
						},
						"b" | "B" => {
							println!("While not even lethal to the weakest elf, the mere fact your arrowheads are made of steel is enough to burn the troll's skin. -10 health points. ");
						},
						"c" | "C" => {
							println!("Pure iron arrows are normally unfavored due to their vulnerability to rust. However, they are not obsolete when it comes to creatures as tough as trolls. -10 points plus loss of focus. ");
						},
						_ => println!("error"),
					} 
				}
				26..=30 => {
					stdin().read_line(&mut choice.1).ok();
					match &choice.1[..1] { 
						"a" | "A" => {
							println!("You grazed the troll in the face. -5 health points. "); 
						},
						"b" | "B" => {
							println!("You grazed the troll in the face. Although, he's got a couple of blisters. -10 health points. ");
						},
						"c" | "C" => {
							println!("You grazed the troll in the face, but somehow it's so bad he needs plastic surgery. -15 health points. "); 
						}
						_ => println!("error"), 
					}
				},
				31..=36 => { 
					stdin().read_line(&mut choice.1).ok();
					match &choice.1[..1] {
						"a" | "A" => {
							println!("You indeed directed your shot to the heart, but the troll is so tough the arrow couldn't penetrate far enough into his chest. -10 health points. "); 
						},
						"b" | "B" => {
							println!("Not only did you hit the troll centermass, you also blistered and tenderized his thick skin. -10 points and -20 damage threshold. ");
						},
						"c" | "C" => {
							println!("The troll did not know what hit him. -20 points -30 damage threshold. "); 
						},
						_ => println!("error"),
					}
				},
				37      => { 
					stdin().read_line(&mut choice.1).ok();
					match &choice.1[..1]{
						"a" | "A" => {
							println!("You officially ended his manhood. But rather than falling to his knees, you pray to the gods he wouldn't go berserk. -15 health points.  "); 
						},
						"b" | "B" => {
							println!("Not only did you end his ability to sire children, you also made his loins look like he has the raging herpes. -20 points plus dilerium.");
						},
						"c" | "C" => {
							println!("Luckily, he fell wimpering like a crab grabbing a dog's tail. Now the outside of his pants is covered in pus and green blood. -100 health points, +100 shame. ");
						},
						_ => println!("error"),
					}
				},

				_ => { println!("error"); },
			}
		},
		"c" | "C" => { 
			println!("How would you like to heal? "); 
			println!(); 
			println!("A) STONE OF LIFE"); 
			println!("   This rare stone grants you full health. Use this as your last "); 
			println!("   resort, because it only recharges once every three battles. ");
			println!("   from the first use. "); 
			println!("   ");
			println!("B) PATCH");
			println!("   You are carrying five bandages. Restores 20 points per use. ");
			println!("   ");
			println!("C) FIRE");
			println!("   Fire can burn and sterilize wounds to stop infection. "); 
			println!("   Painful, but can be life-saving. "); 
			println!(); 

			stdin().read_line(&mut choice.1).ok();
			match &choice.1[..1] {
				"a" | "A" => { 
					println!("Are you sure you want to use this? Any key other than 'y' defaults to 'n'. "); 
					println!("Y) Yes");
					println!("N) No");  
					println!();

					stdin().read_line(&mut choice.2).ok();
					match &choice.2[..1] {
						"y" | "Y" => { println!("Your health is fully restored! "); },
						_ => { println!("You probably made a wise choice. "); },
						// The line above doesn't need to be labeled as "n | N" in this case. 
						// On the surface, it's the same with or without it. 

					//	_ => println!("error"),
					}
				},
				"b" | "B" => { println!("You patch your wounds. "); },
				"c" | "C" => { println!("You seal your wounds via fire magic. "); }, 

				_ => { println!("error"); },
			}
		},

/*		"d" | "D" => {
			print!();
			match {
				1 => {},
				2 => {},
				3 => {},

				_ => {},
			}
		} */

		_ => { println!("Error"); },
	} 
}
