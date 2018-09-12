# Proth prime generator
# Input: k
# Generates primes p <= 2^31 - 1 where 2^k | p - 1


# Reference: http://kirika-comp.hatenablog.com/entry/2018/03/12/210446 (p. 30)
def generator(p):
    fac = []
    pp = 2
    v = p - 1
    while v >= pp * pp:
        e = 0
        while v % pp == 0:
            e += 1
            v //= pp
        if e > 0:
            fac.append(pp)
        pp += 1
    if v > 1:
        fac.append(v)

    g = 2
    while g < p:
        if pow(g, p - 1, p) != 1:
            print(str(p) + ' is not a prime')
            return None
        ok = True
        for pp in fac:
            if pow(g, (p - 1) // pp, p) == 1:
                ok = False
                break
        if ok:
            return g
        g += 1


k = int(input())

for i in range(2 ** (31 - k) - 1, 0, -1):
    p = i * 2 ** k + 1
    if pow(2, p - 1, p) == 1 and pow(7, p - 1, p) == 1 and \
       pow(61, p - 1, p) == 1:
        print("%d %d" % (p, generator(p)))
