use bevy::prelude::*;

#[derive(Component)]
pub struct Fall{
    velocity: f32,
}

impl Fall {
    pub fn new(velocity: f32) -> Self{
        Self { velocity }
    }
}

#[derive(Component)]
pub struct Collider;


pub fn update_fall(mut query: Query<(&mut Transform, &Fall)>, time: Res<Time>) {
    if let Some((mut transform, fall)) = query.iter_mut().next() {
        transform.translation.y -= fall.velocity * time.delta_seconds();
    }

}
