#!/usr/bin/env python3

a = input()
s = input()
for c in s:
    print(c if c > '9' else a[ord(c) - 48], end='')
print()
