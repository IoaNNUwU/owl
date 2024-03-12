# Несколько мутабельных ссылок.

`Interior Mutability` - плохо, больше запутывает, чем вносит ясности.

Если величина изменяема, то на неё можно иметь несколько мутабельных ссылок `&mut`.

```rust

trait AllowMultipleMutableRefs;

impl Copy for &mut T 
where T: AllowMultipleMutableRefs

```

На практике:

```rust
// thread-safe контейнер, разрешает несколько
// мутабельных ссылок на себя.
struct ArcMutex<T> { .. }


// thread-safe контейнер, разрешает несколько
// немутабельных ссылок на себя.
struct Arc<T> { .. }
```

```rust
fn main() {

    let mutex = Mutex::new(120i32);


}
```