use std::rc::Rc;

#[derive(Clone, Copy)] // This only works for types that implement the Copy trait
struct MyNumber {
    number: u32,
}
fn print_my_number(my_n: MyNumber) {
    println!("My Number is : {}", my_n.number);
}

#[test]
fn test_copy_type() {
    let str1 = "somnambulance".to_string();
    let str2 = str1;
    assert_eq!(str2, "somnambulance");
    // assert_eq!(str1, "somnambulance"); // error[E0382]: borrow of moved value: `str1`

    let num1: i32 = 36;
    let num2 = num1; // There is no move here, because i32 is a Copy type
    assert_eq!(num2, 36);
    assert_eq!(num1, 36);
}
#[test]
/// For any type `T`, `Rc<T>` is a pointer to a heap-allocated `T` that is reference-counted. Cloning `Rc<T>` simply
/// increments the reference count, and creates another point to it.
fn test_rc() {
    let rc = Rc::new("hello".to_string());
    let rc1 = rc.clone();
    let rc2 = rc.clone();
    assert!(rc.contains("ello"));
    assert_eq!(rc1.find("llo"), Some(2));
    assert_eq!(rc2.to_owned(), "hello".to_string().into());
}
fn main() {
    println!("Hello, world!");

    let my_n = MyNumber { number: 5 };
    print_my_number(my_n);
    print_my_number(my_n); // error[E0382]: use of moved value: `myN` should occur, but by deriving Copy, Clone, this type becomes a Copy type.
}
