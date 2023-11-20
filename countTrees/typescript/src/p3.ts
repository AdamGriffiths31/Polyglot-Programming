function getInput(): string {
    return `..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#`;
}

enum Square {
    Tree = '#',
    Snow = '.',
}

const items = getInput()
    .split('\n')
    .map((line) => line.split('')
        .map(x => x === "." ? Square.Snow : Square.Tree));

const colLength = items[0].length;

let treeCount = 0;
items.forEach((row, i) => {
    row[i * 3 % colLength] === Square.Tree && treeCount++;
});

console.log(treeCount);
