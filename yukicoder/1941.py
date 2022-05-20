#!/usr/bin/env python3

s = []
for _ in range(3):
    s.append(input())

if s == ["#.#", ".#.", "#.#"] or s == [".#.", "#.#", ".#."]:
    print('Yes')
else:
    print('No')
