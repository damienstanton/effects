#![allow(dead_code)]
use async_recursion::async_recursion;
use async_trait::async_trait;

#[async_trait]
pub trait Effect<A> {
    async fn perform(&self) -> A;
}
pub enum Computation<A> {
    Pure(A),
    Effect(Box<dyn Effect<A> + Send + Sync>),
}

#[async_trait]
pub trait Handler<E: Effect<A>, A> {
    async fn handle(&mut self, effect: E) -> Computation<A>;
}

#[async_recursion]
pub async fn run_effect<E, H, A>(effect: E, handler: &mut H) -> A
where
    E: Effect<A> + Send,
    H: Handler<E, A> + Handler<A, A> + Send,
    A: Send + Sync + Effect<A>,
{
    match handler.handle(effect).await {
        Computation::Pure(v) => v,
        Computation::Effect(e) => run_effect(*Box::new(e.perform().await), handler).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MyEffect1;

    #[async_trait]
    impl Effect<i32> for MyEffect1 {
        async fn perform(&self) -> i32 {
            42
        }
    }

    struct MyEffect2;

    #[async_trait]
    impl Effect<String> for MyEffect2 {
        async fn perform(&self) -> String {
            "hello".to_string()
        }
    }

    struct MyHandler1;
    #[async_trait]
    impl Handler<MyEffect1, i32> for MyHandler1 {
        async fn handle(&mut self, effect: MyEffect1) -> Computation<i32> {
            let n = effect.perform().await;
            Computation::Pure(n + 81)
        }
    }

    struct MyHandler2;
    #[async_trait]
    impl Handler<MyEffect2, String> for MyHandler2 {
        async fn handle(&mut self, effect: MyEffect2) -> Computation<String> {
            _ = effect;
            // ...
            Computation::Pure("world".to_string())
        }
    }

    #[tokio::test]
    async fn test_my_handler_async1() {
        let mut handler = MyHandler1;
        let effect = MyEffect1;

        let a = Computation::Pure(123);
        let b = handler.handle(effect).await;

        let x = if let Computation::Pure(v) = a { v } else { 0 };
        let y = if let Computation::Pure(v) = b { v } else { 0 };
        assert_eq!(x, y);
    }

    #[tokio::test]
    async fn test_my_handler_async2() {
        let mut handler = MyHandler2;
        let effect = MyEffect2;

        let a = Computation::Pure(String::from("hello"));
        let b = handler.handle(effect).await;

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
}
