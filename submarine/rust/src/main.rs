fn get_input() -> &'static str {
    return "forward 5
down 5
forward 8
up 3
down 8
forward 2";
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_input(line: &str) -> Point {
    line.split_once(" ").unwrap();
    let (direction, distance) = line.split_once(" ").unwrap();
    let amount = distance.parse::<i32>().unwrap();
    match direction {
        "forward" => Point { x: amount, y: 0 },
        "up" => Point { x: 0, y: -amount },
        "down" => Point { x: 0, y: amount },
        _ => panic!("Invalid direction"),
    }
}

fn main() {
    let result = get_input()
        .lines()
        .map(parse_input)
        .fold(Point { x: 0, y: 0 }, |acc, point| Point {
            x: acc.x + point.x,
            y: acc.y + point.y,
        });
    println!("Result: {:?}", result);
}
