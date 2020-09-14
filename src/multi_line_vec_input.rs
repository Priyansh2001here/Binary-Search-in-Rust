use std;
use std::process::exit;

pub fn string_to_int(x : String) -> i32 {

    let x : i32 = match x.trim().parse() {
        Ok(value) => value,
        Err(_e) => {
            println!("Enter an integer");
            exit(0) }
    };

    return x;
}

pub fn input() -> Vec<i32> {

    let mut num_of_vals = String::new();

    std::io::stdin().read_line(&mut num_of_vals).expect("Error taking Input");

    let num_of_vals = string_to_int(num_of_vals);

    let mut vec : Vec<i32> = Vec::new();

    for _ in 0..num_of_vals{

        let mut temp = String::new();
        std::io::stdin().read_line(&mut temp).expect("Unable to take input");

        let temp = string_to_int(temp);
        vec.push(temp);

    }

    return vec;

}