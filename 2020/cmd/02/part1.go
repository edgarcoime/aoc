package main

import "fmt"

type Part1 struct {
	entries []*PasswordEntry
}

func NewPart1(entries []*PasswordEntry) *Part1 {
	return &Part1{
		entries: entries,
	}
}

func (p *Part1) Run() {
	count := 0
	for _, entry := range p.entries {

		validCharCount := 0
		for _, char := range entry.Password {
			if char == entry.Char {
				validCharCount++
			}
		}

		if entry.Min <= validCharCount && validCharCount <= entry.Max {
			count++
		}
	}

	fmt.Println("Part 1:", count)
}
