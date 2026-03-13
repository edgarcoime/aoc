package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

const (
	FILEPATH = "cmd/03/input/input.txt"

	// Part 1
	RISE       = 1
	RUN        = 3
	TREE_CHAR  = '#'
	BLANK_CHAR = '.'
)

func main() {
	// part 2
	slopes := [][]int{
		{1, 1},
		{3, 1},
		{5, 1},
		{7, 1},
		{1, 2},
	}

	treeMap, err := processInput(FILEPATH)
	if err != nil {
		fmt.Println("Error processing input:", err)
		os.Exit(1)
	}

	p1 := NewPart1(treeMap)
	p2 := NewPart2(treeMap, slopes)

	p1.Run()
	p2.Run()
}

func processInput(filepath string) ([][]rune, error) {
	file, err := os.Open(filepath)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var treeMap [][]rune

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		line = strings.TrimSpace(line)

		if line == "" {
			continue
		}

		var chars []rune
		for _, char := range line {
			chars = append(chars, char)
		}
		treeMap = append(treeMap, chars)
	}

	return treeMap, nil
}
