#!/usr/bin/env python3

a, b = map(int, input().split())

q = a // b
a -= q * b
dec = ''
for _ in range(50):
    a *= 10
    x = a // b
    dec += chr(48 + x)
    a %= b
print('{}.{}'.format(q, dec))
