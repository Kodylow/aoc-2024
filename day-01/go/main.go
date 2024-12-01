package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

// calculateSimilarityScore computes the similarity score between two lists of numbers.
// Time Complexity: O(n + m) where n is length of left list, m is length of right list
// Space Complexity: O(m) for the frequency map
func calculateSimilarityScore(filename string) (int64, error) {
	start := time.Now()

	// Open file with error handling
	file, err := os.Open(filename)
	if err != nil {
		return 0, fmt.Errorf("error opening file: %v", err)
	}
	defer file.Close()

	// Initialize frequency map for right-side numbers with capacity hint
	rightFreq := make(map[int]int, 1000)

	// Use larger buffer size for potentially better IO performance
	scanner := bufio.NewScanner(file)
	buf := make([]byte, 64*1024)
	scanner.Buffer(buf, 64*1024)

	parseStart := time.Now()

	// First pass: Build frequency map of right-side numbers
	for scanner.Scan() {
		nums := strings.Fields(scanner.Text())
		if len(nums) != 2 {
			continue
		}

		rightNum, err := strconv.Atoi(nums[1])
		if err != nil {
			continue
		}
		rightFreq[rightNum]++
	}

	fmt.Printf("Parsing first pass completed in %v\n", time.Since(parseStart))

	if err := scanner.Err(); err != nil {
		return 0, fmt.Errorf("error reading file: %v", err)
	}

	// Reset file for second pass
	calcStart := time.Now()
	file.Seek(0, 0)
	scanner = bufio.NewScanner(file)
	scanner.Buffer(buf, 64*1024)

	// Second pass: Calculate similarity score
	var totalScore int64

	for scanner.Scan() {
		nums := strings.Fields(scanner.Text())
		if len(nums) != 2 {
			continue
		}

		leftNum, err := strconv.Atoi(nums[0])
		if err != nil {
			continue
		}

		// Multiply left number by its frequency in right list
		totalScore += int64(leftNum) * int64(rightFreq[leftNum])
	}

	fmt.Printf("Calculation completed in %v\n", time.Since(calcStart))
	fmt.Printf("Total time: %v\n", time.Since(start))

	return totalScore, nil
}

func main() {
	totalStart := time.Now()

	score, err := calculateSimilarityScore("../puzzle_input.txt")
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}

	fmt.Printf("Program completed in %v\n", time.Since(totalStart))
	fmt.Printf("Similarity Score: %d\n", score)
}
