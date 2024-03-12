use std::fmt::Display;


pub struct SimpleMutex<T>(std::sync::Mutex<T>);

impl<T> SimpleMutex<T> {
    pub fn new(item: T) -> Self {
        Self(std::sync::Mutex::new(item))
    }

    pub fn with_value<F>(&self, action: F)
    where
        F: FnOnce(&mut T),
    {
        let mut guard = self.0.lock().expect("Unable to lock SimpleMutex");

        action(&mut guard);
    }
}

impl<T: Display> std::fmt::Display for SimpleMutex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.with_value(|value| write!(f, "SimpleMutex{{{}}}", value).unwrap());
        Ok(())
    }
}

pub struct ArcMutex<T>(std::sync::Arc<SimpleMutex<T>>);

impl <T> ArcMutex<T> {
    pub fn new(item: T) -> Self {
        ArcMutex(std::sync::Arc::new(SimpleMutex::new(item)))
    }

    pub fn with_value<F>(&self, action: F)
    where
        F: FnOnce(&mut T),
    {
        let mut guard = self.0.0.lock().expect("Unable to lock ArcMutex");

        action(&mut guard);
    }

    pub fn arc_clone(&self) -> ArcMutex<T> {
        ArcMutex(std::sync::Arc::clone(&self.0))
    }
}

impl<T: Display> std::fmt::Display for ArcMutex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.with_value(|value| write!(f, "SimpleMutex{{{}}}", value).unwrap());
        Ok(())
    }
}


