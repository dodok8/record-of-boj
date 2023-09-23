from sys import stdin

read = lambda: stdin.readline().rstrip()

for _ in range(int(read())):
    a, b, x = map(int, read().split())
    print(a * (x - 1) + b)
