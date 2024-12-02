package main

import (
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	data, err := os.ReadFile(os.Args[1])

	if err != nil {
		panic(err)
	}

	lines := strings.Split(strings.TrimSuffix(string(data), "\n"), "\n")
	reports := make([][]int, 0, len(lines))

	for _, line := range lines {
		ns := strings.Fields(line)
		reports = append(reports, convert(ns))
	}

	part1(reports)
}

func convert(r []string) []int {
	reports := make([]int, 0, len(r))

	for _, s := range r {
		n, _ := strconv.Atoi(s)
		reports = append(reports, n)
	}

	return reports
}

func dir(a, b int) int {
	if a > b {
		return 1
	}

	return 2
}

func part1(reports [][]int) {
	safeReports := 0

	for _, report := range reports {
		safeReport := true
		firstDir := dir(report[0], report[1])

		for i := 0; i < len(report)-1; i++ {
			a, b := report[i], report[i+1]

			if dir(a, b) != firstDir || !(a != b && math.Abs(float64(a-b)) <= 3) {
				safeReport = false
				break
			}
		}

		if safeReport {
			safeReports++
		}
	}

	println(safeReports)
}
