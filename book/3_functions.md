
```rust
struct Bytes(private usize)
impl {

    // Takes itself by reference
    fn as(&self)<usize>() -> usize {
        self.0
    }

    // Consumes itself
    fn to(self)<usize>() -> usize {
        self.0
    }
}
```

```rust
let byte = Byte(10)

let size = byte as usize
let size = byte.as<usize>()

let size = byte to usize ???
let size = byte.to<usize>()
```

Expand on infix functions