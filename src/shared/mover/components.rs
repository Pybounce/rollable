
use avian3d::prelude::{LinearVelocity, RigidBody};
use bevy::{prelude::*};

#[derive(Component)]
#[require(LinearVelocity, RigidBody)]
pub struct OffsetMover {
    positions: Vec<Vec3>,
    current_position: Vec3,
    current_vel: Vec3,
    current_offset_index: usize,
    cycle_mode: OffsetMoverCycleMode,
    speed: f32
}

pub enum OffsetMoverCycleMode {
    None,
    Repeat,
    ReverseRepeat
}

impl OffsetMover {
    pub fn from_offsets(offsets: Vec<Vec3>) -> Self {
        let mut positions: Vec<Vec3> = vec![];
        let mut current_pos = Vec3::new(0.0, 0.0, 0.0);
        for offset in offsets.iter() {
            current_pos += offset;
            positions.push(current_pos);
        }

        return Self {
            positions: positions,
            current_offset_index: 0,
            current_position: Vec3::new(0.0, 0.0, 0.0),
            cycle_mode: OffsetMoverCycleMode::None,
            speed: 5.0,
            current_vel: Vec3::new(0.0, 0.0, 0.0)
        };
    }

    pub fn bobbing_offset(distance: f32) -> Self {
        let offset_mover = OffsetMover::from_offsets(vec![Vec3::new(0.0, distance, 0.0), Vec3::new(0.0, -distance, 0.0)]);
        return offset_mover;
    }

    pub fn with_cycle_mode(&mut self, cycle_mode: OffsetMoverCycleMode) {
        self.cycle_mode = cycle_mode;
    }
    pub fn with_speed(&mut self, speed: f32) -> &mut OffsetMover {
        self.speed = speed;
        return self;
    }

}

impl OffsetMover {
    pub fn progress(&mut self, time_passed: f32) {
        self.calc_new_velocity();
        let mut distance_to_move = time_passed * self.current_vel.length();
        while distance_to_move > 0.0 {
            let target = self.positions[self.current_offset_index];
            let dist_to_target = self.current_position.distance(target);
            
            let dist_moved = dist_to_target.min(distance_to_move);
            let dir = (target - self.current_position).normalize_or_zero();

            distance_to_move -= dist_moved;
            self.current_position += dir * dist_moved;

            let dist_remaining = self.current_position.distance(target);
            if dist_remaining < 0.01 {
                self.next_offset();
            }
            self.calc_new_velocity();
        }
    }
    pub fn current_offset(&self) -> Vec3 {
        return self.current_position;
    }

    pub fn current_velocity(&self) -> Vec3 {
        return self.current_vel;
    }

    fn calc_new_velocity(&mut self) {
        let target = self.positions[self.current_offset_index];
        let dir = (target - self.current_position).normalize_or_zero();
        self.current_vel = dir * self.speed;

        let slowdown_range = 4.0;   // distance from target that it starts to slowdown.
        if self.min_dist_from_end_target() < slowdown_range {
            let slowdown_speed = self.speed / 3.0;   
            self.current_vel = (dir * slowdown_speed).lerp(dir * self.speed, self.min_dist_from_end_target() / slowdown_range);
        }
    }

    /// The smallest distance from either the first or last target
    fn min_dist_from_end_target(&self) -> f32 {
        //TODO: Currently if the distance between the final offset and second to last one is smaller than the slowdown range, it will act strange and not be smooth
        //slow down if first/last offset
        let front_dist = self.current_position.distance(self.positions[0]);
        let back_dist = self.current_position.distance(self.positions[self.positions.len() - 1]);
        return front_dist.min(back_dist);
    }

    fn next_offset(&mut self) {
        self.current_offset_index += 1;
        if self.current_offset_index >= self.positions.len() {
            self.current_offset_index = 0;
            self.current_position = Vec3::new(0.0, 0.0, 0.0);
        }
    }
}

