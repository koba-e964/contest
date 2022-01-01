#!/usr/bin/env python3

p, q = map(float, input().split())
print(100 * (p * q) / (p * q + (100 - p) * (100 - q)))
