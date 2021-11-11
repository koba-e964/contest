#!/usr/bin/env python3

n = int(input())
print(n + 1 - pow(2, bin(n).count('1')))
