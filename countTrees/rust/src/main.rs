fn get_input() -> &'static str {
    return "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
}

fn main() {
    let result = get_input()
        .lines()
        .enumerate()
        .filter(|(i, line)| {
            let x = (i * 3) % line.len();
            line.chars().nth(x).unwrap() == '#'
        })
        .count();
    println!("{}", result);
}
