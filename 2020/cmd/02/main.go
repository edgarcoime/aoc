// https://adventofcode.com/2020/day/2
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

const (
	FILEPATH = "cmd/02/input/input.txt"
)

type PasswordEntry struct {
	Raw      string
	Min      int
	Max      int
	Char     rune // Char that needs to be in range
	Password string
}

func main() {
	entries, err := processInput(FILEPATH)
	if err != nil {
		fmt.Println("Error processing input:", err)
	}

	p1 := NewPart1(entries)
	p1.Run()

	p2 := NewPart2(entries)
	p2.Run()
}

func processInput(filepath string) ([]*PasswordEntry, error) {
	file, err := os.Open(filepath)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var entries []*PasswordEntry
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		line = strings.TrimSpace(line)

		if line == "" {
			continue
		}

		entry, err := createPasswordEntry(line)
		if err != nil {
			fmt.Println("Error processing line:", err)
		}

		entries = append(entries, entry)
	}

	return entries, nil
}

func createPasswordEntry(line string) (*PasswordEntry, error) {
	raw := line

	parts := strings.Split(raw, " ")

	password := parts[len(parts)-1]
	character := strings.Split(parts[1], ":")[0]
	charRanges := strings.Split(parts[0], "-")

	minRange, err := strconv.Atoi(charRanges[0])
	if err != nil {
		fmt.Println("Could not convert min range to int:", err)
	}
	maxRange, err := strconv.Atoi(charRanges[1])
	if err != nil {
		fmt.Println("Could not convert max range to int:", err)
	}

	entry := &PasswordEntry{
		Raw:      raw,
		Password: password,
		Char:     rune(character[0]),
		Min:      minRange,
		Max:      maxRange,
	}
	return entry, nil
}
