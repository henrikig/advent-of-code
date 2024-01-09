package main

import (
	_ "embed"
	"flag"
	"fmt"
	"strings"
)

//go:embed input.txt
var input string

func main() {
	var part int
	flag.IntVar(&part, "part", 1, "part 1 or 2")
	flag.Parse()
	fmt.Println("Running part", part)

	if part == 1 {
		ans := part1(input)
		fmt.Println("Output:", ans)
	} else {
		ans := part2(input)
		fmt.Println("Output:", ans)
	}
}

var bag = map[string]int{
	"red":   12,
	"green": 13,
	"blue":  14,
}

func part1(input string) int {
	lines := strings.Split(input, "\n")
	sum := 0

Game:
	for _, line := range lines {
		game, rounds, _ := strings.Cut(line, ":")
		var gameNum int
		fmt.Sscanf(game, "Game %d", &gameNum)

		for _, round := range strings.Split(rounds, ";") {
			round = strings.TrimSpace(round)

			for _, draw := range strings.Split(round, ", ") {
				var color string
				var amount int
				fmt.Sscanf(draw, "%d %s", &amount, &color)
				if amount > bag[color] {
					continue Game
				}
			}
		}
		sum += gameNum
	}
	return sum
}

func part2(input string) int {
	lines := strings.Split(input, "\n")
	sum := 0

	for _, game := range lines {
		var max_map = map[string]int{
			"red":   0,
			"green": 0,
			"blue":  0,
		}

		_, rounds, _ := strings.Cut(game, ":")
		for _, round := range strings.Split(rounds, ";") {
			round = strings.TrimSpace(round)

			for _, draw := range strings.Split(round, ", ") {
				var color string
				var amount int
				fmt.Sscanf(draw, "%d %s", &amount, &color)
				if amount > max_map[color] {
					max_map[color] = amount
				}
			}
		}
		sum += multiplyMapValues(max_map)
	}
	return sum
}

func multiplyMapValues(m map[string]int) int {
	sum := 1
	for _, value := range m {
		sum *= value
	}
	return sum
}
