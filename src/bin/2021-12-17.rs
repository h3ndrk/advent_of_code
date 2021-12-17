use std::cmp::{max, min, Ordering};

#[derive(Clone, Copy, Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, Debug)]
struct Target {
    start_x: isize,
    start_y: isize,
    end_x: isize,
    end_y: isize,
}

impl Target {
    fn is_inside(&self, point: &Point) -> bool {
        point.x >= self.start_x
            && point.x <= self.end_x
            && point.y >= self.start_y
            && point.y <= self.end_y
    }

    fn distance_to(&self, point: &Point) -> f32 {
        let distance_x = if point.x >= self.start_x && point.x <= self.end_x {
            0
        } else {
            min((self.start_x - point.x).abs(), (self.end_x - point.x).abs())
        };
        let distance_y = if point.y >= self.start_y && point.y <= self.end_y {
            0
        } else {
            min(self.start_y - point.y, (self.end_y - point.y).abs())
        };
        ((distance_x as f32).powi(2) + (distance_y as f32).powi(2)).sqrt()
    }
}

#[test]
fn test_target_distance() {
    assert!(!Target {start_x: 10,start_y: 20,end_x: 20,end_y: 30    }    .is_inside(&Point { x: 0, y: 0 }));
    assert!(!Target {start_x: 10,start_y: 20,end_x: 20,end_y: 30    }    .is_inside(&Point { x: 10, y: 0 }));
    assert!(!Target {start_x: 10,start_y: 20,end_x: 20,end_y: 30    }    .is_inside(&Point { x: 10, y: 10 }));
    assert!(Target {start_x: 10,start_y: 20,end_x: 20,end_y: 30    }    .is_inside(&Point { x: 10, y: 20 }));
    assert!(!Target {start_x: 10,start_y: 20,end_x: 20,end_y: 30    }    .is_inside(&Point { x: 9, y: 20 }));
    assert!(Target {start_x: 10,start_y: 20,end_x: 20,end_y: 30    }    .is_inside(&Point { x: 20, y: 20 }));
    assert!(!Target {start_x: 10,start_y: 20,end_x: 20,end_y: 30    }    .is_inside(&Point { x: 21, y: 20 }));
    assert!(Target {start_x: 10,start_y: 20,end_x: 20,end_y: 30    }    .is_inside(&Point { x: 20, y: 30 }));
    assert!(!Target {start_x: 10,start_y: 20,end_x: 20,end_y: 30    }    .is_inside(&Point { x: 20, y: 31 }));
    assert!(!Target {start_x: 10,start_y: 20,end_x: 20,end_y: 30    }    .is_inside(&Point { x: 21, y: 30 }));
    assert!(!Target {start_x: 10,start_y: 20,end_x: 20,end_y: 30    }    .is_inside(&Point { x: 21, y: 31 }));
    
    assert!(!Target {start_x: 10,start_y: -30,end_x: 20,end_y: -20    }    .is_inside(&Point { x: 0, y: -0 }));
    assert!(!Target {start_x: 10,start_y: -30,end_x: 20,end_y: -20    }    .is_inside(&Point { x: 10, y: -0 }));
    assert!(!Target {start_x: 10,start_y: -30,end_x: 20,end_y: -20    }    .is_inside(&Point { x: 10, y: -10 }));
    assert!(Target {start_x: 10,start_y: -30,end_x: 20,end_y: -20    }    .is_inside(&Point { x: 10, y: -20 }));
    assert!(!Target {start_x: 10,start_y: -30,end_x: 20,end_y: -20    }    .is_inside(&Point { x: 9, y: -20 }));
    assert!(Target {start_x: 10,start_y: -30,end_x: 20,end_y: -20    }    .is_inside(&Point { x: 20, y: -20 }));
    assert!(!Target {start_x: 10,start_y: -30,end_x: 20,end_y: -20    }    .is_inside(&Point { x: 21, y: -20 }));
    assert!(Target {start_x: 10,start_y: -30,end_x: 20,end_y: -20    }    .is_inside(&Point { x: 20, y: -30 }));
    assert!(!Target {start_x: 10,start_y: -30,end_x: 20,end_y: -20    }    .is_inside(&Point { x: 20, y: -31 }));
    assert!(!Target {start_x: 10,start_y: -30,end_x: 20,end_y: -20    }    .is_inside(&Point { x: 21, y: -30 }));
    assert!(!Target {start_x: 10,start_y: -30,end_x: 20,end_y: -20    }    .is_inside(&Point { x: 21, y: -31 }));
}

#[derive(Clone, Debug)]
struct Trajectory {
    points: Vec<Point>,
}

impl Trajectory {
    fn target_hit(&self, target: &Target) -> bool {
        self.points.iter().any(|point| target.is_inside(point))
    }

