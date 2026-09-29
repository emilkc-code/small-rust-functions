// Lizi Sort
pub fn lizi_sort <T:PartialOrd>(mut list: Vec<T>) -> Vec<T> {
    let mut place:       usize = 1;
    let mut move_right:   bool = true;
    let mut pass_number: usize = 1;
    let mut run_number:  usize = 1;

    // Sort first two values
    if list.get(place + 1).is_some() && list[place] > list[place + 1] {
        list.swap(place, place + 1);
    }

    loop {
        // Sort with the right neighbor
        let sort_right = |list: &mut Vec<T>| {
            if list.get(place + 1).is_some() && list[place] > list[place + 1] {
                list.swap(place, place + 1);
            }
        };
    
        // Sort with the left neighbor
        let sort_left = |list: &mut Vec<T>| {
            if list[place - 1] > list[place] {
                list.swap(place - 1, place);
            }
        };

        // Sorts with the right neighbor first if we are moving to the right in the vector
        if move_right {
            sort_right(&mut list);
            sort_left(&mut list);
        }
        // Inverse
        else {
            sort_left(&mut list);
            sort_right(&mut list);
        }

        // When we are at the second last one
        if move_right && place == list.len() - (2 * run_number - 1) {
        //if move_right && place >= list.len() - 1 {
            if pass_number > (list.len() + 1) / 2 {
                return list;
            }
            move_right = false;
            pass_number += 1;
        }

        // When we come back to the second one
        else if !move_right && place == 2 * run_number - 1 {
        //else if !move_right && place <= 1 {
            // When we have gone through the vector a certain amount of times, it's guaranteed to be sorted
            if pass_number > (list.len() + 3) / 4 + 2 {
            //if pass_number > 2 {
                return list;
            }
            move_right = true;
            pass_number += 1;
            run_number += 1;
        }

        // Moves our "pointer" depending on which direction we are going
        if move_right {
            place += 1;
        }
        else {
            place -= 1;
        }
    }
}

