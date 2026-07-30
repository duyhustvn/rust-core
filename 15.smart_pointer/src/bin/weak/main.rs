use std::cell::RefCell;
use std::rc::{Rc, Weak};

// Struct Node biểu diễn một nút (node) trong mô hình cấu trúc dữ liệu cây (Tree)
#[derive(Debug)]
struct Node {
    // Giá trị dữ liệu kiểu số nguyên i32 mà nút lưu trữ
    value: i32,

    parent: RefCell<Weak<Node>>,
    // RefCell<T>: Cho phép thay đổi danh sách nút con tại runtime ngay cả khi Node là immutable (Interior Mutability).
    // Vec<Rc<Node>>: Mảng động chứa các con trỏ chia sẻ quyền sở hữu Rc<Node> trỏ tới các nút con (Multiple Ownership).
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // Bước 1: Tạo nút lá (leaf) có value = 3
    // Ban đầu nút lá chưa có nút cha (parent là Weak rỗng) và chưa có nút con nào.
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        // Bước 2: Tạo nút nhánh/nút cha (branch) có value = 5
        // Thêm `leaf` vào danh sách nút con `children` của `branch` thông qua `Rc::clone(&leaf)`.
        // Điều này tăng `strong_count` của `leaf` lên 2 (leaf variable + branch's children).
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // Bước 3: Tạo liên kết ngược từ nút lá `leaf` trỏ lên nút cha `branch`
        // Sử dụng `Rc::downgrade(&branch)` để tạo một con trỏ yếu `Weak<Node>` trỏ tới `branch`.
        // `Rc::downgrade` tăng `weak_count` của `branch` nhưng KHÔNG tăng `strong_count`,
        // giúp tránh hiện tượng vòng lặp tham chiếu (Reference Cycle / Memory Leak).
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    // Bước 4: Kiểm tra nút cha của `leaf` từ con trỏ yếu `parent`
    // Hàm `upgrade()` kiểm tra xem đối tượng mà `Weak` trỏ tới còn tồn tại không:
    // - Nếu còn: Trả về `Some(Rc<Node>)`
    // - Nếu đã bị deallocate: Trả về `None`
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
