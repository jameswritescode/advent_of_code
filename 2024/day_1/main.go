package main

import (
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
	data, err := os.ReadFile(os.Args[1])

	if err != nil {
		panic(err)
	}

	lines := strings.Split(strings.TrimSuffix(string(data), "\n"), "\n")

	left := make([]int, 0, len(lines))
	right := make([]int, 0, len(lines))

	for _, line := range lines {
		fields := strings.Fields(line)

		left = append(left, convert(fields[0]))
		right = append(right, convert(fields[1]))
	}

	part1(left, right)
	part2(left, right)

}

func convert(s string) int {
	i, _ := strconv.Atoi(s)
	return i
}

func part1(left, right []int) {
	left = slices.Clone(left)
	right = slices.Clone(right)

	slices.Sort(left)
	slices.Sort(right)

	distance := 0

	for i, l := range left {
		r := right[i]

		if l > r {
			l, r = r, l
		}

		distance += r - l
	}

	println(distance)
}

func part2(left, right []int) {
	occurrences := make(map[int]int)

	for _, n := range right {
		occurrences[n]++
	}

	similarity := 0

	for _, n := range left {
		similarity += n * occurrences[n]
	}

	println(similarity)
}
