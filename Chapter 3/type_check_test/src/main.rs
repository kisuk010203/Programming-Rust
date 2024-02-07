use regex::Regex;
fn main() {
    let big_val = std::i32::MAX;
    let x = big_val.wrapping_add(1); // overflow happens, but no panic. If we simply add, there will be panic.
    println!("big_val = {}, x = {}", big_val, x);
    println!("{:?}", Regex::new(r"([a-zA-Z]+)"));
}
#[test]
fn type_test() {
    assert_eq!(b'A', 65u8); // byte literals to show the ASCII value of a character

    // Conversions that are out of range produces values that are equivalent to the original modulo `2^N`.
    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(65535_u32 as i16, -1_i16);
    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);
}
#[test]
fn int_test() {
    //Some useful methods of `std` integer types. They also can have methods, as in a type class implementation.
    assert_eq!(2_i32.pow(4), 16);
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(4_i32.max(6), 6);
    assert_eq!(0b101101_u8.count_ones(), 4);
}
#[test]
fn float_test() {
    // Some useful methods of `std` float types.
    assert_eq!(2.0_f32.sqrt(), 1.4142135);
    assert_eq!(1.0_f32.exp(), 2.7182817);
    assert_eq!(2.0_f32.ln(), 0.6931472);
    assert_eq!(10.0_f32.log10(), 1.0);
    assert_eq!(2.0_f32.powf(4.0), 16.0);
    assert_eq!(4.0_f32.max(6.0), 6.0);
    assert_eq!(4.0_f32.min(6.0), 4.0);
    assert_eq!(4.5_f32.ceil(), 5.0);
    assert_eq!(4.5_f32.floor(), 4.0);
    assert_eq!(4.5_f32.round(), 5.0);
    assert_eq!(4.5_f32.trunc(), 4.0);
    assert_eq!((-4.5_f32).trunc(), -4.0);
    assert_eq!(4.5_f32.fract(), 0.5);
    assert_eq!((-4.5_f32).fract(), -0.5);
    assert_eq!((-4.5_f32).abs(), 4.5);
    assert_eq!((-4.5_f32).signum(), -1.0);
    assert_eq!((-4.5_f32).copysign(1.0), 4.5);
}
#[test]
fn pointer_test() {
    let x = 5;
    let y = &x; // This borrows a reference to x.
    let z = Box::new(x); // This allocates a value in the heap and returns a pointer to it. If x goes out of scope, z is freed immediately.
    assert_eq!(5, x);
    assert_eq!(5, *y); // This refers to the value of x, which y points to.
    assert_eq!(5, *z);
    // References are born to be immutable by default, unless we pass it like `&mut T`.
}
#[test]
fn sequence_test() {
    // Array : [T; N]
    // is a fixed-size list of elements of the same type. You cannot append new elements, or shrink it.

    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta", "Coleoptera"]; // Here we see that the size is inferred.
    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 4);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    assert!(sieve[211]);
    assert!(!sieve[9898]);

    let mut unsorted = [3, 4, 1, 2, 5];
    unsorted.sort();
    assert_eq!(unsorted, [1, 2, 3, 4, 5]);

    // Vector : Vec<T>
    // is a dynamically allocated, growable sequence of the same type, which is on the heap.

    let mut v = vec![2, 3, 5, 6];
    assert!(v.iter().fold(1, |a, b| a * b) == 180);

    v.push(11);
    v.push(13);
    assert!(v.iter().fold(1, |a, b| a * b) == 25740);

    let mut zero_to_five = (0..6).collect::<Vec<i32>>();
    assert_eq!(zero_to_five, [0, 1, 2, 3, 4, 5]);
    zero_to_five.reverse();
    assert_eq!(zero_to_five, [5, 4, 3, 2, 1, 0]);

    let mut cap_v = Vec::with_capacity(2);
    assert_eq!(cap_v.len(), 0);
    for i in 0..3 {
        cap_v.push(i);
    }
    assert_eq!(cap_v.len(), 3);
    assert_eq!(cap_v.capacity(), 4);

    let mut insert_v = vec![1, 2, 4, 5];
    insert_v.insert(2, 3);
    assert_eq!(insert_v, [1, 2, 3, 4, 5]);
    insert_v.remove(1);
    assert_eq!(insert_v, [1, 3, 4, 5]);

    let mut pop_v = vec![1, 2, 3];
    assert_eq!(pop_v.pop(), Some(3));
    assert_eq!(pop_v.pop(), Some(2));
    assert_eq!(pop_v.pop(), Some(1));
    assert_eq!(pop_v.pop(), None);
    assert_eq!(pop_v, []);

    // Slices : &[T] | &mut [T]
    // shared slice, mutable slice of Ts, references to a series of elements that are a part of some other value, like arr or vec.
    // Slice is a pointer to its first element, together with a count.

    let v = vec![1, 2, 3, 4]; // this is an owning pointer to the heap
    let a: [i32; 4] = [1, 2, 3, 4]; // this is array in the stack
    let _sv: &[i32] = &v; // this is a reference (non-owning pointer) to the vector in the stack
    let _sa: &[i32] = &a; // this is a reference to the array in the stack
}
#[test]
fn string_test() {
    let _speech = "\"Ouch!\" said the well.\n";
    let _pattern = Regex::new(r"([a-zA-Z]+)").unwrap();

    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);

    let noodles = "noodles".to_string(); //Inside the stack, a pointer that points to the heap. It owns the string.
    let oodles = &noodles[1..]; // Inside the stack, a reference that points to the heap. It borrows the string.
    let poodles = "ಠ_ಠ"; // Inside teh stack, a pointer that points to the preallocated read-only memory. It borrows the string.
    assert_eq!(oodles, "oodles");
    assert_eq!(poodles.len(), 7); // This is 7, not 3 because of the UTF-8 encoding which encodes ಠ.
    assert_eq!(poodles.chars().count(), 3);

    let mut _immutable_mut_string = "hello"; // We cannot mutate this string.
    let mut mutable_string = "hello".to_string(); // We can mutate this string.
    mutable_string.push('H');
    assert_eq!(mutable_string, "helloH");

    let tokens = ["veni", "vidi", "vici"];
    assert_eq!(tokens.join(", "), "veni, vidi, vici");
    assert_eq!(tokens.concat(), "venividivici");

    assert_eq!("ONE".to_lowercase(), "one");
    assert!("peanut".contains("nut"));
    assert_eq!("ಠ_ಠ".replace("ಠ", "■"), "■_■");
    assert_eq!("  clean\n  ".trim(), "clean");
    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"));
    }
}
