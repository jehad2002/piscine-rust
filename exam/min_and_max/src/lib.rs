pub fn min_and_max(nb_1: i32, nb_2: i32, nb_3: i32) -> (i32, i32) {
    let min_val = nb_1.min(nb_2).min(nb_3);
    let max_val = nb_1.max(nb_2).max(nb_3);

    (min_val, max_val)
}
