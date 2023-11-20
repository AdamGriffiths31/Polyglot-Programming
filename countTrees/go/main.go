package main

import "strings"

type Item = int

const (
	Tree Item = iota
	Snow
)

func getInput() string {
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
.#..#...#.#`
}

func main() {
	treeCount := 0
	for row, line := range strings.Split(getInput(), "\n") {
		if line[row*3%len(line)] == '#' {
			treeCount++
		}
	}
	println(treeCount)
}
