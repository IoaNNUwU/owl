# O v O (Owl-lang)
My thoughts on ideal programming language that someday I'll write.

- Inspired by `Rust`, `Kotlin`

## Types
### Syntax choises
'`,`' '`;`' are bad because they make hard to copy & format code, it is also easy to forget to put them. They are used in as few places as possible.

General rule: `,` used to split expressions in single line

'`{ }`' is good because they allow to easily determine neccessary indentation and easily determine scopes of values.

- should be putting ',' after line even possible?
- do whitespaces feel right?
- how many whitespaces do we need to visually distinct fields on same line from one another (current: `2`)?

## Functions
```kotlin
// Main function is an entry point
fun main() {
    val n = 5
    // cannot be reassigned
    // n = 10

    mut a = 3
    a = 10

    println(n)
    println(a)
}

fun add ( a: i32  b: i32 ) -> i32 = a + b

```
Some readability tricks
```kotlin
// Return type can be put inside parenthesis
fun add ( a: i32  b: i32  -> i32 ) {
    a + b
}
// Useful in multiline functions
// This is the preferred way to write multiline function arguments.
fun add ( -> i32
    a: i32
    b: i32
) { 
    a + b
}
```
This is how add function looks with return type put after arguments. 
It is hard to visually locate it especially when generics are used.
So I think it is better to have return type right after name of function.
```kotlin
// NOT preferred way to write multiline functions
fun add ( 
    a: i32
    b: i32
) -> i32 { 
    a + b
}
```
#### Function call
```kotlin
fun main() {
    // no need for comas between arguments
    val answer = add( 2  3 )
    println(answer)
}
```
Somewhat full program
```kotlin
struct User {
    name: String
}

enum Ordering { Greater  Equal  Less }

fun compare_users ( user1: User  user2: User ) -> Ordering {
    user1.name.compare(user2.name)
}

fun main() {
    compare_users( User { name: "User1" }  User { name: "User2" })
    compare_users(
        User { name: "User1" }
        User { name: "User2" }
    )

    val user1 = User { name: "User1" }
    val user2 = User { name: "User2" }

    compare_users( user1  user2 )
}
```
#### Inside operators
- Boolean NOT (`!`) should be placed right before function call
```Rust
let authorized = true
if !cond {
    println!("No access")
    return
}

let user = User::default()
if user.is_authorized() {
    println("Welcome")
}
// Note: ! before boolean function acts as NOT
if user.!is_admin() {
    println("Maintanence! Try next time")
    return
}
```
- Reference op (`&`) should be placed before the thing
```Rust
let user = User::default();
let name = user.&name;
// Same with dereference
if user.*password == 1234567 {
    println("Too easy password")
    return
}
```

### Modules and visibility
Project structure

```
. ~/rust/myproject
│
├── src
│   ├── amazing_library.ovo
│   └── my_library_module
│       ├── my_library_module.ovo
│       └── additional_module.ovo
│
├── target
├── out
│
├── OvO.lock
└── OvO.toml
```
```toml
# OvO.toml
[lib]
name = "amazing_library"
version = 0.0.1
```
```rust
// amazing_library.ovo
mod my_library_module

// my_library_module.ovo
mod additional_module
```
Everything is visible to users of the library by default.

`visibility` can be changed using `vis` keyword.

- `vis(none)`. Visible nowhere. Even in structure's own implementation blocks. Handy for markers, padding etc. Since it's impossible to access such field does not disable ability to construct structure.
- `vis(private)`. Only visible in structure's own implementation blocks.
- `vis(file)`. Only visible in file, where structure defined.
- `vis(mod)`. Only visible in module, where structure defined.

### for/while loop

for/while loops create a lot of indentation.

