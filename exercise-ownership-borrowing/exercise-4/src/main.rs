//Exercise 4
// Mục đích : giải quyết vấn đề ownership và borrowing ko dùng clone()
// // Logic hiện tại đang sai (cho 1 vec -> đảo chiều vector đó)
// fn main() {
//     let a = vec![1, 2, 3, 4, 5];
//     let i = 0;
//     let c = 0;
//     loop {
//         let (a, c) = test(a);
//         println!("{}", c);
//         if i >= 5 {
//             break;
//         }
//     }
// }
// pub fn test(mut a: Vec<u8>) -> (Vec<u8>, i32) {
//     let mut b: Vec<u8> = Vec::new();
//     let mut c: u8 = 0;
//     loop {
//         if a.len() == 0 {
//             break;
//         }
//         let d = a.pop().unwrap();
//         c = c + d;
//         b.push(d);
//     }
//     (b, c as i32)
// }

// ==================== SOLUTIONS =============================
fn main() {
    let a = vec![1, 2, 3, 4, 5];
    
    let b = vec_reverse(&a);

    println!("{:?}", b); // [5, 4, 3, 2, 1]
}
pub fn vec_reverse(a: &Vec<u8>) -> Vec<u8> {
    let mut b: Vec<u8> = Vec::new();

    let len = a.len();
    let rev_indices = (0..len).rev();

    for i in rev_indices {
        b.push(a[i]);
    }

    b
}