    fn minimal_distance_to_target(&self, target: &Target) -> f32 {
        self.points
            .iter()
            .map(|point| target.distance_to(point))
            .min_by(|distance_a, distance_b| {
                if distance_a == distance_b {
                    Ordering::Equal
                } else if distance_a < distance_b {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .unwrap()
    }

    fn maximum_y(&self) -> isize {
        self.points.iter().map(|point| point.y).max().unwrap()
    }
}

#[test]
fn test_trajectory_target_hit() {
    assert!(!Trajectory {
        points: vec![
            Point { x: 0, y: 0 },
            Point { x: 10, y: 0 },
            Point { x: 10, y: 10 },
            Point { x: 9, y: 20 },
            Point { x: 21, y: 20 },
            Point { x: 20, y: 31 },
            Point { x: 21, y: 30 },
            Point { x: 21, y: 31 },
        ],
    }
    .target_hit(&Target {
        start_x: 10,
        start_y: 20,
        end_x: 20,
        end_y: 30,
    }));
    assert!(Trajectory {
        points: vec![
            Point { x: 0, y: 0 },
            Point { x: 10, y: 0 },
            Point { x: 10, y: 20 },
            Point { x: 10, y: 10 },
            Point { x: 9, y: 20 },
            Point { x: 21, y: 20 },
            Point { x: 20, y: 31 },
            Point { x: 21, y: 30 },
            Point { x: 21, y: 31 },
        ],
    }
    .target_hit(&Target {
        start_x: 10,
        start_y: 20,
        end_x: 20,
        end_y: 30,
    }));
    assert!(Trajectory {
        points: vec![
            Point { x: 0, y: 0 },
            Point { x: 10, y: 0 },
            Point { x: 10, y: 10 },
            Point { x: 9, y: 20 },
            Point { x: 21, y: 20 },
            Point { x: 20, y: 20 },
            Point { x: 20, y: 31 },
            Point { x: 21, y: 30 },
            Point { x: 21, y: 31 },
        ],
    }
    .target_hit(&Target {
        start_x: 10,
        start_y: 20,
        end_x: 20,
        end_y: 30,
    }));
    assert!(Trajectory {
        points: vec![
            Point { x: 0, y: 0 },
            Point { x: 10, y: 0 },
            Point { x: 10, y: 10 },
            Point { x: 9, y: 20 },
            Point { x: 21, y: 20 },
            Point { x: 20, y: 31 },
            Point { x: 21, y: 30 },
            Point { x: 20, y: 30 },
            Point { x: 21, y: 31 },
        ],
    }
    .target_hit(&Target {
        start_x: 10,
        start_y: 20,
        end_x: 20,
        end_y: 30,
    }));
}

fn infinite_series(n: isize) -> isize {
    n * (n + 1) / 2
}

fn main() {
    let target = Target {
        start_x: 288,
        start_y: -96,
        end_x: 330,
        end_y: -50,
    };
    // let target = Target {
    //     start_x: 20,
    //     start_y: -10,
    //     end_x: 30,
    //     end_y: -5,
    // };

    let mut amount_of_inside_shots = 0;
    for y in -1000..=1000 {
        println!("y: {}, amount_of_inside_shots: {}", y, amount_of_inside_shots);
        for x in -1000..=1000 {
            // println!(
            //     "velocity_x: {} (-> {}), velocity_y: {}",
            //     x,
            //     infinite_series(x),
            //     y
            // );
            let trajectory = simulate(Point { x, y });
            let target_hit = trajectory.target_hit(&target);
            // print!("{},", trajectory.minimal_distance_to_target(&target));
            // println!(
            //     "target_hit: {}, minimal_distance_to_target: {}",
            //     target_hit,
            //     trajectory.minimal_distance_to_target(&target)
            // );
            if target_hit {
                // println!("{}, {}", x, y);
                amount_of_inside_shots += 1;
            }
        }
        // println!();
    }
    println!("amount_of_inside_shots: {}", amount_of_inside_shots);
    let trajectory = simulate(Point { x: 25, y: 95 });
    println!(
        "distance: {}, maximum_y: {}",
        trajectory.minimal_distance_to_target(&target),
        trajectory.maximum_y()
    );
}

fn simulate(mut velocity: Point) -> Trajectory {
    let mut trajectory = Trajectory {
        points: vec![Point { x: 0, y: 0 }],
    };
    for _ in 0..2000 {
        let point = *trajectory.points.last().unwrap();
        trajectory.points.push(Point {
            x: point.x + velocity.x,
            y: point.y + velocity.y,
        });
        velocity.x = max(0, velocity.x - 1);
        velocity.y -= 1;
    }
    trajectory
}
