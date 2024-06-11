pub fn middle_pos_to_left_top_pos(x: i64, y: i64, width: i64, height: i64) -> (i64, i64) {
    let middle_x = width / 2;
    let middle_y = height / 2;

    (middle_x + x , middle_y + y)
}