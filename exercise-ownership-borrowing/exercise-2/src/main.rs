//Exercise 2
// Mục đích: giải quyết vấn đề ownership và borrowing ko dùng clone()
// Các bạn có thể sửa thêm logic để đúng với mục đichs bài này là liệt kê các số nguyên tố
// fn main() {
//     let mut count: u32 = 1;
//     let mut num: u64 = 1;
//     let mut primes: Vec<u64> = Vec::new();
//     primes.push(2);

//     while count < 10 {
//         num += 2;
//         if vector_is_prime(num, primes) {
//             count += 1;
//             primes.push(num);
//         }
//     }
//     println!("{:?}", primes);
// }

// fn vector_is_prime(num: u64, p: Vec<u64>) -> bool {
//     for i in p {
//         if num > i && num % i != 0 {
//             return false;
//         }
//     }

//     true
// }

fn main() {
    let mut i = 2;

    let mut found_primes: Vec<u32> = Vec::new();
    
    while found_primes.len() < 10 {
        if is_prime(i) {
            found_primes.push(i)
        }
        i += 1;
    }
    
    println!("{:?}", found_primes); // [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
}

// reference: https://www.geeksforgeeks.org/c-program-to-check-whether-a-number-is-prime-or-not/
fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    let upper_bound = (num as f64).sqrt() as u32;
    for i in 2..=upper_bound {
        if num % i == 0 {
            return false;
        }
    }
    true
}

