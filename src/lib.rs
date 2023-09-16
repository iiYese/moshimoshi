//! # moshimoshi
//! A small crate to sugar working with command callbacks in bevy.
//! ```rust
//! use bevy::prelude::*;
//! use moshimoshi::*;
//!
//! #[derive(Component)]
//! struct Button;
//!
//! #[derive(Component, Deref, DerefMut)]
//! struct OnClick(EntityCallback);
//!
//! #[derive(Component, Deref, DerefMut)]
//! struct Counter(u32);
//!
//! #[derive(Component)]
//! struct Text(String);
//!
//! fn setup(mut commands: Commands) {
//!     commands.spawn((
//!         Button,
//!         Counter(0),
//!         Text("Click Me".to_string()),
//!         OnClick(moshi!([e: Entity], counter: Query<&mut Counter> => {
//!             **counter.get_mut(e).unwrap() += 1;
//!         }))
//!     ));
//! }
//!
//! impl Button {
//!     fn update(mut commands: Commands, buttons: Query<(Entity, &OnClick), Changed<Button>>) {
//!         for (entity, callback) in buttons.iter() {
//!             commands.add(RunEntityCallback { entity, func: **callback });
//!         }
//!     }
//! }
//!
//! fn main() {
//!     App::new()
//!         .add_systems(Update, (Button::update, apply_deferred).chain())
//!         .run()
//! }
//! ```

pub use bevy::ecs::{
    entity::Entity,
    system::{Command, SystemState},
    world::World,
};

pub type EntityCallback = fn(Entity, &mut World);

#[derive(Clone, Copy)]
pub struct RunEntityCallback {
    pub entity: Entity,
    pub func: EntityCallback,
}

impl Command for RunEntityCallback {
    fn apply(self, world: &mut World) {
        (self.func)(self.entity, world)
    }
}

#[macro_export]
macro_rules! moshi {
    ($($I:ident: $T:ty),* => $B:block) => {
        |mut world: &mut World| {
            let mut system_state: SystemState<($($T),*)> = SystemState::new(world);
            #[allow(unused_mut)]
            let ($(mut $I),*) = system_state.get_mut(&mut world);
            $B
        }
    };
    ([$($LI:ident: $LT:ty),*], $($I:ident: $T:ty),* => $B:block) => {
        |$($LI: $LT),*, mut world: &mut World| {
            let mut system_state: SystemState<($($T),*)> = SystemState::new(world);
            #[allow(unused_mut)]
            let ($(mut $I),*) = system_state.get_mut(&mut world);
            $B
        }
    };
}
