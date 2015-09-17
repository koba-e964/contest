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
	n := nextInt()
	a := make([]int, n)
	sum := 0
	for i := 0; i < n; i += 1 {
		a[i] = nextInt()
		sum += a[i]
	}
	if sum % n != 0 {
		fmt.Println(-1)
		return
	}
	avr := sum / n
	cnt := 0
	cur := 0
	for i := 0; i < n - 1; i += 1 {
		cur += a[i]
		if cur != avr * (i + 1) {
			cnt += 1
		}
	}
	fmt.Println(cnt)
}
