package main

import (
    "fmt"
    "math"
    "os"
    "strconv"
    "strings"
)

func main() {
    input := input()

    fuelCosts := []int{}
    maxPosition := max(input)
    for position := 0; position <= maxPosition; position++ {
        fuel := 0
        for _, otherPosition := range input {
            fuel += int(math.Abs(float64(position - otherPosition)))
        }
        fmt.Println("Cost:", position, fuel)
        fuelCosts = append(fuelCosts, fuel)
    }

    fmt.Println("Answer:", min(fuelCosts))
}

func min(items []int) int {
    min := items[0]
    for _, value := range items {
        if min > value {
            min = value
        }
    }
    return min
}

func max(items []int) int {
    max := 0
    for _, value := range items {
        if max < value {
            max = value
        }
    }
    return max
}

func input() []int {
    file, err := os.ReadFile("input.txt")
    if err != nil {
        panic(err)
    }

    result := []int{}
    for _, value := range strings.Split(strings.TrimSpace(string(file)), ",") {
        number, _ := strconv.Atoi(value)
        result = append(result, number)
    }

    return result
}
