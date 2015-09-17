package main

import (
    "bufio"
    "fmt"
    "os"
    "strconv"
)

var _ = fmt.Scanf
var sc = bufio.NewScanner(os.Stdin)

func nextInt() int {
    sc.Scan()
    i, e := strconv.Atoi(sc.Text())
    if e != nil {
        panic(e)
    }
    return i
}

func main() {
    sc.Split(bufio.ScanWords)
}
