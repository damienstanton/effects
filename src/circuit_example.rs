// use crate::{run_effect, Computation, Effect, Handler};
// use async_trait::async_trait;

// #[derive(Debug, PartialEq)]
// pub enum Voltage {
//     Low,
//     High,
// }

// #[derive(Debug, PartialEq)]
// pub enum CircuitEffect {
//     ReadVoltage,
//     ToggleSwitch,
// }

// pub struct Circuit {
//     switch_on: bool,
//     voltage: Voltage,
// }

// impl Circuit {
//     pub fn new() -> Self {
//         Circuit {
//             switch_on: false,
//             voltage: Voltage::Low,
//         }
//     }
// }

// #[async_trait]
// impl Effect for CircuitEffect {
//     type Value = Voltage;

//     async fn perform(&self) -> Voltage {
//         match self {
//             CircuitEffect::ReadVoltage => {
//                 let circuit = Circuit::new();
//                 if circuit.switch_on {
//                     Voltage::High
//                 } else {
//                     Voltage::Low
//                 }
//             }
//             CircuitEffect::ToggleSwitch => {
//                 let mut circuit = Circuit::new();
//                 circuit.switch_on = !circuit.switch_on;
//                 circuit.voltage = if circuit.switch_on {
//                     Voltage::High
//                 } else {
//                     Voltage::Low
//                 };
//                 circuit.voltage
//             }
//         }
//     }
// }

// struct CircuitHandler;
// #[async_trait]
// impl Handler<Voltage> for CircuitHandler {
//     type Effect = CircuitEffect;
//     async fn handle(&mut self, effect: CircuitEffect) -> Computation<Voltage> {
//         Computation::Pure(effect.perform().await)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_circuit_handler() {
//     }
// }
