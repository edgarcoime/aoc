package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strings"
)

const (
	INPUTPATH = "cmd/04/input/input.txt"
	TESTPATH  = "cmd/04/input/test.txt"
)

func main() {
	entries1, err := processInput(INPUTPATH, validatePassportEntry)
	if err != nil {
		fmt.Println("Simple validator error:", err)
		os.Exit(1)
	}

	p1 := NewPart1(entries1)
	res1 := p1.Run()
	fmt.Println("Part 1:", res1)

	entries2, err := processInput(INPUTPATH, validatePassportEntryStrict)
	if err != nil {
		fmt.Println("Strict validator error:", err)
		os.Exit(1)
	}

	p2 := NewPart1(entries2)
	res2 := p2.Run()
	fmt.Println("Part 2:", res2)
}

func processInput(filepath string, validator func(map[string]string) bool) ([]map[string]string, error) {
	// open file
	file, err := os.Open(filepath)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var entries []map[string]string

	var entryParts []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		line = strings.TrimSpace(line)

		// If empty line then that means end of entry
		if line == "" {
			passport := parsePassportEntry(entryParts)
			valid := validator(passport)
			if valid {
				entries = append(entries, passport)
			}

			entryParts = make([]string, 0)
			continue
		}

		// otherwise have to append parts
		lineParts := strings.Split(line, " ")
		entryParts = slices.Concat(entryParts, lineParts)
	}

	// Get last entry of page
	if len(entryParts) >= 0 {
		passport := parsePassportEntry(entryParts)
		valid := validator(passport)
		if valid {
			entries = append(entries, passport)
		}
	}

	return entries, nil
}
