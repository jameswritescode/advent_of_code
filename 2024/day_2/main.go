package main

import (
	"math"
	"os"
	"strconv"
	"strings"
)

type Result struct {
	Index   int
	Report  []int
	Success bool
}

type Reports [][]int

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
	part2(reports)
}

func part1(reports Reports) {
	safe := 0

	check(reports, func(result Result) {
		if result.Success {
			safe++
		}
	})

	println(safe)
}

func part2(reports Reports) {
	safe := 0

	check(reports, func(result Result) {
		if result.Success {
			safe++
		} else {
			for i := 0; i < len(result.Report); i++ {
				alternativeReport := append([]int{}, result.Report[:i]...)
				alternativeReport = append(alternativeReport, result.Report[i+1:]...)
				success := false

				check(Reports{alternativeReport}, func(result Result) {
					if result.Success {
						success = true
					}
				})

				if success {
					safe++
					break
				}
			}
		}
	})

	println(safe)
}

func convert(r []string) []int {
	reports := make([]int, 0, len(r))

	for _, s := range r {
		n, _ := strconv.Atoi(s)
		reports = append(reports, n)
	}

	return reports
}

func check(reports Reports, cb func(result Result)) {
	for _, report := range reports {
		direction := report[0] > report[1]

		result := Result{
			Report:  report,
			Success: true,
		}

		for i := 0; i < len(report)-1; i++ {
			a, b := report[i], report[i+1]

			if a == b || a > b != direction || math.Abs(float64(a-b)) > 3 {
				result.Success = false
				result.Index = i

				break
			}
		}

		cb(result)
	}
}
