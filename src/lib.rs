//! This is an experiment, and may or may not be fleshed out into a proper crate.

use anyhow::Result;
use async_trait::async_trait;

pub enum Computation<T> {
    Pure(T),
    Bind(Box<dyn Effect<Value = T>>),
}

#[async_trait]
pub trait Effect {
    type Value;
    async fn perform(&self) -> Self::Value;
}

#[async_trait]
pub trait Handler<T> {
    type Effect;
    async fn handle(&mut self, effect: Self::Effect) -> Computation<Result<T>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn basics() {
        struct Inc(u32);
        #[async_trait]
        impl Effect for Inc {
            type Value = Result<u32>;
            async fn perform(&self) -> Self::Value {
                Ok(self.0 + 1)
            }
        }

        struct IncHandler;
        #[async_trait]
        impl Handler<u32> for IncHandler {
            type Effect = Inc;
            async fn handle(&mut self, e: Self::Effect) -> Computation<Result<u32>> {
                match e.perform().await {
                    Ok(v) => Computation::Pure(Ok(v)),
                    Err(e) => Computation::Pure(Err(e)),
                }
            }
        }

        let effect = Inc(41);
        let mut handler = IncHandler;

        let foo = match handler.handle(effect).await {
            Computation::Pure(v) => v.unwrap(),
            Computation::Bind(e) => e.perform().await.unwrap(),
        };

        assert_eq!(foo, 42);
    }
}