There are posibilities:
1. Everything after `loop-variable` declarations is placed inside the loop (Break using `break 'label` syntax)
2. Only things that use `loop-variable` are placed inside the loop (This is stupid because you can mix `loop` & `non-loop` things but may be cool)
3. Loop ends with last usage of `loop-variable` (This is stupid because there is no visible difference between loop and non-loop)
4. Indentations are lowered using `fn name() = for {}` syntax.
5. Use various indentations for different things. For `match` it can be 2 spaces
```rust
// 1
fn add_connect_and_disconnect_message_to_queue_on_server_event(
    mut server_events: EventReader<ServerEvent>,
    mut c2s_queue: ResMut<IncomingC2SPacketsQueue>,
) {
    let event = 'label for server_events.iter();
    match event {
        ServerEvent::ClientConnected { client_id } => {
            let message = Client2ServerMessage {
                client_id: *client_id,
                packet: Client2ServerPacket::Ping,
            };
            c2s_queue.0.push(message);
        }
        ServerEvent::ClientDisconnected { client_id, reason } => {
            let message = Client2ServerMessage {
                client_id: *client_id,
                packet: Client2ServerPacket::Disconnect(C2SDisconnect {
                    reason: reason.to_string(),
                }),
            };
            c2s_queue.0.push(message);
        }
    }
    break 'label
    println("Loop ends");
}
// 4
fn add_connect_and_disconnect_message_to_queue_on_server_event(
    mut server_events: EventReader<ServerEvent>,
    mut c2s_queue: ResMut<IncomingC2SPacketsQueue>,
)
= for event in server_events.iter {
    match event {
        ServerEvent::ClientConnected { client_id } => {
            let message = Client2ServerMessage {
                client_id: *client_id,
                packet: Client2ServerPacket::Ping,
            };
            c2s_queue.0.push(message);
        }
        ServerEvent::ClientDisconnected { client_id, reason } => {
            let message = Client2ServerMessage {
                client_id: *client_id,
                packet: Client2ServerPacket::Disconnect(C2SDisconnect {
                    reason: reason.to_string(),
                }),
            };
            c2s_queue.0.push(message);
        }
    }
}
// no way to print something after loop ends
// 4+
fn add_connect_and_disconnect_message_to_queue_on_server_event(
    mut server_events: EventReader<ServerEvent>,
    mut c2s_queue: ResMut<IncomingC2SPacketsQueue>,
)
= for event in server_events.iter match event {
    ServerEvent::ClientConnected { client_id } => {
        let message = Client2ServerMessage {
            client_id: *client_id,
            packet: Client2ServerPacket::Ping,
        };
        c2s_queue.0.push(message);
    }
    ServerEvent::ClientDisconnected { client_id, reason } => {
        let message = Client2ServerMessage {
            client_id: *client_id,
            packet: Client2ServerPacket::Disconnect(C2SDisconnect {
                reason: reason.to_string(),
            }),
        };
        c2s_queue.0.push(message);
    }
}
// 5
fn add_connect_and_disconnect_message_to_queue_on_server_event(
    mut server_events: EventReader<ServerEvent>,
    mut c2s_queue: ResMut<IncomingC2SPacketsQueue>,
) {
    for event in server_events.iter() {
        match event {
          ServerEvent::ClientConnected { client_id } => {
                let message = Client2ServerMessage {
                    client_id: *client_id,
                    packet: Client2ServerPacket::Ping,
                };
                c2s_queue.0.push(message);
            }
          ServerEvent::ClientDisconnected { client_id, reason } => {
            let message = Client2ServerMessage {
                client_id: *client_id,
                packet: Client2ServerPacket::Disconnect(C2SDisconnect {
                    reason: reason.to_string(),
                }),
            };
            c2s_queue.0.push(message);
            }
        }
    }
    println("Loop ends");
}
// Connect unconnectable (Stupid because it is hard to see match)
fn add_connect_and_disconnect_message_to_queue_on_server_event(
    mut server_events: EventReader<ServerEvent>,
    mut c2s_queue: ResMut<IncomingC2SPacketsQueue>,
) {
    for match event in server_events.iter() {
        ServerEvent::ClientConnected { client_id } => {
            let message = Client2ServerMessage {
                client_id: *client_id,
                packet: Client2ServerPacket::Ping,
            };
            c2s_queue.0.push(message);
        }
        ServerEvent::ClientDisconnected { client_id, reason } => {
            let message = Client2ServerMessage {
                client_id: *client_id,
                packet: Client2ServerPacket::Disconnect(C2SDisconnect {
                    reason: reason.to_string(),
                }),
            };
            c2s_queue.0.push(message);
        }
    }
    println("Loop ends");
}
// Functional
fn add_connect_and_disconnect_message_to_queue_on_server_event(
    mut server_events: EventReader<ServerEvent>,
    mut c2s_queue: ResMut<IncomingC2SPacketsQueue>,
) {
    server_events.iter().map(|event| => { match event {
        ServerEvent::ClientConnected { client_id } => Client2ServerMessage::new(*client_id, Client2ServerPacket::Ping)
        ServerEvent::ClientDisconnected { client_id, reason } =>
            Client2ServerMessage::new(*client_id, Client2ServerPacket::Disconnect(C2SDisconnect::new(reason.to_string()))),
    }})
    .for_each(|message| c2s_queue.0.push(message));
}
// Special map operator
fn add_connect_and_disconnect_message_to_queue_on_server_event(
    mut server_events: EventReader<ServerEvent>,
    mut c2s_queue: ResMut<IncomingC2SPacketsQueue>,
)
  = server_events
    |> |event| => match event {
    |       ServerEvent::ClientConnected { client_id } => Client2ServerMessage::new(*client_id, Client2ServerPacket::Ping)
    |       ServerEvent::ClientDisconnected { client_id, reason } => {
    |           Client2ServerMessage::new(*client_id, Client2ServerPacket::Disconnect(C2SDisconnect::new(reason.to_string()))),
    |       }
    |   }
    *> |message| c2s_queue.0.push(message)
}
```

### Functional
- `|>` 
- `*` immideately executes operations. If there is final function (Like `count()`) it should be placed right after.
- `**` immideately executes `.collect()` with specified buffer
```Rust
let mut strings: Vec<String> = vec!["BigLong String", "hel lo", "worl d"]
    .into_iter()
    .map(String::from)
    .collect();

// Rust
let size = strings.iter_mut()
    .filter(|s| s.len() > 3)
    .filter(|s| s.len() < 8)
    .filter(|s| !s.is_empty())
    .map(|s| s.replace(' ', "@"))
    .map(|s| s.chars()
        .rev()
        .filter(|_| rand::gen_bool(0.9))
        .collect::<Vec<_>>())
    .count();

// Functional
let size = strings.iter_mut()
    |#  len() > 3
    |#  len() < 8
    |#  !is_empty()
    |>  replace(' ', "@")
    |>  chars()
    |   |>  rev()
    |   |#  rand::gen_bool(0.9)
    |   .collect()
    |
    .count()

// Functional 2
let size = strings.iter_mut()
            |#  len() is in 3..8
            .   Vec<_>// Execute action immediately and store result in vec
            |#  !is_empty())
            .count()

let size = strings.iter_mut()
    |#  s -> "Hello World".len() is in s.len()..8
    .count()

```

### Uses `.ovo` extension:
`main.ovo` (Looks like owl !)
- possible `.owl` (Is owl !)
- possible `.owo` (Looks like something cute)
- possible `.uwu` (why not?)


