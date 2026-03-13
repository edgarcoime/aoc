package main

type Part1 struct {
	entries []map[string]string
}

func NewPart1(entries []map[string]string) *Part1 {
	return &Part1{
		entries: entries,
	}
}

func (p *Part1) Run() int {
	return len(p.entries)
}
