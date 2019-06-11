use crate::{
    comp::{phys, Animation, AnimationInfo, Attacking, Rolling, Crunning, Cidling, Gliding, Jumping, OnGround},
    state::DeltaTime,
};
use specs::{Entities, Join, Read, ReadStorage, System, WriteStorage};

/// This system will apply the animation that fits best to the users actions
pub struct Sys;
impl<'a> System<'a> for Sys {
    type SystemData = (
        Entities<'a>,
        Read<'a, DeltaTime>,
        ReadStorage<'a, phys::Vel>,
        ReadStorage<'a, OnGround>,
        ReadStorage<'a, Jumping>,
        ReadStorage<'a, Gliding>,
        ReadStorage<'a, Attacking>,
        ReadStorage<'a, Rolling>,
        ReadStorage<'a, Crunning>,
        ReadStorage<'a, Cidling>,
        WriteStorage<'a, AnimationInfo>,
    );

    fn run(
        &mut self,
        (entities, dt, velocities, on_grounds, jumpings, glidings, attackings, rollings, crunnings, cidlings, mut animation_infos): Self::SystemData,
    ) {
        for (entity, vel, on_ground, jumping, gliding, attacking, rolling, crunning, cidling, mut animation_info) in (
            &entities,
            &velocities,
            on_grounds.maybe(),
            jumpings.maybe(),
            glidings.maybe(),
            attackings.maybe(),
            rollings.maybe(),
            crunnings.maybe(),
            cidlings.maybe(),
            &mut animation_infos,
        )
            .join()
        {
            animation_info.time += dt.0 as f64;
            let moving = vel.0.magnitude() > 3.0;

            fn impossible_animation() -> Animation {
                warn!("Impossible animation");
                Animation::Idle
            }

            let animation = match (
                on_ground.is_some(),
                moving,
                attacking.is_some(),
                gliding.is_some(),
            ) {
                (true, false, false, false) => Animation::Idle,
                (true, true, false, false) => Animation::Run,
                (false, _, false, false) => Animation::Jump,
                (_, _, false, true) => Animation::Gliding,
                (_, _, true, false) => Animation::Attack,
                (true, true, false, false) => Animation::Roll,
                (_, true, false, false) => Animation::Crun,
                (true, false, false, false) => Animation::Cidle,
                (_, _, true, true) => impossible_animation(),
            };

            let last = animation_info.clone();
            let changed = last.animation != animation;

            *animation_info = AnimationInfo {
                animation,
                time: if changed { 0.0 } else { last.time },
                changed,
            };
        }
    }
}
