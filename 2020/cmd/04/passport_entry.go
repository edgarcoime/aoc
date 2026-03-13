package main

import (
	"regexp"
	"strconv"
	"strings"
	"unicode"
)

var REQUIRED_ATTRIBUTES = map[string]func(string) bool{
	"byr": func(val string) bool { return validateWithinYearRange(val, 1920, 2002) },
	"iyr": func(val string) bool { return validateWithinYearRange(val, 2010, 2020) },
	"eyr": func(val string) bool { return validateWithinYearRange(val, 2020, 2030) },
	"hgt": validateHGT,
	"hcl": validateHCL,
	"ecl": validateECL,
	"pid": validatePID,
	// "cid": validateCID, // Optional
}

var VALID_ECL = map[string]bool{
	"amb": true,
	"blu": true,
	"brn": true,
	"gry": true,
	"grn": true,
	"hzl": true,
	"oth": true,
}

var (
	HEIGHT_RE = regexp.MustCompile(`^(\d+)([a-zA-Z]+)$`)
	CM_RANGE  = []int{150, 193}
	IN_RANGE  = []int{59, 76}
)

func isAlphaNum(r rune) bool {
	return unicode.IsLetter(r) || unicode.IsDigit(r)
}

// expects an array of "key:val" strings
func parsePassportEntry(entryParts []string) map[string]string {
	passport := make(map[string]string)

	for _, part := range entryParts {
		part := strings.TrimSpace(part)
		keyVal := strings.SplitN(part, ":", 2)
		passport[keyVal[0]] = keyVal[1]
	}

	return passport
}

func validatePassportEntry(passport map[string]string) bool {
	for attr := range REQUIRED_ATTRIBUTES {
		if _, ok := passport[attr]; !ok {
			return false
		}
	}

	return true
}

func validatePassportEntryStrict(passport map[string]string) bool {
	for attr, validator := range REQUIRED_ATTRIBUTES {
		val, ok := passport[attr]
		if !ok {
			return false
		}
		if !validator(val) {
			return false
		}
	}
	return true
}

func validateWithinYearRange(val string, minVal int, maxVal int) bool {
	year, err := strconv.Atoi(val)
	if err != nil {
		return false
	}

	if minVal <= year && year <= maxVal {
		return true
	} else {
		return false
	}
}

func validateHGT(val string) bool {
	// original, num, "cm" or "in"
	m := HEIGHT_RE.FindStringSubmatch(val)

	if len(m) != 3 {
		return false
	}

	// attempt to convert height
	hNum, err := strconv.Atoi(m[1])
	if err != nil {
		return false
	}

	if m[2] == "cm" && CM_RANGE[0] <= hNum && hNum <= CM_RANGE[1] {
		return true
	}

	if m[2] == "in" && IN_RANGE[0] <= hNum && hNum <= IN_RANGE[1] {
		return true
	}

	return false
}

func validateHCL(val string) bool {
	if val[0] != '#' {
		return false
	}

	// #XXXXXX
	if len(val) != (6 + 1) {
		return false
	}

	for i := 1; i < len(val); i++ {
		if !isAlphaNum(rune(val[i])) {
			return false
		}
	}

	return true
}

func validateECL(val string) bool {
	if _, ok := VALID_ECL[val]; !ok {
		return false
	}

	return true
}

func validatePID(val string) bool {
	if len(val) != 9 {
		return false
	}

	for _, r := range val {
		ok := unicode.IsDigit(r)
		if !ok {
			return false
		}
	}

	return true
}
