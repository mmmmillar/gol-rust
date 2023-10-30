use gol_rust::Universe;

#[test]
fn live_neighbour_count_no_neighbours() {
    let u = Universe::new_with_cells(3, 3, &[(1, 1)]);
    assert_eq!(0, u.live_neighbour_count(1, 1));
}

#[test]
fn live_neighbour_count_all_neighbours() {
    let u = Universe::new_with_cells(
        3,
        3,
        &[
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ],
    );
    assert_eq!(8, u.live_neighbour_count(1, 1));
}

#[test]
fn live_neighbour_count_top_left_corner() {
    let u = Universe::new_with_cells(3, 3, &[(0, 0), (0, 1), (1, 0), (1, 1)]);
    assert_eq!(3, u.live_neighbour_count(0, 0));
}

#[test]
fn live_neighbour_count_top_right_corner() {
    let u = Universe::new_with_cells(3, 3, &[(1, 0), (2, 0), (1, 1), (2, 1)]);
    assert_eq!(3, u.live_neighbour_count(2, 0));
}

#[test]
fn live_neighbour_count_bottom_left_corner() {
    let u = Universe::new_with_cells(3, 3, &[(0, 1), (0, 2), (1, 1), (1, 2)]);
    assert_eq!(3, u.live_neighbour_count(0, 2));
}

#[test]
fn live_neighbour_count_bottom_right_corner() {
    let u = Universe::new_with_cells(3, 3, &[(2, 1), (2, 2), (1, 1), (1, 2)]);
    assert_eq!(3, u.live_neighbour_count(2, 2));
}

#[test]
fn tick_when_live_and_two_live_neighbours() {
    let mut u = Universe::new_with_cells(3, 3, &[(1, 0), (1, 1), (1, 2)]);
    u.tick();
    assert_eq!(true, u.is_live(1, 1));
}

#[test]
fn tick_when_live_and_three_live_neighbours() {
    let mut u = Universe::new_with_cells(3, 3, &[(0, 1), (1, 0), (1, 1), (1, 2)]);
    u.tick();
    assert_eq!(true, u.is_live(1, 1));
}

#[test]
fn tick_when_live_and_four_live_neighbours() {
    let mut u = Universe::new_with_cells(3, 3, &[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]);
    u.tick();
    assert_eq!(false, u.is_live(1, 1));
}

#[test]
fn tick_when_live_and_one_live_neighbours() {
    let mut u = Universe::new_with_cells(3, 3, &[(1, 0), (1, 1)]);
    u.tick();
    println!("{:?}", u.cells());
    assert_eq!(false, u.is_live(1, 1));
}

#[test]
fn tick_when_dead_and_three_live_neighbours() {
    let mut u = Universe::new_with_cells(3, 3, &[(0, 1), (1, 0), (1, 2)]);
    u.tick();
    assert_eq!(true, u.is_live(1, 1));
}

#[test]
fn tick_when_dead_and_two_live_neighbours() {
    let mut u = Universe::new_with_cells(3, 3, &[(1, 0), (1, 2)]);
    u.tick();
    assert_eq!(false, u.is_live(1, 1));
}

#[test]
fn tick_when_dead_and_four_live_neighbours() {
    let mut u = Universe::new_with_cells(3, 3, &[(1, 0), (0, 1), (2, 1), (1, 2)]);
    u.tick();
    assert_eq!(false, u.is_live(1, 1));
}

// #[test]
// fn renders_correctly() {
//     let u = Universe::new_with_cells(3, 3, &[(1, 0), (0, 1), (2, 1), (1, 2)]);
//     assert_eq!(" ◼ \n◼ ◼\n ◼ \n", u.render())
// }
