use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // Sơ đồ mô tả cấu trúc danh sách dùng chung:
    // b: Cons(3) ──┐
    //              ├──> a: Cons(5, Nil)  <--- Danh sách dùng chung (Shared)
    // c: Cons(4) ──┘

    // 1. Tạo một RefCell chứa giá trị 5 bọc trong Rc để có thể chia sẻ và thay đổi nội dung bên trong sau này.
    let value = Rc::new(RefCell::new(5));

    // 2. `a` BẮT BUỘC phải dùng `Rc::new(...)` (kiểu Rc<List>):
    // Vì `a` là nút dùng chung (shared node) mà cả `b` và `c` đều trỏ tới.
    // Việc bọc trong `Rc::new` cho phép ta tạo nhiều con trỏ bằng `Rc::clone(&a)`.
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // 3. `b` và `c` KHÔNG CẦN `Rc::new(...)`:
    // Vì `b` và `c` là điểm đầu của 2 nhánh, không có biến nào khác trỏ vào chúng.
    // Chúng chỉ cần là kiểu `List` bình thường (Cons), nhận `Rc::clone(&a)` làm tham số thứ 2.
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // Thay đổi giá trị bên trong RefCell thông qua con trỏ `value`
    *value.borrow_mut() += 10;

    println!("a = {a:?}");
    println!("b = {b:?}");
    println!("c = {c:?}");
}
