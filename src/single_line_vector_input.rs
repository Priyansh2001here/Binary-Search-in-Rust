use std::io;
use std::process::exit;

pub fn input () -> Vec<i32>{
    let mut num_of_vals = String::new();

    std::io::stdin().read_line(&mut num_of_vals).expect("Error taking Input");

    let num_of_vals : i32 = match num_of_vals.trim().parse() {
        Ok(value) => value,
        Err(_e) => {
            println!("Please enter a integer");
            exit(0);
        }
    };

    let mut values = String::new();
    io::stdin().read_line(&mut values).expect("Unable to take input");

    let values : Result<Vec<i32>, _> = values.trim().split_whitespace().map(|x| x.parse()).collect();


    let values = values.ok().unwrap();
    let len = values.len() as i32;

    assert_eq!(len, num_of_vals);

    return values;

}