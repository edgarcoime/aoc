package main

import "fmt"

type Part2 struct {
	entries []*PasswordEntry
}

func NewPart2(entries []*PasswordEntry) *Part2 {
	return &Part2{
		entries: entries,
	}
}

func (p *Part2) Run() {
	count := 0
	for _, entry := range p.entries {
		// fmt.Println("Processing entry:", entry.Raw)

		// If out of range of max then invalid
		if len(entry.Password) < entry.Min {
			continue
		}

		charMin, charMax := entry.Password[entry.Min-1], entry.Password[entry.Max-1]
		// if both are same then invalid
		if charMin == byte(entry.Char) && charMax == byte(entry.Char) {
			continue
		}

		// If both not then invalid
		if charMin != byte(entry.Char) && charMax != byte(entry.Char) {
			continue
		}

		count++
	}

	fmt.Println("Part 2:", count)
}
