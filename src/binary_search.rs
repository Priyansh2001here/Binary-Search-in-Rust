use std::cmp::Ordering;

pub fn binary_search(values: &Vec<i32 >, value : i32) -> i32{

    let mut low = 0;
    let mut high  = values.len();

    while low < high {

        let mid = (low+high)/2;

        match values[mid].cmp(&value) {
            Ordering::Greater => {
                high = mid;
            },
            Ordering::Less => {
                low = mid;
            },
            Ordering::Equal => {
                return mid as i32;
            }
        }
    }

    return -1;

}