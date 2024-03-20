#

Lets have a user

```rust
let u = User { id: 5 };
```

Lets have a reference to the user

```rust
let u_ref = &i;
println!("{i_ref}");
```

Lets have a mutable reference to the user

- By default mutable access is only allowed through mutable reference. This tells the user it is the only way, which is not true considering Mutex, RefCell etc.
```rust

let u_mut_ref = &mut u;
u_mut_ref.id += 1;
println!("{u_mut_ref}");
```

Lets have a multithreaded scenario

```rust
let u: &'static Mutex<User> = Box::leak(Mutex::new(User { id: 6}));
```
This code is fine.
```rust
for i in 0..10 {
    std::thread::spawn(move || {
        println!("{u}");
    });
}
```

This should not work because u is immutable reference but we actually can mutate the value behind immutable reference. 

This is called Interior Mutability.
```rust
for i in 0..10 {
    std::thread::spawn(move || {
        u.lock().unwrap().id += 1;
    });
}
```
**Interior mutability is a joke and breaks all mutability rules. There are 2 ways to fix it.**

### 1. Allow multiple mutable references for specific structures.

This will ensure that `mut` semantics are valid even for **non-trivial** structures, such as **Smart Pointers**, **MultiThreaded data structures**, etc.

```rust
let u: &mut 'static Mutex<User> = Box::leak(Mutex::new(User { id: 6}));

for i in 0..10 {
    // &mut for Mutex is Clone so we are able to move reference in closure
    std::thread::spawn(move || {
        u.lock().unwrap().id += 1;
    });
}
// impls look something like this
unsafe trait AllowMultipleMutableRefs {}

unsafe impl AllowMultipleMutableRefs for Mutex<T> {}

impl<T: AllowMultipleMutableRefs> Clone for &mut T {}
```

### 2. Remove the concept of mutability in references and change it to uniqueness.

This will invert the way of thinking and remove this contradiction. You can change the value **because** reference is unique and not the other way - *you can change the value, **which means** reference should be unique.*

```rust
let u: &'static Mutex<User> = Box::leak(Mutex::new(User { id: 6}));

for i in 0..10 {
    std::thread::spawn(move || {
        u.lock().unwrap().id += 1;
    });
}
// impls look something like this
unsafe trait AllowMutableAccessThroughSharedRef {}

unsafe impl AllowMutableAccessThroughSharedRef for Mutex<T> {}
```
And in single-threaded scenario.

- Compiler checks if it is unique. By default mutable access is allowed only using &unique reference but nothing tells user it is the only way, so no contradiction here.
 
```rust
let u_ref = &i;
println!("{u_ref}");

let u_mut_ref = &unique u;
u_mut_ref.id += 1;
println!("{u_mut_ref}");
```
First way is better, because it allows us to nicely integrate in existing mut semantics in variable declarations.

Traits `AllowMutableAccessThroughSharedRef` & `AllowMultipleMutableRefs` are unsafe marker traits (only 1 trait exists in each scenario, we dont't need both) and are very simillar to `Send` & `Sync`.

trait `Sync` allows multiple mutable references to be shared across threads. May be there should be some rules and traits for the situation where there are multiple mutable references present (ex. 1).

May be `Sync` should be generic over type of the reference and allow multiple mutable references if type implements `AllowMultipleMutableRefs`, or there should be special different trait like `SyncMut` to allow types that implement `AllowMultipleMutableRefs` to share mutable reference between threads.

### Additional `unique` usages

There are a lot of additional usages for `unqiue` keyword:

#### Generics

There are associated types in Rust traits.
Purpose of them is to have single generic implementation for a trait.

For example:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

When we implement `Iterator` for type, this type can only yield items of **single** type, that is `Self::Item`. We cannot implement iterator that yields `i32` and `i16` depending on your needs.

Second kind of generics is confusing. We can fix it by using `unique` keyword:
```rust
trait Iterator<unique I> {
    fn next(&mut self) -> Option<I>;
}
```

So it's obvious there can only be single `I` per implementation.