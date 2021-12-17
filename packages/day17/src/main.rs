use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    let bbox = ((248,-56), (285,-85));
    // let bbox = ((20,-5), (30,-10));
    let now = Instant::now();

    println!("Answer 1: {:?}", part_1(bbox));
    println!("Answer 2: {}", part_2(bbox));


    println!(
        "Solution took {} ms",
        now.elapsed().as_micros() as f64 / 1000.0
    );
}

fn part_1(bbox: BBox) -> isize {
    let mut highest_y = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if let Some(result) = shoot_probe(bbox, (x,y)) {
                highest_y = highest_y.max(result);
            }
        }
    }


    highest_y
}

fn part_2(bbox: BBox) -> usize {
    let mut hit_count = HashSet::new();
    for x in -1000..1000 {
        for y in -1000..1000 {
            if let Some(result) = shoot_probe(bbox, (x,y)) {
                hit_count.insert((x,y));
            }
        }
    }

    hit_count.len()
}

fn shoot_probe(bbox: BBox, start_vel: Velocity) -> Option<isize> {
    let mut vel = start_vel;
    let mut probe_pos = (0,0);

    let mut y = 0;

    while(!moved_passed_bb(bbox, probe_pos) && !moved_below_bb(bbox, probe_pos)) {
        let result = step_probe(probe_pos, vel);

        probe_pos = result.0;
        vel = result.1;
        y = y.max(probe_pos.1);

        if is_in_bb(bbox, probe_pos) {
            return Some(y);
        }
    }
    None
}

type Velocity = (isize, isize);
type Pos = (isize, isize);
type BBox = (Pos, Pos);

fn step_probe(pos: Pos, vel: Velocity) -> (Pos, Velocity) {
        let new_pos_x = pos.0 + vel.0;
        let new_pos_y = pos.1 + vel.1;

    let new_vel_x = vel.0 - 1;
    let new_vel_x = new_vel_x.max(0);
    let new_vel_y = vel.1 -1;


    ((new_pos_x, new_pos_y), (new_vel_x, new_vel_y))
}

fn is_in_bb(bbox: BBox, pos: Pos) -> bool {
    pos.0 >= bbox.0.0 && pos.0 <= bbox.1.0 && pos.1 <= bbox.0.1 && pos.1 >= bbox.1.1
}

fn moved_passed_bb(bbox: BBox, pos: Pos) -> bool {
    pos.0 > bbox.1.0
}

fn moved_below_bb(bbox: BBox, pos: Pos) -> bool {
    pos.1 < bbox.1.1
}


#[cfg(test)]
mod tests {
    use crate::{is_in_bb, step_probe};

    #[test]
    fn it_should_be_in_out_box() {
        // Arrange
        let top = (0,5);
        let left = (5,0);
        let right = (5,10);
        let bottom = (10,5);
        let in_box = (5,5);

        let bbox = ((4,4), (8,8));

        // Assert
        assert_eq!(is_in_bb(bbox, top), false);
        assert_eq!(is_in_bb(bbox, left), false);
        assert_eq!(is_in_bb(bbox, right), false);
        assert_eq!(is_in_bb(bbox, bottom), false);
        assert_eq!(is_in_bb(bbox, in_box), true);
    }

    #[test]
    fn it_should_step_gravity() {
        // Arrange
        let pos = (0,0);
        let vel = (7,2);

        let step =  step_probe(pos, vel);

        // Assert
        assert_eq!(step, ((7,2),(6,1)));
    }

    #[test]
    fn it_should_perform_steps() {
        // Arrange
        let mut pos = (0,0);
        let mut vel = (7,2);
        let bbox = ((20,-10), (30, -5));

        for i in 0..6 {
            let step =  step_probe(pos, vel);
            pos = step.0;
            vel = step.1;
            assert_eq!(is_in_bb(bbox, pos), false);
        }

        // Assert
        let step =  step_probe(pos, vel);
        assert_eq!(is_in_bb(bbox, step.0), true);
    }
}
