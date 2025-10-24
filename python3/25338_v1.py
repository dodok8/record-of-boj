# 바지 구매

from sys import stdin

read = lambda: stdin.readline().rstrip()

a, b, c, d = map(int, read().split())


def is_fit(length, height):
    pants = max(a * (height - b) ** 2 + c, d)
    if pants != length or b > height:
        return False
    return True


n = int(read())
ans = 0
for _ in range(n):
    u, v = map(int, read().split())
    ans += 1 if is_fit(u, v) else 0

print(ans)
