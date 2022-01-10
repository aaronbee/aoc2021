fn check_y(dy: i64, target_min: i64, target_max: i64) -> bool {
    let mut y: i64 = 0;
    let mut dy = if dy > 0 { -(dy + 1) } else { dy };
    loop {
        if y < target_min {
            return false;
        } else if y >= target_max {
            return true;
        }
        y += dy;
        dy -= 1;
    }
}

fn check_x(mut dx: i64, target_min: i64, target_max: i64) -> bool {
    let mut x = 0;
    loop {
        if x > target_max {
            return false;
        } else if x >= target_min {
            return true;
        } else if dx == 0 {
            return false;
        }
        x += dx;
        dx -= 1;
    }
}

fn check_x_y(
    mut dx: i64,
    mut dy: i64,
    target_x_min: i64,
    target_x_max: i64,
    target_y_min: i64,
    target_y_max: i64,
) -> bool {
    let mut x = 0;
    let mut y = 0;
    loop {
        if x > target_x_max || y < target_y_min {
            return false;
        } else if x >= target_x_min && y <= target_y_max {
            return true;
        }
        x += dx;
        y += dy;
        if dx > 0 {
            dx -= 1;
        }
        dy -= 1;
    }
}

fn main() {
    const TARGET_Y_MIN: i64 = -146;
    const TARGET_Y_MAX: i64 = -90;
    const MAX_Y: i64 = 146;
    let ys: Vec<i64> = (-MAX_Y..=MAX_Y)
        .filter(|y| check_y(*y, TARGET_Y_MIN, TARGET_Y_MAX))
        .collect();

    const TARGET_X_MIN: i64 = 102;
    const TARGET_X_MAX: i64 = 157;
    let mut count = 0;
    for dx in (1..=TARGET_X_MAX).filter(|dx| check_x(*dx, TARGET_X_MIN, TARGET_X_MAX)) {
        for dy in ys.iter() {
            if check_x_y(
                dx,
                *dy,
                TARGET_X_MIN,
                TARGET_X_MAX,
                TARGET_Y_MIN,
                TARGET_Y_MAX,
            ) {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
