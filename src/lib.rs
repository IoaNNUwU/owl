use std::fmt::Display;

/// All code in this file in inherently unsafe, barely works and doesn't 
/// follow any Rust rules. It relies and makes heavy use of undefuned
/// behaviour.
/// 
/// It exists just to illustrate an example and
/// does not intended to provide any useful features.
///
/// Let's imagine the world where multiple mutable references
/// are allowed to some values - this can help to remove the need for
/// interior mutability and simplify some APIs.
pub unsafe trait AllowMultipleMutRef {}

pub struct SimpleMutex<T>(std::sync::Mutex<T>);

impl<T> SimpleMutex<T> {
    pub fn new(item: T) -> Self {
        Self(std::sync::Mutex::new(item))
    }

    /// NOTE: std::sync::Mutex allows immutable reference use
    /// in this function and allows us to change the value inside
    /// the mutex using concept of `interior mutability`. Using mutable
    /// reference here is an API choise and intended to clearly tell the 
    /// user that value can be changed.
    pub fn with_mut_value<F>(&mut self, func: F)
    where
        F: FnOnce(&mut T),
    {
        let mut guard = self.0.lock().expect("Unable to lock SimpleMutex");

        func(&mut guard);
    }

    /// NOTE: Even if std::sync::Mutex allows mutable actions using immutable
    /// reference, it is the API choise to allow only immutable access through
    /// immutable references.
    pub fn with_value<F>(&self, func: F)
    where
        F: FnOnce(&T),
    {
        let mut guard = self.0.lock().expect("Unable to lock SimpleMutex");

        func(&mut guard);
    }
}

impl<T: std::fmt::Display> std::fmt::Display for SimpleMutex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.with_value(|value| write!(f, "SimpleMutex {{ {} }}", value).unwrap());
        Ok(())
    }
}

/// SAFETY: Even if you have multiple mutable references to
/// SimpleMutex, you cannot modify it in a way that is not thread-safe.
unsafe impl<T> AllowMultipleMutRef for SimpleMutex<T> {}

pub struct SimpleArc<T>(std::sync::Arc<T>);

impl<T> SimpleArc<T> {

    pub fn new(t: T) -> Self {
        Self(std::sync::Arc::new(t))
    }
}

impl<T> std::ops::Deref for SimpleArc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(mutable_transmutes)]
impl<T: AllowMultipleMutRef> std::ops::DerefMut for SimpleArc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self.0.as_ref()) }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for SimpleArc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SimpleArc {{ {} }}", self.0.as_ref())
    }
}

impl<T> Clone for SimpleArc<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct Mut<'a, T>(&'a mut T);

impl <'a, T> Mut<'a, T> {

    pub fn new(r: &'a mut T) -> Self {
        Self(r)
    }
}

impl<'a, T> std::ops::Deref for Mut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, T> std::ops::DerefMut for Mut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

#[allow(mutable_transmutes)]
impl<'a, T: AllowMultipleMutRef> Clone for Mut<'a, T> {
    fn clone(&self) -> Self {

        let another_mut_ref: &mut T = unsafe { std::mem::transmute(&self.0) };
        Self(another_mut_ref)
    }
}

impl<'a, T: Display> std::fmt::Display for Mut<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mut {{ {} }}", self.0)
    }
}
