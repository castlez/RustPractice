// Character creator,
//Jonny Castle

// saves characters with inventory and names 
// in a text file, and create new ones

use std::io::{self,Write};

struct character {
	name: String,
	inven: Vec<String>
}

fn main() {
    println!("Welcome to  character creator!");
	let mut ans = String::new();
	loop {
		println!("Select an option:");
		println!("1. Create a character");
		println!("2. Look at characters");
		println!("3. Quit");

		io::stdin().read_line(&mut ans);
		println!("ans = {}", &ans);
		if ans.trim() == "1" {
			println!("TODO");
		}
		else if ans.trim() == "2" {
			println!("TODO");
        }
		else if ans.trim() == "3" {
			println!("KTHXBYE");
			break;
        }
	}

	io::stdin().read_line(&mut ans);
	
}

fn make_char() {
	let mut char_name = String::new();
	let mut char_inven = Vec::<String>::new();

	print!("Name >> ");
    io::stdout().flush().unwrap();
	io::stdin().read_line(&mut char_name);
	
	let mut count = 0;
	let mut temp : String = String::new();
	loop {
		print!("Item {} >> ",count);
		io::stdout().flush().unwrap();
		io::stdin().read_line(&mut temp);
		char_inven.push(temp);



    }
}