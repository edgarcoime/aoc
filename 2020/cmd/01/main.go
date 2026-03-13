package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

const (
	FILEPATH = "cmd/01/input/input.txt"
	TARGET   = 2020
)

// https://adventofcode.com/2020/day/1
func main() {
	// Process input.txt into an array of numbers
	nums, err := processInput(FILEPATH)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	p1_ans, err := part1(nums, TARGET)
	if err != nil {
		fmt.Println("Part 1 error: ", err)
	}
	p2_ans, err := part2(nums, TARGET)
	if err != nil {
		fmt.Println("Part 2 error: ", err)
	}

	fmt.Println("Part 1 Answer:", p1_ans)
	fmt.Println("Part 2 Answer:", p2_ans)
}

func part2(orig []int, target int) (int, error) {
	nums := make([]int, len(orig))
	copy(nums, orig)
	sort.Ints(nums)

	for i, num := range nums {
		// If greater no way to get a sum
		if num > target {
			break
		}
		// Skip if duplicate
		if i > target && num == nums[i-1] {
			continue
		}

		// Regular 2 sum
		l, r := i+1, len(nums)-1
		for l < r {
			res := nums[i] + nums[l] + nums[r]

			if res == target {
				answer := nums[i] * nums[l] * nums[r]
				return answer, nil
			}

			if res < target {
				l++
			} else {
				r--
			}
		}

	}

	return 0, fmt.Errorf("No solution found")
}

func part1(orig []int, target int) (int, error) {
	nums := make([]int, len(orig))
	copy(nums, orig)
	sort.Ints(nums)

	// Two sum solution only for sorted array
	l, r := 0, len(nums)-1
	for l < r {
		res := nums[l] + nums[r]

		// Early break here so that else (==) doesn't evaluate
		if res == target {
			answer := nums[l] * nums[r]
			return answer, nil
		}

		if res < target {
			l++
		} else {
			r--
		}
	}

	return 0, fmt.Errorf("No solution found")
}

func processInput(filepath string) ([]int, error) {
	file, err := os.Open(filepath)
	if err != nil {
		err := fmt.Errorf("Error opening file: +%v", err)
		return nil, err
	}
	defer file.Close()

	var nums []int
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		// "  124 "
		line := scanner.Text()
		line = strings.TrimSpace(line)

		// Adding line solved problem
		// This would case 3 sum to fail because it would find 2 sum and the 3rd number
		// would be 0 then multiplying together it would multiply the 0 as well
		if line == "" {
			continue
		}

		num, err := strconv.Atoi(line)
		if err != nil {
			fmt.Println("Could not convert string to int:", err)
		}
		nums = append(nums, num)
	}

	return nums, nil
}
