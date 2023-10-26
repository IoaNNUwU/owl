#[cfg(test)]
mod tests;

fn main() {
    let mut strings: Vec<String> = vec![
        "BigLong String",
        "hel lo",
        "worl d",
        "123 ",
        "2",
        "Str",
        "Whois",
        "  a     ",
    ]
    .into_iter()
    .map(String::from)
    .collect();

    let vec: Vec<_> = strings
        .iter_mut()
        .filter(|s| s.len() > 3)
        .filter(|s| s.len() < 8)
        .filter(|s| !s.is_empty())
        .map(|s| s.replace(' ', "@"))
        .map(|s| {
            s.chars()
                .rev()
                .filter(|_| rand::gen_bool(0.9))
                .collect::<Vec<_>>()
        })
        .collect();
}

mod rand {
    pub fn gen_bool(chance_true: f32) -> bool {
        true
    }
}
