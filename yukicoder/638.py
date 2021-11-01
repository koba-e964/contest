#!/usr/bin/env python3

n = int(input())

a = 3
while a < n:
    b = n - a
    if a != (a & -a) and b != (b & -b):
        print('{} {}'.format(a, b))
        exit()
    a += 1

print('-1')
