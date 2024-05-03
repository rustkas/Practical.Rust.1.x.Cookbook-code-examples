#[test]
fn ex01() {
    enum Direction {
        North,
        South,
        East,
        West,
    }
    struct Point {
        x: i32,
        y: i32,
    }

    fn move_point(point: &mut Point, dir: Direction) {
        match dir {
            Direction::North => point.y += 1,
            Direction::South => point.y -= 1,
            Direction::East => point.x += 1,
            Direction::West => point.x -= 1,
        }
    }

    let mut point = Point { x: 0, y: 0 };
    move_point(&mut point, Direction::North);
    move_point(&mut point, Direction::East);
    move_point(&mut point, Direction::West);
    move_point(&mut point, Direction::South);
    println!("Moved to ({}, {})", point.x, point.y);
}
