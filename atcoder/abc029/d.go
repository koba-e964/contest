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

func onec(n int64) int64 {
	var c int64 = 0
	for n > 0 {
		if n % 10 == 1 { c += 1 }
		n /= 10
	}
	return c
}

func onec2(n int64) int64 {
	if n < 0 {
		return 0
	}
	var cnt int64 = 0
	r := n % 10
	if r >= 1 {
		cnt += 1
	}
	cnt += onec(n / 10) * (r + 1)
	cnt += onec2(n / 10 - 1) * 10 + n / 10
	return cnt
}



func main() {
	sc.Split(bufio.ScanWords)
	n := nextInt()
	fmt.Println(onec2(int64(n)))
}
