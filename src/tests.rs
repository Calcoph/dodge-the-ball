use crate::World;

#[test]
fn update_ball() {
    let mut world = World::new();
    
    for _ in 1..10 {
        world.tick();
        world.ball_positions();
    }
}
