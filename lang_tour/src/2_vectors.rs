// 2_vectors

pub fn run() {
    /* Note: Vectors are like arrays, except that
    their content or length doesn't need to be known
    in advance and can grow on demand. */

    let mut number_vec: Vec<u8> = Vec::new();
    number_vec.push(2);
    number_vec.push(33);

    let mut vec_with_macro = vec![2];
    vec_with_macro.push(33);
    vec_with_macro.push(34);
    vec_with_macro.pop(); // value ignored with `_`

    let message = if number_vec == vec_with_macro {
        "They are equal"
    } else {
        "Nah! They look different to me"
    };

    println!("{} {:?} {:?}", message, number_vec, vec_with_macro);
}