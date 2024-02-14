
```rust
use std::*

# crash_on(AllocationFailure)
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

# sneaky crash_on(AllocationFailure)
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
use std::*

# unsafe_never(IntegerOverflow)
fn func() -> i32 {
    return 5 + 3
}
```