//! represents a creature in the world that can eat grass and reproduce
//!
mod code;
use super::entity::{Cell, Entity};
use crate::settings::Settings;
use crate::utils::move_pos;
use crate::world::types::{Update, UpdateQueue};
use code::Processor;
use direction::{Coord, Direction};
use fastrand::Rng as FastRng;

#[derive(Debug)]
pub struct Creature {
    id: u64,
    coord: Coord,
    code: Processor,
    config: Settings,
    rng: FastRng,
}

impl Entity for Creature {
    fn new(coord: Coord, config: Settings) -> Creature {
        let rng = FastRng::new();
        let (b, e) = config.creature_initial_energy;
        let energy = rng.i32(b..e);
        Creature {
            id: 0,
            coord,
            code: Processor::new(energy),
            rng,
            config,
        }
    }

    fn cell_type(id: u64) -> Cell {
        Cell::Creature(id)
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn coord(&self) -> Coord {
        self.coord
    }

    fn move_to(&mut self, pos: Coord) {
        self.coord = pos;
    }

    fn set_id(&mut self, id: u64) {
        // id is immutable once set
        if self.id == 0 {
            self.id = id;
        }
    }

    fn tick(&mut self, queue: &mut UpdateQueue) {
        self.tick(queue)
    }
}

impl Creature {
    fn random_direction(&self) -> Direction {
        // TODO is this the fastest way to do this?
        match self.rng.u8(0..8) {
            0 => Direction::North,
            1 => Direction::NorthEast,
            2 => Direction::East,
            3 => Direction::SouthEast,
            4 => Direction::South,
            5 => Direction::SouthWest,
            6 => Direction::West,
            7 => Direction::NorthWest,
            _ => unreachable!(),
        }
    }

    pub fn tick(&mut self, queue: &mut UpdateQueue) {
        self.code.energy -= self.config.creature_idle_energy;

        self.code.tick();

        if self.code.energy <= 0 {
            queue.push(Update::RemoveCreature(self.id, self.coord()));
        } else if self.code.energy >= self.config.creature_reproduction_energy {
            self._reproduce(queue);
        } else if self.rng.f32() <= self.config.creature_move_rate {
            let direction = self.random_direction();
            let new_pos = move_pos(self.coord, direction, self.config.size);

            self.code.energy -= self.config.creature_move_energy;
            queue.push(Update::MoveCreature(self.id(), self.coord(), new_pos));
        }
    }

    pub fn eat(&mut self, amount: i32) {
        self.code.energy += amount;
    }

    pub fn _reproduce(&mut self, queue: &mut UpdateQueue) {
        let mut child = Creature::new(self.coord, self.config);
        self.code.energy /= 2;
        child.code.energy = self.code.energy;
        // child is spawned to the left unless we are against the left wall
        if self.coord.x == 0 {
            child.coord.x += 1;
        } else {
            child.coord.x -= 1
        }
        queue.push(Update::AddCreature(child));
    }
}
