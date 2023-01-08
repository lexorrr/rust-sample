fn main() {
    // let s1 = String::from("hello");

    // let len = calculate_length(&s1);

    // println!("The length of '{s1}' is {len}.");

    // Mutable reference
    // let mut s = String::from("hello");

    // change(&mut s);

    // Mutable reference restriction: reference one variable at a time!
    let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    // println!("{r1} , {r2}");

    // Combining mutable and immutable references
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!