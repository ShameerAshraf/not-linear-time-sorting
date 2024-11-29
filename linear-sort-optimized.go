package main

import (
    "fmt"
    "time"
)

func linearSortGoRoutine(num int, c chan<- int) {
    time.Sleep(500 * time.Duration(num) * time.Millisecond)
    c <- num
}

func main() {

    array:= [...]int{8, 6, 10, 9, 5, 2, 7, 1, 4, 3}

    fmt.Println("array:", array)

    messages := make(chan int)

    for i := 0; i < len(array); i++ {
        go linearSortGoRoutine(array[i], messages)
    }

    res := []int{}

    for i:= 0; i < len(array); i++ {
        num := <-messages
        fmt.Println("Got:", num)
        res = append(res, num)
    }

    fmt.Println(res)

    fmt.Println("hello world")
}