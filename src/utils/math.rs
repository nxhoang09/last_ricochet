use bevy::prelude::*;
use crate::components::collider::Collider;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionSide {
    Left,
    Right,
    Top,
    Bottom,
    None,
}

pub fn check_collision(
    transform_a: &Transform,
    collider_a: &Collider,
    transform_b: &Transform,
    collider_b: &Collider,
) -> CollisionSide {
    let a_pos = transform_a.translation.truncate();
    let b_pos = transform_b.translation.truncate();

    let delta = a_pos - b_pos;
    let overlap_x = (collider_a.half_size.x + collider_b.half_size.x) - delta.x.abs();
    let overlap_y = (collider_a.half_size.y + collider_b.half_size.y) - delta.y.abs();

    if overlap_x > 0.0 && overlap_y > 0.0 {
        if overlap_x < overlap_y {
            // Va chạm theo trục X
            if delta.x > 0.0 {
                return CollisionSide::Right; // A ở bên phải B
            } else {
                return CollisionSide::Left; // A ở bên trái B
            }
        } else {
            // Va chạm theo trục Y
            if delta.y > 0.0 {
                return CollisionSide::Top; // A ở phía trên B
            } else {
                return CollisionSide::Bottom; // A ở phía dưới B
            }
        }
    }

    CollisionSide::None
}