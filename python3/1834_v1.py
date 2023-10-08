from sys import stdin

read = lambda: stdin.readline().rstrip()

n = int(read())
print(n * (n - 1) * (n + 1) // 2)
