pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut starting_index = 0;
    let mut ending_index = array.len() as isize - 1;

    while starting_index <= ending_index {
        let middle_point = (starting_index + ending_index) / 2;
        let middle_number = array[middle_point as usize];

        if middle_number == key {
            return Some(middle_point as usize);
        } else if middle_number < key {
            starting_index = middle_point + 1;
        } else {
            ending_index = middle_point - 1;
        }
    }

    None
}

