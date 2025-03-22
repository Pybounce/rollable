
use bevy::prelude::*;

#[derive(Component)]
pub struct OffsetMover {
    positions: Vec<Vec3>,
    current_position: Vec3,
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
            speed: 5.0
        };
    }

    pub fn bobbing_offset(distance: f32) -> Self {
        let offset_mover = OffsetMover::from_offsets(vec![Vec3::new(0.0, distance, 0.0), Vec3::new(0.0, -distance, 0.0)]);
        return offset_mover;
    }

    pub fn with_cycle_mode(&mut self, cycle_mode: OffsetMoverCycleMode) {
        self.cycle_mode = cycle_mode;
    }
    pub fn with_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

}

impl OffsetMover {
    pub fn progress(&mut self, time_passed: f32) {
        let mut distance_to_move = time_passed * self.speed;
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
        }
    }
    pub fn current_offset(&self) -> Vec3 {
        return self.current_position;
    }

    fn next_offset(&mut self) {
        self.current_offset_index += 1;
        if self.current_offset_index >= self.positions.len() {
            self.current_offset_index = 0;
            self.current_position = Vec3::new(0.0, 0.0, 0.0);
        }
    }
}

