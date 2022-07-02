// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::cmp::min;

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        self.dead().then(|| Player {
            health: 100,
            mana: (self.level >= 10).then(|| 100),
            level: self.level,
        })
    }

    fn dead(&self) -> bool {
        self.health == 0
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        self.attribute_mana_cost(mana_cost);
        self.damage(mana_cost)
    }

    fn attribute_mana_cost(&mut self, cost: u32) {
        if self.mana.is_none() {
            self.reduce_health_by(cost);
        } else if self.has_enough_mana_for(cost) {
            self.reduce_mana_by(cost);
        }
    }

    fn reduce_health_by(&mut self, amount: u32) {
        self.health = self.health.saturating_sub(amount);
    }

    fn reduce_mana_by(&mut self, amount: u32) {
        self.mana = self.mana.map(|mana| mana - amount);
    }

    fn has_enough_mana_for(self, amount: u32) -> bool {
        self.mana.map_or(false, |mana| mana >= amount)
    }

    fn damage(&mut self, mana_cost: u32) -> u32 {
        self.has_enough_mana_for(mana_cost)
            .then(|| mana_cost * 2)
            .unwrap_or_default()
    }
}
