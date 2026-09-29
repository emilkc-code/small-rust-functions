use sorting_algorithm::lizi_sort;

use rand::seq::SliceRandom;

fn main() {
    let mut list: Vec<u32> = (1..=200).collect();
    list.shuffle(&mut rand::thread_rng());
    //let list = vec!["Snoofl", "Arty", "sploinky", "Lizi", "lich", "llfy", "matcha creampie", "Archy", "renxo", "umbra", "mcsheeranWolf", "t0pzy", "doc the racist"];
    let sorted = lizi_sort(list);
    println!("{:?}", sorted);
}