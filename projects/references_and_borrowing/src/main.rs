fn main() {
    let mut s1 = String::from("hello");

    // The &s1 syntax lets us create a reference that refers to the value of
    // s1 but does not own it. Because it does not own it, the value it points
    // to will not be dropped when the reference stops being used.
    //
    // A reference is like a pointer in that it's an address we can follow to
    // access the data stored at that address; that data is owned by some
    // other variable.
    //
    // Unlike a pointer, a reference is guaranteed to point to a valid value
    // of a particular type for the life of that reference.
    //
    // We call the action of creating a reference borrowing.
    let len = calculate_length(&s1);

    change(&mut s1);

    println!("The length of '{}' was {}", s1, len);
}

// Does not change the value of s
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Try to change the value of the reference some_string
//
// Just as variables are immutable by default, so are references. We're not
// allowed to modify something we have a reference to.
//
// In order to allow it, we must use a mutable reference.
//
// Mutable references have one big restriction: if you have a mutable reference
// to a value, you can have no other references to that value.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
