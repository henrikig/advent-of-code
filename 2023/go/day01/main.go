package main

import (
	_ "embed"
	"flag"
	"fmt"
	"strconv"
	"strings"
)

var int_str = map[string]int{
	"one":   1,
	"two":   2,
	"three": 3,
	"four":  4,
	"five":  5,
	"six":   6,
	"seven": 7,
	"eight": 8,
	"nine":  9,
}

type LineResult struct {
	first int
	last  int
}

// newLineResult returns a new LineResult
func newLineResult() *LineResult {
	return &LineResult{
		first: -1,
		last:  -1,
	}
}

// setFirst sets the first number if it hasn't been set yet
func (l *LineResult) setFirst(num int) {
	if l.first == -1 {
		l.first = num
	}
}

// getConcatVal returns the concatenation of the first and last numbers
func (l *LineResult) getConcatVal() int {
	sum, err := strconv.Atoi(fmt.Sprintf("%d%d", l.first, l.last))
	if err != nil {
		panic(err)
	}
	return sum
}

// parseStr parses a string and sets the first and last numbers
func (l *LineResult) parseStr(str string) {
	num, err := strconv.Atoi(str)
	if err != nil {
		return
	} else {
		l.setFirst(num)
		l.last = num
	}
}

// numFromStr checks if the substring starting from `idx` is a number
func (l *LineResult) numFromStr(line string, idx int) int {
	for k := range int_str {
		if idx+len(k) > len(line) {
			continue
		}
		if line[idx:idx+len(k)] == k {
			l.setFirst(int_str[k])
			l.last = int_str[k]
			return int_str[k]
		}
	}
	return -1
}

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
		res := newLineResult()
		for _, c := range line {
			res.parseStr(string(c))
		}
		sum += res.getConcatVal()
	}
	return sum
}

func part2(input string) int {
	lines := strings.Split(input, "\n")
	sum := 0
	for _, line := range lines {
		l := newLineResult()
		for i := range line {
			num := l.numFromStr(line, i)
			if num != -1 {
				continue
			}
			l.parseStr(string(line[i]))
		}
		sum += l.getConcatVal()
	}

	return sum
}
