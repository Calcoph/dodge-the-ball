use crate::World;

#[test]
fn update_ball() {
    let mut world = World::new();

    world.add_ball(50.0, 10.0, 45.0);
    world.add_ball(50.0, 10.0, 60.0);
    world.add_ball(50.0, 10.0, 80.0);
    world.add_ball(50.0, 10.0, 30.0);
    world.add_ball(50.0, 10.0, 15.0);
    world.add_dodger(150.0, 5.0, 1.0);
    world.add_dodger(150.0, 150.0, -1.0);
    
    for i in 1..10000000 {
        world.tick();
        world.get_dodger_amount();
        world.dodger_positions();
        world.corridor_length();
        world.get_ball_amount();
        world.ball_positions();
        world.get_dodger_amount();
        world.get_counters();
        if i%10000==0 {
            println!("{}", i);
        }
    }
}
