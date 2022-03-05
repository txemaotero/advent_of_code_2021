use std::cmp::max;

fn check_traj_and_get_ymax(vx_0: i32, vy_0: i32, target: &Target) -> (i64, bool) {
    let mut max_y = 0;
    let mut vx = vx_0;
    let mut vy = vy_0 as i64;
    let mut x = 0;
    let mut y: i64 = 0;
    loop {
        y += vy;
        vy -= 1;
        if vx != 0 {
            x += vx;
            vx -= 1;
        }
        max_y = max(max_y, y);
        if target.contains(x, y as i32) {
            return (max_y, true);
        };
        if x > target.x_max || y < target.y_min as i64 {
            return (0, false);
        }
    }
}

fn part1(target: Target) -> (i64, u32) {
    let mut max_y = target.y_min as i64;
    let mut n_traj = 0;
    for vx_0 in 0..=(target.x_max + 2) {
        for vy_0 in target.y_min..=(target.y_min.abs() + 1000) {
            let result = check_traj_and_get_ymax(vx_0, vy_0, &target);
            if result.1 {
                n_traj += 1;
                max_y = max(max_y, result.0);
            }
        }
    }
    (max_y, n_traj)
}

struct Target {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Target {
    fn contains(&self, x: i32, y: i32) -> bool {
        x <= self.x_max && x >= self.x_min && y <= self.y_max && y >= self.y_min
    }
}

fn main() {
    let target = Target {
        x_min: 85,
        x_max: 145,
        y_min: -163,
        y_max: -108,
    };
    let (max_y, n_traj) = part1(target);
    println!("Result of part 1: {}", max_y);
    println!("Result of part 2: {}", n_traj);
}