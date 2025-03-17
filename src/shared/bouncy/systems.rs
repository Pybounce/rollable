
use avian3d::prelude::{CollidingEntities, LinearVelocity};
use bevy::prelude::*;

use super::components::{Bounceable, Bouncy};

pub fn bounce(
    mut bounceable_query: Query<(&mut LinearVelocity, &CollidingEntities), With<Bounceable>>,
    bouncy_query: Query<(&Bouncy, &Transform)>
) {
    for (mut linvel, colliding_entities) in &mut bounceable_query {
        for colliding_entity in colliding_entities.iter() {
            if let Ok((bouncy, transform)) = bouncy_query.get(*colliding_entity) {
                let force = transform.rotation * (Vec3::Y * bouncy.factor);
                linvel.0 += force;
            }
        }
    }
}