# Negative People in Da House

from sys import stdin

read = lambda: stdin.readline().rstrip()

t = int(read())

for _tdx in range(t):
    m = int(read())
    ans = 0
    cur = 0

    for _idx in range(m):
        p1, p2 = map(int, read().split())
        cur = cur - p1 + p2
        ans = max(ans, cur)

    print(ans)
