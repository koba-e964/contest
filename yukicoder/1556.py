#!/usr/bin/env python3

a, b = map(int, input().split())
if a + b == 6 and a * b == 8:
    print('Yes')
    exit()

print('Yes' if a == b else 'No')
