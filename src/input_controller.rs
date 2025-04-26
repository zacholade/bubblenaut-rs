use std::collections::HashMap;

use cgmath::{Vector2, Vector3, Zero};
use winit::{
    event::{ElementState, KeyEvent},
    keyboard::Key,
};

// pub enum GenericValue<T> {
//     Float(f32),
//     Vector2(Vector2<f32>),
//     Vector3(Vector3<f32>),
//     Delta(f32),
// }

// pub enum ActionType<T> {
//     Button(DiscreteValue),
//     Float(ContinuousValue<T>),
//     Vec2(ContinuousValue<T>),
//     Vec3(ContinuousValue<T>),
// }

// pub struct ContinuousValue<T> {
//     initial_value: T,
// }

// impl<T> ContinuousValue<T> {
//     pub fn new(initial_value: T) -> Self {
//         Self { initial_value }
//     }
// }

// pub struct DiscreteValue {
//     initial_value: bool,
// }

// pub struct Binding<T> {
//     magnitude: T,
//     key: Key,
// }

// pub struct Action<T> {
//     action_type: ActionType<T>,
//     bindings: Vec<Binding<T>>,
// }

// pub enum ActionKind {
//     Button(Action<bool>),
//     Toggle(Action<bool>),
//     Vec2(Action<Vector2<f32>>),
//     Vec3(Action<Vector3<f32>>),
// }

// pub struct Action<T: ActionKind> {
//     label: String,
//     initial_value: T,
//     bindings: Vec<Binding<T>>,
// }

// pub struct Binding<T> {
//     magnitude: T,
//     key: Key,
// }

// pub struct ActionMap {
//     actions: Vec<ActionType>,
// }

// impl ActionMap {
//     pub fn new() -> Self {
//         Self {
//             actions: Vec::new(),
//         }
//     }

//     pub fn add_action(&mut self, action_type: ActionType) -> &mut Self {
//         self.actions.push(action_type);
//         self
//     }

//     pub fn process_actions(&mut self, key_event: KeyEvent) {
//         for action in self.actions.iter_mut() {
//             for binding in action.bindings.iter() {
//                 if binding.key != key_event.logical_key {
//                     continue;
//                 }
//                 match action.action_type {
//                     ActionType::Button(pressed) => match key_event.state {
//                         ElementState::Pressed => {}
//                         ElementState::Released => {}
//                     },
//                     ActionType::Float(vlaue) => {}
//                     ActionType::Vec2(value) => {}
//                     ActionType::Vec3(value) => {}
//                 }
//             }
//         }
//     }
// }

// // #[cfg(test)]
// // mod tests {
// //     use cgmath::{Vector2, Vector3, Zero};
// //     use winit::keyboard::{Key, SmolStr};

// //     use super::{Action, ActionMap, ActionType, Binding, ContinuousValue, GenericValue};

// //     fn foo(f: Vector3<f32>) {}
// //     #[test]
// //     fn test_make_actions() {
// //         let mut input_manager: ActionMap<GenericValue<f32>> = ActionMap::new();
// //         let move_backward = Action {
// //             action_type: ActionType::Vec2(ContinuousValue::new(GenericValue::Vector2(
// //                 Vector2::zero(),
// //             ))),
// //             bindings: vec![Binding {
// //                 magnitude: GenericValue::Vector2(Vector2::new(0.0, -1.0)),
// //                 key: Key::Character("s".into()),
// //             }],
// //         };
// //         let action_map = ActionMap::new()
// //             .add_action(ActionType::Vec2(Action))
// //     }
// // }
