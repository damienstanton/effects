#![allow(dead_code)]

pub trait Effect<T> {
    fn perform(&self) -> T;
}

pub trait Handler<E: Effect<T>, T> {
    fn handle(&mut self, effect: E) -> Computation<T>;
}

pub enum Computation<T> {
    Pure(T),
    Effect(Box<dyn Effect<T> + Send + Sync>),
}

struct MyEffect1;

impl Effect<i32> for MyEffect1 {
    fn perform(&self) -> i32 {
        42
    }
}

struct MyEffect2;

impl Effect<String> for MyEffect2 {
    fn perform(&self) -> String {
        "hello".to_string()
    }
}

struct MyHandler1;
impl Handler<MyEffect1, i32> for MyHandler1 {
    fn handle(&mut self, effect: MyEffect1) -> Computation<i32> {
        let n = effect.perform();
        Computation::Pure(n + 81)
    }
}

struct MyHandler2;
impl Handler<MyEffect2, String> for MyHandler2 {
    fn handle(&mut self, effect: MyEffect2) -> Computation<String> {
        _ = effect;
        // ...
        Computation::Pure("world".to_string())
    }
}

pub fn run_effect<E, H, T>(effect: E, handler: &mut H) -> T
where
    E: Effect<T> + Send,
    H: Handler<E, T> + Handler<T, T> + Send,
    T: Send + Sync + Effect<T>,
{
    match handler.handle(effect) {
        Computation::Pure(v) => v,
        Computation::Effect(e) => run_effect(*Box::new(e.perform()), handler),
    }
}

#[test]
fn test_my_handler1() {
    let mut handler = MyHandler1;
    let effect = MyEffect1;

    let a = Computation::Pure(123);
    let b = handler.handle(effect);

    let x = if let Computation::Pure(v) = a { v } else { 0 };
    let y = if let Computation::Pure(v) = b { v } else { 0 };
    assert_eq!(x, y);
}

#[test]
fn test_my_handler2() {
    let mut handler = MyHandler2;
    let effect = MyEffect2;

    let a = Computation::Pure(String::from("hello"));
    let b = handler.handle(effect);

    let x = if let Computation::Pure(v) = a {
        v
    } else {
        String::default()
    };
    let y = if let Computation::Pure(v) = b {
        v
    } else {
        String::default()
    };
    assert_eq!(format!("{} {}", x, &y), String::from("hello world"))
}
