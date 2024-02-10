struct Person {
    name: String,
    birth: i32,
}
fn make_composers_and_print() {
    let mut composers = Vec::new(); // owns a vector, vector owns its elements
    composers.push(Person{name: "Palestrina".to_string(), birth: 1525}); // each Person structure owns its fields, string owns its text
    composers.push(Person{name: "Dowland".to_string(), birth: 1563});
    composers.push(Person{name: "Lully".to_string(), birth: 1632});

    let first_name = &composers[0].name; // error: cannot move out will occur without `&`, which means that it merely borrows, not move.
    assert_eq!(first_name, "Palestrina");

    for composer in composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
}
fn print_padovan() {
    let mut padovan = vec![1, 1, 1]; // allocated on the heap
    for i in 3..10 {
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan); // padovan is deallocated here
}
fn main() {
    print_padovan();
    make_composers_and_print();
}
#[test]
/// `Box<T>` is a pointer to a heap-allocated T stored on the heap. Calling `Box::new(v)` 
/// allocates some heap space, moves the value v into it, and returns a `Box` pointing th the heap space. 
/// Whenever `Box` is dropped, the space is freed as well.
fn box_test() {
    let point = Box::new((0.625, 0.5)); // point allocated here
    let label = format!("{:?}", point); // label allocated here
    assert_eq!(label, "(0.625, 0.5)"); // both dropped here
}
#[test]
/// When a value is assigned, the value is not copied, it is moved. Hence, the ownership of the value is relinquished.
/// ```
/// a = [1, 2, 3]
/// b = a
/// c = a
/// ```
/// In the above python code, when we alter a, b and c are also altered. This is not the case in Rust.
/// Python makes assignment cheap, and the expense of requiring reference counting or garbage collection to manage the memory.
/// 
/// ```
/// vector<int> a = {1, 2, 3};
/// vector<int> b = a;
/// vector<int> c = a;
/// ```
/// In the above c++ code, when we alter a, b and c is not altered. However, even this is not the case in Rust.
/// C++ keeps all the ownership of memory clear, at the expense of making assignment carry out a deep copy. expensive :( 
fn test_move() {
    let s = vec![1, 2, 3];
    let t = s; // this moves the value of s into t, which makes s uninitialized.
    // let u = s; // error: value used here after move
    let u = t.clone();
    assert_eq!(t, vec![1, 2, 3]);
    assert_eq!(u, vec![1, 2, 3]);
}
#[test]
fn move_assign() {
    let mut s = "HELLO".to_string();
    let t = s;
    s = "WORLD".to_string(); // this is a reassign.
    assert_eq!(t, "HELLO");
    assert_eq!(s, "WORLD");
}
/* So why is this `move` good? 
    1. It always apply to the `value proper`, not the heap storage they own. Hence, it is cheap.
    2. This is compiler friendly, hence the machine code stores the value directly where it belong.
 */
#[test]
fn pop_element() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // It is easy to remove the end of a vector.
    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105");
    assert_eq!(v, vec!["101", "102", "103", "104"]);

    // Moving a middle of the vector and move the last element to its spot.
    let second = v.swap_remove(1);
    assert_eq!(second, "102");
    assert_eq!(v, vec!["101", "104", "103",]);

    // Swap in another value for the one we're taking out.
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");
    assert_eq!(v, vec!["101", "104", "substitute"]);

    for mut i in v { // This moves the vector out of v, leaving v uninitialized.
        i += "2";
        println!("{}", i);
    }
    // println!("{:?}", v); // error: value used here after move


    // let third = v[2]; // cannot move out of index of Vec<String>, this is a move because value has `String` type.
    // let fifth = v[4];
    // assert_eq!(third, "103");
    // assert_eq!(fifth, "105");
}
