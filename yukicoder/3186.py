import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

# https://yukicoder.me/problems/no/3186
# The author read the solution before implementing this.


t = int(readline())
for _ in range(t):
    a, b, c = map(int, readline().split())
    opt_num, opt_den = 0, 1
    x = 0
    r = 1
    for m in range(1, 133):
        r *= a
        while r % c == 0:
            r //= c
            x += 1
        if opt_num * m < x * opt_den:
            opt_num, opt_den = x, m
    print(opt_num * b // opt_den % 998244353)
