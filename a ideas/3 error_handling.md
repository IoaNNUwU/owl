
### Отсутствие внутренних паник - везде результат.

Даже для самых простых операций используется результат - это делается для избежания паник

Those are equivalent 
```rust
fn sum(a: i32, b: i32) -> Result<i32, IntegerOverflow> {
    return a + b // a + b can fail with IntegerOverflow
}

#[sneaky(result(IntegerOverflow))]
fn sum(a: i32, b: i32) -> Result<i32, ^> {
    return a + b
}

#[sneaky(easy(result(IntegerOverflow)))]
fn sum(a: i32, b: i32) -> i32 {
    return a + b
}
```

```rust
use std::*

# panic_on(AllocationFailure)
# result(UnableToBindTcpListener, UnableToGetTcpStream)
fn handle_tcp() -> Result<(), ^> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878")??

    mut vec: Vec<usize> = Vec()
    vec.push(1)!!

    for stream in listener.incoming() {
        let stream: TcpStream = (stream: Result<TcpStream, UnableToGetTcpStream>)??
    }
}
```

```rust
use std::*

# sneaky panic_on(AllocationFailure)
# sneaky result(UnableToBindTcpListener, UnableToGetTcpStream)
fn handle_tcp() -> Result<(), ^> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878")

    mut vec: Vec<usize> = Vec()
    vec.push(1)

    for stream in listener.incoming() {
        let stream: TcpStream = (stream: Result<TcpStream, UnableToGetTcpStream>)
    }
}
```

Those are equivalent 
```rust
fn sum(a: i32, b: i32) -> Result<i32, IntegerOverflow> {
    return a + b
}

#[sneaky(result(IntegerOverflow))]
fn sum(a: i32, b: i32) -> i32 {
    return a + b
}
```