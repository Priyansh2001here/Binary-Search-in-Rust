mod single_line_vector_input;
mod multi_line_vec_input;
mod binary_search;


fn main() {

    let mut values: Vec<i32> = single_line_vector_input::input();
    let queries: Vec<i32> = multi_line_vec_input::input();
    values.sort();

    for x in queries.iter() {
        let mut temp : i32;
        temp = binary_search::binary_search(&values, *x);
        println!("{}", temp);
    }
}