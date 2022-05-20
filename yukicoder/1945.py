#!/usr/bin/env python3

n = int(input())
a = list(map(int, input().split()))
print('Second' if n % 2 == 0 else 'First')
