
//Exercise 3
// Mục đích: giải quyết vấn đề ownership and borrowing ko dùng clone()
// fn main() {
//     let mut values = vec![10, 11, 12];
//     let v = &mut values;

//     let mut max = 0;

//     //for n in &mut values {
//     for n in v {
//         max = std::cmp::max(max, *n);
//     }

//     println!("max is {}", max);
//     println!("Converting to percentages of maximum value...");
//     //for n in &mut values {
//     for n in v {
//         *n = 100 * (*n) / max;
//     }
//     println!("values: {:#?}", values);
// }

// ==================== SOLUTIONS =============================
fn main() {
    let mut values = vec![10, 11, 12];
    let v = &mut values;

    let mut max = 0;
    for n in v.iter() {                 // this is a fix
        max = std::cmp::max(max, *n);
    }
    println!("max is {}", max);

    println!("Converting to percentages of maximum value...");
    for n in v.iter_mut() {         // this is a fix
        *n = 100 * (*n) / max;
    }

    println!("values: {:#?}", values);
}
