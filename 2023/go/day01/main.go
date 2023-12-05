package main

import (
	_ "embed"
	"flag"
	"fmt"
	"strconv"
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

func part1(input string) int {
	lines := strings.Split(input, "\n")
	sum := 0
	for _, line := range lines {
		first := -1
		last := -1
		for _, c := range line {
			num, err := strconv.Atoi(string(c))
			if err != nil {
				continue
			} else {
				if first == -1 {
					first = num
				}
				last = num
			}
		}
		num, err := strconv.Atoi(fmt.Sprintf("%d%d", first, last))
		if err != nil {
			panic(err)
		}
		sum += num
	}
	return sum
}

func part2(input string) int {
	fmt.Println("part 2")
	return 42
}
