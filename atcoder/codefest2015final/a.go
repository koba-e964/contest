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
     sc.Scan(); a := sc.Text()
     sc.Scan(); b := sc.Text()
     sc.Scan(); c := sc.Text()
     if len(a) == 5 && len(b) == 7 && len(c) == 5 {
        fmt.Println("valid")
     } else {
	fmt.Println("invalid")
    }  
}		
