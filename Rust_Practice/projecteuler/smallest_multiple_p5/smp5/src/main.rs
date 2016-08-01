// Solution to Project Euler problem 5
use std::io::{self, Write};
///Global input
const A:[i64;20] = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20];

///Main function
fn main() {
    let first_result = first_attempt();
    println!("First attempt result = {}",first_result);

    let second_result = take_two();
    println!("Second attempt result = {}",second_result);
	let mut input = String::new();
	io::stdin().read_line(&mut input);
}

///First attempt (fails)
// Incrementally multiples the primes between 1 and 20
// together and checks to see if any of the results are solutions
fn first_attempt() -> i64 {
    let pa:[i64;8] = [2,3,5,7,11,13,17,19]; //primes in A discluding 1 (which is init of 'cur')

    let mut cur:i64 = 1; //1*2*3*5*7*11*13*17*19 = 9699690

    //incerementally multiply together the values in pa
    //checking if any of the results are solutions
    for i in 0..8 {
        cur = cur * pa[i];
        println!("cur = {}",cur);
        if check_num(cur) { 
            return cur; //solution!
        }
    }

    return -1; //failed to find a potential solution
}

///Second attempt (fails)
// If the numbers in rm are divisors of the number
// than all other numbers in range 1-20 will be as well.
// Using the same algorith as first attempt, but with rm 
// instead of pa
fn take_two() -> i64 {
    let rm:[i64;8] = [13,14,15,16,17,18,19,20]; //required multiples, the rest of the numbers are trivially included

    let mut cur:i64 = 11; 

    //incerementally multiply together the values in pa
    //checking if any of the results are solutions
    for i in 0..8 {
        cur = cur * rm[i];
        println!("cur = {}",cur);
        if check_num(cur) { 
            return cur; //solution!
        }
    }

    return -1; //fail face
}


///Check if a number is a multiple of 1-20
fn check_num(check:i64)->bool{
    for i in 1..21 {
        if check%i != 0 { //check if it's evenly divisible (no remainder)
            println!("nope!");
            return false;
        }
    }
    println!("yep!");
    return true;
}
