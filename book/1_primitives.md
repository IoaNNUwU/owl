
### Primitives

```rust
i8, i16, i32, i64, i128
u8, u16, u32, u64, u128

f32, f64

bool, bit

char

bit // bit as primitive type
```

### Efficient size
`i32` uses all its space to store data.

`bool` uses only 1 bit of it's `8-bit` implementation, so it's efficient size is 1 bit.

Efficient size could be used as memory layout optimization in bitfields, arrays, unions.

Inspired from `Zig` language

```rust
fn size_of::<T>() -> usize // bytes (u8)

// Add struct name as generic argument to allow
// functions to share name but being different
fn size_of::<bit, T>() -> usize { size_of::<T>() / 8 } // bits

fn eff_size_of::<T>() -> usize // bits

fn eff_size_of::<bit, T>() -> usize { eff_size_of::<T>() } // bits

// can be used in effective data structures
struct EnumVec<T: Enum> {
    // Tag and Data for enum can be calculated using eff_size_of
    tags: Vec<Tag<T>>,
    data: Vec<Data<T>>,
}

let size = size_of::<i64>()
assert_eq!(size, 8)

let size_bits = size_of::<bit, i64>()
assert_eq!(size_bits, 8 * 8)

let eff_size_bits = eff_size_of::<i64>()
assert_eq!(size, 8 * 8)

let size = size_of::<Option<i64>>()
assert_eq!(size, 2 * 8) // For alignment reasons it doubles

let size_bits = size_of::<bit, Option<i64>>()
assert_eq!(size_bits, 2 * 8 * 8)

let eff_size_bits = eff_size_of::<Option<i64>>()
assert_eq!(size, 65)
```

### bit as primitive

```rust
let size_bytes = size_of::<bit>()
assert_eq!(size_bytes, 1)

// Note: first bit symbol means size should be in bits
let size_bits = size_of::<bit, bit>()
assert_eq!(size_bits, 8)

let eff_size_bits = eff_size_of::<bit>()
assert_eq!(eff_size_bits, 1)
```

Including single bit representation allows for easy bit manupulations.
Best way to think about bit as u8 with eff_size = 1 and additional methods.
```rust
# allow_byte_indexing

let n: u8 = 0x10000000
let first_bit: bit = n[0]
assert_eq!(first_bit, 1)
```

bit is very simillar to `bool`.

```rust
let size_bytes = size_of::<bool>()
assert_eq!(size_bytes, 1)

let size_bits = size_of::<bit, bool>()
assert_eq!(size_bits, 8)

let eff_size_bits = eff_size_of::<bool>()
assert_eq!(eff_size_bits, 1)
```

### Additional efficient types
`i63` uses 63 bits of it's `64-bit` implementation. `1` bit could be used as some kind of marker.

`Option<i63>` could use `64-bits` (the same as `i64`)

```rust
let size_bytes = size_of::<i63>()
assert_eq!(size_bytes, 8)

let size_bits = size_of::<bit, i63>()
assert_eq!(size_bits, 8 * 8)

let eff_size_bits = eff_size_of::<i63>()
assert_eq!(eff_size_bits, 63)

let op_size_bytes = size_of::<Option<i63>>()
assert_eq!(op_size_bytes, 8)

let op_size_bits = size_of::<bit, Option<i63>>()
assert_eq!(op_size_bytes, 8 * 8)

let op_eff_size_bits = eff_size_of::<Option<i63>>()
assert_eq!(op_eff_size_bytes, 64)
```
- is `i63` appropriate name?
