package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Point struct {
	x int
	y int
}

func getInput() string {
	return `forward 5
down 5
forward 8
up 3
down 8
forward 2`
}

func parseLine(line string) Point {
	parts := strings.Split(line, " ")
	amount, _ := strconv.Atoi(parts[1])

	direction := parts[0]
	if direction == "forward" {
		return Point{
			x: amount,
			y: 0,
		}
	} else if direction == "up" {
		return Point{
			x: 0,
			y: -amount,
		}
	}

	return Point{
		x: 0,
		y: amount,
	}
}

func main() {
	pos := Point{
		x: 0,
		y: 0,
	}
	lines := strings.Split(getInput(), "\n")
	for _, line := range lines {
		p := parseLine(line)
		pos.x += p.x
		pos.y += p.y
	}

	fmt.Printf("x: %d, y: %d\n", pos.x, pos.y)
}
