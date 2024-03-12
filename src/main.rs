use owl::*;

fn main() {
    let am = SimpleArc::new(SimpleMutex::new(String::from("Hello world!")));

    let mut handles = vec![];

    for i in 0..10 {
        let mut arc_mutex = SimpleArc::clone(&am);
        let handle = std::thread::spawn(move || arc_mutex.with_mut_value(|text| text.push_str(&format!("{}", i))));

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("sm: {}", am);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let am = SimpleMutex::new(1);
        
        let static_mutex_ref = Mut::new(Box::leak(Box::new(am)));

        let mutable_ref1 = static_mutex_ref.clone();
        let mutable_ref2 = static_mutex_ref.clone();

        println!("1: {}", mutable_ref1);
        println!("2: {}", mutable_ref2);
        println!("0: {}", static_mutex_ref);

        // We should be able to safely have multiple mutable references to SimpleMutex.

    }
}
