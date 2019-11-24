use crate::{components::*, resources::*, util::*};
use kiss3d::event::Key::Space;
use na::zero;
use specs::prelude::*;

#[derive(Default)]
pub struct ShootingSystem {
    cooldown: TimePrecision,
}

impl ShootingSystem {
    const COOLDOWN: TimePrecision = 0.2;
    pub fn name() -> &'static str {
        "sim::shooting_system"
    }
}

impl<'s> specs::System<'s> for ShootingSystem {
    type SystemData = (
        ReadExpect<'s, Player>,
        Read<'s, LazyUpdate>,
        Read<'s, CurrentInput>,
        Read<'s, Time>,
        Entities<'s>,
        ReadStorage<'s, Vel>,
        ReadStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (player, updater, input, timer, entities, vels, transform): Self::SystemData,
    ) {
        self.cooldown = (self.cooldown - timer.delta()).max(zero());
        if self.cooldown <= zero() && input.keys.contains(&Space) {
            let ptrans = transform.get(**player).expect("No transform of player?!");
            let mut ntrans = Transform::default().with_position(ptrans.position.clone());
            ntrans.position[1] += 1.; // fire from up
            ntrans.position[2] += 1.; // Don't fire inside. Don't shoot yourself.
            let mut vel = Vel::from(vels.get(**player).expect("Player has no vel?!").0);
            vel.0[2] += 40.;
            updater
                .create_entity(&entities)
                .with(ObjectKind::Obstacle)
                .with(Extent::new(0.1))
                .with(Health::one())
                .with(NodeBuilder::projectile())
                .with(ntrans)
                .with(vel)
                .build();
            self.cooldown = Self::COOLDOWN;
        }
    }
}