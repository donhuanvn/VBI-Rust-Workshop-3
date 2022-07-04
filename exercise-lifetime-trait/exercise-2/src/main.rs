// Bài 2: Lifetime
// Yêu cầu: Sửa lỗi Lifetime

// use std::fmt;
// struct StrDisplayable(Vec<&str>);

// impl fmt::Display for StrDisplayable {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for v in &self.0 {
//             write!(f, "\n{}", v)?;
//         }
//         Ok(())
//     }
// }

// fn main() {
//         let vec: Vec<&str> = vec!["a","bc","def"];
//         let vec_Foo = StrDisplayable(vec);
//         println!("{}",vec_Foo);
// }

use std::fmt;
struct StrDisplayable(Vec<&'static str>); // this is an only change.

impl fmt::Display for StrDisplayable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in &self.0 {
            write!(f, "\n{}", v)?;
        }
        Ok(())
    }
}

fn main() {
    let vec: Vec<&str> = vec!["a", "bc", "def"];
    let vec_Foo = StrDisplayable(vec);
    println!("{}", vec_Foo);
}
