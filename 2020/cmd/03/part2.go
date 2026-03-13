package main

import "fmt"

type Part2 struct {
	treeMap [][]rune
	slopes  [][]int
}

func NewPart2(treeMap [][]rune, slopes [][]int) *Part2 {
	return &Part2{
		treeMap: treeMap,
		slopes:  slopes, // x, y
	}
}

func (p *Part2) Run() {
	combined := 1
	for _, slope := range p.slopes {
		// {x,y}
		combined *= p.slopeRun(slope[0], slope[1])
	}

	fmt.Println("Part 2 - combined:", combined)
}

func (p *Part2) slopeRun(run int, rise int) int {
	x, y := 0, 0
	count := 0

	// traverse
	yLen := len(p.treeMap)
	for y < yLen {
		modulus := len(p.treeMap[y])

		// Check current character
		currChar := p.treeMap[y][x]
		if currChar == TREE_CHAR {
			count++
		}

		// Increment
		x = (x + run) % modulus
		y += rise
	}

	return count
}
