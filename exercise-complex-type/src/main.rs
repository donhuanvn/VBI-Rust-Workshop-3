<<<<<<< HEAD
use std::collections::HashMap;
use itertools::Itertools;

struct School {
    students: HashMap<String, u32>
=======
fn main() {
    println!("Hello, world!");
}


// Bài tập
// Cho ngữ cảnh như sau : Một ngôi trường cần lập danh
//sách thông tin sinh viên bao gồm tên sinh viên và điểm của sinh viên đó.
// với mục đích thống kê kiểm tra giáo dục của ngôi trường này


/*-----------------------------*/
// Gợi ý:
// Định nghĩa bằng struct, mọi người nên sử dụng HashMap 
// Tại sao lại sử dụng HashMap và ko phải Vec
//https://doc.rust-lang.org/std/collections/struct.HashMap.html
// struct School {
//     students: HashMap<String, u32>
// }

// trong trường hợp này thì String : tên của sinh viên đó
// u32 là điểm số 

// Một số yêu cầu như sau:

/*-----------------------------*/
//0. Tạo 1 function new() khởi tạo rỗng ban đầu cho danh sách

/*-----------------------------*/
//1. Có thể thêm thông tin của sinh viên gồm có tên và điểm
// Ví dụ: thêm tên "Alice" có 7 điểm, thêm tên "Bob" có 2 điểm, ...
// Gợi ý : định nghĩa hàm "add" implement cho Struct

/*-----------------------------*/
//2. Liệt kê các điểm số hiện tại mà trường đã cập nhập
// ví dụ :danh sách hiện tại gồm có [(Alice, 10), (Bob,4)]
//trả về là [4,10] (điểm số nên tăng dần và ko có duplicate)
// ví dụ: [(Alice, 10), (Bob,4), (Steve,4)] -> [4,10]

/*-----------------------------*/
//3. Liệt kê danh sách các học sinh có cùng 1 điểm số
// ví dụ: hiện tại danh sách gồm có (Alice, 3), (Bob, 10), (Charlie,3)
// liệt kê danh sách học sinh có cùng 3 điểm : [Alice, Charlie]

// Yêu cầu trả về: danh sách sinh viên là alphabet theo tên 

// Gợi ý: 
// ví dụ : Ban đầu [(Alice, 10), (Bob,2), (Eve,4), (Long,2)] -> [(Bob, 2), (Long,2), (Eve,4), (Alice,10)]
//định nghĩa hàm "find_student" có tham số là điểm số -> trả về danh sách các sinh viên có cùng điểm số mong muốn


// Các bạn phải vuợt qua một số test case như sau :

/*-----------------------------*/
//Test case 1: Khởi tạo đầu tiên danh sách phải rỗng

/*-----------------------------*/

// Test case 2:
//Thêm sinh viên có tên "Lee" với điểm số là 2
// thì tất cả các điểm số hiện có của trường là 2
//nếu thêm sinh viên khác "Nancy" với điểm số là 3
//thì các điểm số hiện tại là [2,3]

/*-----------------------------*/

// Test case 3:
// Giả sử danh sách hiện tại : [(Bob, 4), (Alice,4), (Tom,5)]
// với điểm số 4 thì ta có sinh viên nào: -> [Alice, Bob] not [Bob ,Alice]
// vì cần tên theo alphabet


/*-----------------------------*/
// Nếu mọi người làm xong rùi thì có thể làm advance hơn bằng cách 
// sử dụng Generic type cho điểm số không nhất thiết phải U32 nữa mà có thể "A+", "B+" chẳng hạn (string)
/*-----------------------------*/

// Sườn thông tin cho mọi người dễ làm



pub struct School {
    // !TODO
>>>>>>> eef58d8133729363ea926550dfd31fdc7bb82bf7
}

impl School {
    pub fn new() -> School {
<<<<<<< HEAD
        Self { students: HashMap::new() }
    }
    pub fn add(&mut self, student: &str, grade: u32) {
        self.students.entry(String::from(student)).or_insert(grade);
    }
    pub fn print_students(&self) {
        let count = self.students.len();
        println!("There is {count} students.");
        for (student, grade) in &self.students {
            println!("{student}: '{grade}'");
        }
    }
    pub fn grades(&self) -> Vec<u32> {
        let scores: Vec<u32> = self.students.values().cloned().collect();
        let scores: Vec<u32> = scores.into_iter().unique().sorted().collect();
        scores
    }
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut temp = self.students.clone();
        temp.retain(|_, v| *v == grade);
        let temp = temp.into_keys().sorted().collect_vec();
        temp
    }
}

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn main() {
    let mut school = School::new();
    // Testcase 1: Khoi tao danh sach rong
    println!("\n\nTestcase 1: Creating an empty school (vector)");
    school.print_students();

    println!("\n\nTestcase 2: Adding new students into the school");
    school.add("Lee", 2);
    school.add("Nancy", 3);
    school.print_students();
    
    println!("\n\nTestcase 3: Filtering students with a specified grade");
    school.add("Bob", 4);
    school.add("Alice", 4);
    school.add("Tom", 5);
    let students_with_grade_4 = school.grade(4);
    println!("Students with grade of 4: {:?}", students_with_grade_4);
}
=======
        unimplemented!()

    }

    pub fn add(&mut self, grade: u32, student: &str) {
        unimplemented!()

        
    }

    pub fn grades(&self) -> Vec<u32> {
        unimplemented!()

    }


    pub fn grade(&self, grade: u32) -> Vec<String> {
        unimplemented!()

        
    }
}










>>>>>>> eef58d8133729363ea926550dfd31fdc7bb82bf7
