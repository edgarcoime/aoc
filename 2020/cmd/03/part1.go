package main

import "fmt"

type Part1 struct {
	treeMap [][]rune
}

func NewPart1(treeMap [][]rune) *Part1 {
	return &Part1{
		treeMap: treeMap,
	}
}

func (p *Part1) Run() {
	// current progress
	x, y := 0, 0
	count := 0

	// traversing
	for y < len(p.treeMap) {
		modulus := len(p.treeMap[y])

		// Check current character
		currChar := p.treeMap[y][x]
		if currChar == TREE_CHAR {
			count++
		}

		// Increment
		x = (x + RUN) % modulus
		y += RISE
	}

	fmt.Println("Part 1 - tree count:", count)
}
