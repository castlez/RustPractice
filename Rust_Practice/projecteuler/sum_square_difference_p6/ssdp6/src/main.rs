//this program solve project euler question #6
//what is the different between the sum of squares up to 100 
//and the sqaure of the sum of numbers up to 100
//TODO: use lambdas!
use std::io::{self,Write};
fn main() {
    let mut     r = 0..101;
    let mut sumos = 0;
    let mut squaos= 0;
    
    for x in r { //TODO: use lambdas instead of loops!
        sumos  += x*x;
        squaos += x;
    }

    let diff = (squaos * squaos) - sumos;

    println!("diff = {}",diff);
	let mut input = String::new();
	io::stdin().read_line(&mut input);
}
