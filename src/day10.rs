fn line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
    let mut path: Vec<(i32, i32)> = Vec::new();
    let deltax = x1 - x0;
    let deltay = y1 - y0;
    let deltaerr = (deltay as f32 / deltax as f32).abs(); // Assume deltax != 0 (line is not vertical),
                                                          // note that this division needs to be done in a way that preserves the fractional part
    let mut error = 0.0; // No error at start
    let mut y = y0;
    for x in x0..x1 {
        path.push((x, y));
        error += deltaerr;
        if error >= 0.5 {
            y += deltay.signum();
            error -= 1.0;
        }
    }

    path
}

#[aoc(day10, part1)]
fn part1(input: &str) -> String {
    let _map: Vec<char> = input.replace('\n', "").chars().collect();
    let mut map_plot: Vec<char> = input
        .chars()
        // .map(|x| match x {
        //     '\n' => x,
        //     _ => ' ',
        // })
        .collect();

    let path = line(0, 0, 8, 6);
    for point in path.iter() {
        println!("{:?}", point);
        map_plot[(point.0 + point.1 * 32) as usize] = 'X';
    }

    let map_plot: String = map_plot.into_iter().collect();
    println!("{}", map_plot);

    let mut input = input.replace('\n', "").to_owned();
    input.insert(0, '\n');
    input
}
