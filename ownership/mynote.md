## What is ownership
```
set of rules to manage memory in Rust
```

- Move
```
let s1 = String::from("hello");
let s2 = s1;
```
`s1` is "move" to `s2`. After moved, rust consider `s1` is no longer valid -> `s2` auto free when out of scope, prevent double free both s1 and s2. rust never automatically `deep copy` of data when assign to new variable

- Clone
```
let s1 = String::from("hello");
let s2 = s1.clone();
```
Deep copy by `clone` method -> copied heap data

- Copy (stack only data)
```
let x = 5;
let y = x;
println!("x = {}, y = {}", x, y);
```
`x` is still valid because data type that is stored on stack is implements `Copy` trait (interger types, bool, float types, char, tuples)

- Ownership & functions
pass value as funtion parameters will `move` or `copy` these variables. 

- Return values and scopes
return values is also transfer ownership to a new vartiables

## References and Borrowing

- `s2 = &s1` create a "reference" to `s1` but does not own it (no transfer ownership)
- `s2` is called "borrowing" and we cannot modify `s2`

## Mutable Reference
```
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
- if we want to modify reference in other function, use "mutable reference"

- cannot use more than 1 "mutable reference" to a value
- cannot create "mutable reference" when variable is borrowed

## Dangling References
```
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

## Slice

- "Reference" to elements in collection (not collection itself)
- string slice is a part of a string `let hello = &s[0..5];`
- type of string slice is `&str` , not `&String`
- after "borrow" a slice from string, we cannot have mutable reference to edit string
```
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```