
# Отсутствие внутренних паник - везде результат.

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

```rust
# unsafe_never(IntegerOverflow)
fn func() -> i32 {
    return 5 + 3
}

fn func() -> Result<i32, IntegerOverflow> {
    return 5 + 3
}
```