### Arrays

```rust
// array with 8 elements of type i32
let nums: [8]f32 = [1, 2, 3, 4, 5, 6, 7, 8]f32
let nums = [1, 2, 3, 4, 5, 6, 7, 8]f32

// preffered way to initialize arrays
let nums: [8]f32 = [1, 2, 3, 4, 5, 6, 7, 8]

// type should be defined on initialization
// so this is not allowed even if type will be defined later
let nums = [1, 2, 3, 4, 5, 6, 7, 8] // doesn't compile
```

Efficient arrays

```rust

// [8]bool
let size: bits = size_of::<bit, [8]bool>()
assert_eq!(size_bits, 8 * 8)

let eff_size: bits = eff_size_of::<[8]bool>()
assert_eq!(eff_size, 8) // only 8 of 64 bits are efficently occupied

let size: bits = size_of::<compact[8]bool>()
assert_eq!(size, 8) // 8 bools occupy only 1 byte (8 bit)

let eff_size: bits = eff_size_of::<compact[8]bool>()
assert_eq!(eff_size, 8) // 8 bools occupy only 1 byte (8 bit)
```


### Structs

```rust
struct User {
    name: String,
    age: i32,
}
// to visually distinct fields from one another we use double whitespaces between
struct Pos { x: i32  y: i32 }
```

### Union
Tagged union is a type that can represent multiple types by assigning each type a tag. Tag is a number that will take additional space but will allow to know which exact variant this union holds.

Union can only store existing types, but allows anonymous types as part of its declaration.

```rust
union Int { i8  i16  i32  i64  i128 }

let num = Int::i8(15)
// You can match over types of union like this
// those match & if expressions are equivalent
match num {
    Int::i8(n) -> println("number is $n")
    else -> {}
}
if num is Int::i8(n) {
    println("number is $n")
}
```
```rust
// Unions can have anonymous types as part of them
union Message {
    String
    i32
    struct Quit
    struct Move { x: i32  y: i32 }
}

val msg = Message::i32(15)

match msg {
    Message::String(text) -> print("Message: $text")
    Message::i32(num) -> print("Number: $num")
    else -> {}
}
```

### Enums
Enums are unions where every variant is new type.

Enums are short form of union declaration where every type is anonymous.
```rust
enum Day {
    Monday
    Tuesday
    Wednesday
    Thursday
    Friday
    Saturday
    Sunday
}

// This message are equivalent to Message from union example.
// Difference is you cannot match on String or i32 directly
// you have to create a new type and match on it
enum Message {
    Text(String)
    Number(i32)
    Quit
    Move { x: i32  y: i32 }
}

val msg = Message::Text(String("Hello World"))

match msg {
    Message::Number(num) -> println("Number $num")
    Message::Text(text) -> println("Text $text")
    else -> {}
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
c-union { i32  u32 }
```

### Tuples & Arrays

```kotlin
val tuple: (f32, u16, u8) = (1, 1, 1)
val array: [3]f32 = [1, 1, 1]
val slice: []f32 = [1, 1, 1, 1]
```
