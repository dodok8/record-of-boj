# 가희와 방어율 무시

from sys import stdin

read = lambda: stdin.readline().rstrip()

a, b = map(int, read().split())

if a - 0.01 * a * b >= 100:
    print(0)
else:
    print(1)
