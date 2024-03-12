### Arrays

```rust
// array with 8 elements of type f32
let nums: [8]f32 = [1, 2, 3, 4, 5, 6, 7, 8]f32
let nums = [1, 2, 3, 4, 5, 6, 7, 8]f32
let nums = [1: f32, 2, 3, 4, 5, 6, 7, 8]f32

// preferred way to initialize arrays
let nums: [8]f32 = [1, 2, 3, 4, 5, 6, 7, 8]
let nums = [8]f32::from_fn(|i| i + 1)
let nums: [8]f32 = [*]::from_fn(|i| i + 1)

// type should be defined on initialization
// so this is not allowed even if type will be defined later
let nums = [1, 2, 3, 4, 5, 6, 7, 8] // doesn't compile
```

### Structs

```rust
struct User {
    name: String,
    age: i32,
}
```

### Union
Tagged union is a type that can represent multiple types by assigning each type a tag. Tag is a number that will take additional space but will allow to know which exact variant this union holds.

Union can only store existing types, but allows anonymous types as part of its declaration.

```rust
union Int { i8, i16, i32, i64, i128 }

let num: Int = Int::i8(15)

// You can match over types of union like this
// those match & if expressions are equivalent
match num {
    i8 -> println("number is i8 = $num"),
    else -> {}
}
if num is i8 { 
    println("number is i8 = $num")
}
```

- Because each variant of `Int` is existing type, we don't need new variable in match statement, but we can add it to match if we need

```rust
match num {
    i32 -> println("num is i32: {num}"),
    i64 -> println("num is i64: {num}"),
    i8 pattern_num -> println("number is i8 = {pattern_num}"),
    i16 n16 -> println("number is i16 = {n16}"),
    else -> {},
}
```

Unions can have anonymous types as part of them

```rust
union Message {
    String,
    i32,
    struct Quit,
    struct Move { x: i32  y: i32 },
}

let msg = Message::String(String::new("Hello World"))
let msg = Message::i32(18)

let msg = Message::Quit
let msg = Message::Move { x: 18, y: 14 }

match msg {
    String -> println("Message: {msg}"),
    i32 -> println("Number: {msg}"),
    Message::Quit -> println("Quit"),
    Message::Move { x, y } -> println("Move: ({x}, {y})"),
}
```

### Enums
Enums are unions where every variant is new type.

Enums are short form of union declaration where every type is anonymous.
```rust
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

// This message are equivalent to Message from union example.
// Difference is you cannot match on String or i32 directly
// you have to create a new type and match on it
enum Message {
    String(String),
    Number(i32),
    Quit,
    Move { x: i32  y: i32 },
}

let msg = Message::String(String::new("Hello World"))

match msg {
    Message::Number(num) -> println("Number {num}"),
    Message::String(text) -> println("Text {text}"),
    Message::Quit -> println("Quit"),
    Message::Move { x, y } -> println("Move: ({x}, {y})"),
}
```

### Ease of use

Some enums have special abilities

```rust
fn main() {
    let o: Option<i32> = Some(10)
    match o {
        Some n => println!("Some {n}"),
        None => println!("None"),
    }
}
```

### Enum vs Union

Enums are short form for union where every type is new type.

This is the case almost all the time, so you should probably use enums all the time.

But if you have excessive amount of existing types in your enum consider using union instead.

### Untagged union
c-union that doesn't add tag on top of underlaying data.

can be used to easily convert between 2 types.

If 2 types have different layout in memory it is not safe to use c-union.

```rust
c_union { i32  u32 }
```