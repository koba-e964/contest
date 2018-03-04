import sys

input()
s = input()
s = list(map(len, s.split('0')))
for c in s:
    sys.stdout.write(str(c))
print()
