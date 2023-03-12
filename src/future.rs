use async_recursion::async_recursion;
use async_trait::async_trait;

#[async_trait]
pub trait Effect: Send + Sync {
    type Value;
    async fn perform(&self) -> Self::Value;
}
pub enum Computation<T> {
    Pure(T),
    Effect(Box<dyn Effect<Value = T>>),
}

#[async_trait]
pub trait Handler<T>: Send + Sync {
    type Effect: Effect<Value = T>;
    async fn handle(&mut self, effect: Self::Effect) -> Computation<T>;
}

#[async_recursion]
pub async fn run_effect<H, T>(effect: <H as Handler<T>>::Effect, handler: &mut H) -> T
where
    H: Handler<T, Effect = T>,
    T: Send + Sync + Effect<Value = T>,
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
    impl Effect for MyEffect1 {
        type Value = i32;
        async fn perform(&self) -> i32 {
            42
        }
    }

    struct MyEffect2;

    #[async_trait]
    impl Effect for MyEffect2 {
        type Value = String;
        async fn perform(&self) -> String {
            "hello".to_string()
        }
    }

    struct MyHandler1;
    #[async_trait]
    impl Handler<i32> for MyHandler1 {
        type Effect = MyEffect1;
        async fn handle(&mut self, effect: MyEffect1) -> Computation<i32> {
            let n = effect.perform().await;
            Computation::Pure(n + 81)
        }
    }

    struct MyHandler2;
    #[async_trait]
    impl Handler<String> for MyHandler2 {
        type Effect = MyEffect2;
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
