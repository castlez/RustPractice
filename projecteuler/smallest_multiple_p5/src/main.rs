//solution to Project Euler problem 5

//global input
const A:[i64;20] = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20];

fn main() {
    let mut sm = -1;
    let pa:[i64;8] = [2,3,5,7,11,13,17,19]; //primes in A discluding 1 (which is init of 'cur')

    let mut cur:i64 = 1; //1*2*3*5*7*11*13*17*19 = 9699690

    //incerementally multiply together the primes in A
    for i in 0..8 {
        cur = cur * pa[i];
        println!("cur = {}",cur);
        if check_num(cur) {
            sm = cur;
        }
    }

    println!("Result = {}",sm);
}

//check if a number is a multiple of 1-20
fn check_num(check:i64)->bool{
    for i in 1..21 {
        if check%i != 0 { //if it's evenly divisible (no remainder)
            println!("nope!");
            return false;
        }
    }
    println!("yep!");
    return true;
}
