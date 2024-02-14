# Types are essential part of Owl programming language

Type inference is a problem in Rust.

```rust
fn main() {
    let string = r##"{ "n": "Hello" }"##

    let parsed_string = serde_json::parse(string).unwrap()

    // ...
    // let mut u = Vec::new::<User>()
    // u.push(parsed_string)
    // ...

    println!("{}", parsed_string)
}
```

- There is a lot of space between `serde_json::parse` call and usage of it's result.

Type of `parsed_string` determined between those points and it's often really hard to tell exact type of variable without visual search through all lines. In most cases you don't need to change the types of your variables very often, so adding more explicit types are usually a good thing.

```rust
fn main() {
    let string = r##"{ "n": "Hello" }"##

    let parsed_string: User = serde_json::parse(string).unwrap()

    // ...

    println!("{}", parsed_string)
}
```
- You can clearly see the type you're parsing, no need to read futher code to know, what we're printing.

Because types are a good thing, in your `Owl` source code you can put them everywhere to clarify your intentions.

```rust
fn main() {
    let string = r##"{ "n": "Hello" }"##: &str

    let parsed_string: User = (serde_json::parse(string): Result<User, *>).unwrap(): User 

    // ...

    println!("{}": &str, &parsed_string: &User): ()
    println!("{}", parsed_string)
}
```

```rust
fn main() {
    let string = r##"{ "n": "Hello" }"##: &str

    // no need for turbofish syntax
    let user = (serde_json::parse(string): Result<User, *>).unwrap()
    let user = (serde_json::parse(string)).unwrap(): User

    // recommended way is still this
    let user: User = (serde_json::parse(string)).unwrap()

    // with ?? syntax
    let user = serde_json::parse(string): Result<User, *>??
    let user = serde_json::parse(string)??: User

    // ...

    println!("{}", parsed_string): ()
}
```