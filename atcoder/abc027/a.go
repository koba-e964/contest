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
	a := nextInt()
	b := nextInt()
	c := nextInt()
	if b != c {
		fmt.Println(b + c - a)
	} else {
		fmt.Println(a)
	}
}
