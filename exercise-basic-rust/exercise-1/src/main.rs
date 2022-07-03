/*
Bài tập 1: Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ? (yêu cầu đúng thứ tự của các phần tử)
Ví dụ :
    let org_arr = [1, 2,3,5,6,8, 10, 11];
    let sub_arr = [6,8,10];
*/
fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];

    let mut is_child = true;
    for sub_item in sub_arr.iter() {
        let mut found = false;
        for org_item in org_arr.iter() {
            if sub_item == org_item {
                found = true;
                break;
            }
        }
        if !found {
            is_child = false;
            break;
        }
    }

    println!("The result: {}", is_child);
}
