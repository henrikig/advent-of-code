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

type game struct {
	gameNum int
	rounds  []round
}

type round struct {
	draws map[string]int
}

func parseInput(input string) []game {
	lines := strings.Split(input, "\n")
	games := make([]game, len(lines))

	for i, line := range lines {
		split_line := strings.Split(line, ":")
		var gameNum int
		fmt.Sscanf(split_line[0], "Game %d", &gameNum)
		for _, record := range strings.Split(split_line[1], ";") {
			var rounds []round
			record = strings.TrimSpace(record)
			for _, draw := range strings.Split(record, ", ") {
				var color string
				var amount int
				fmt.Sscanf(draw, "%d %s", &amount, &color)
				round := round{
					color:  color,
					amount: amount,
				}
				rounds = append(rounds, round)
			}
		}
		game := game{
			gameNum: gameNum,
			rounds:  rounds,
		}
		games[i] = game
	}

	return games
}

func part1(input string) int {
	games := parseInput(input)
	for _, game := range games {
		for _, round := range game.rounds {
			fmt.Println(game.gameNum, round)
			fmt.Println()
		}
	}
	return 42
}

func part2(input string) int {
	return 42
}
