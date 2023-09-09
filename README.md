# moshimoshi
A small crate to sugar working with command callbacks in bevy.

[![Crates.io](https://img.shields.io/crates/v/moshimoshi)](https://crates.io/crates/moshimoshi)
[![Docs.rs](https://img.shields.io/docsrs/moshimoshi)](https://docs.rs/aery/latest/moshimoshi)

```rust
use bevy::prelude::*;
use moshimoshi::*;

#[derive(Component)]
struct Button;

#[derive(Component)]
struct OnClick(EntityCallback);

#[derive(Component, Deref, DerefMut)]
struct Counter(u32);

#[derive(Component)]
struct Text(String);

fn setup(mut commands: Commands) {
    commands.spawn((
        Button,
        Counter(0),
        Text("Click Me".to_string()),
        OnClick(moshi!(e, counter: Query<&mut Counter> => {
            **counter.get_mut(e).unwrap() += 1;
        }))
    ));
}

impl Button {
    fn update(mut commands: Commands, buttons: Query<(Entity, &OnClick), Changed<Button>>) {
        for (entity, callback) in buttons.iter() {
            commands.add(RunEntityCallback { entity, func: ***callback });
        }
    }
}

fn main() {
    App::new()
        .add_systems(Update, (Button::update, apply_deferred).chain())
        .run()
}
```
