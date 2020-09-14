pub fn binary_search(values: &Vec<i32 >, value : i32) -> i32{

    let mut low = 0;
    let mut high  = values.len();

    while low < high {

        let mid = (low+high)/2;
        let key = values[mid];

        if key > value {
            high = mid;
        }
        else if key < value {
            low = mid;
        }
        else if key == value {
            return mid as i32;
        }

    }

    return -1;

}