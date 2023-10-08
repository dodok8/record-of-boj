from sys import stdin

read = lambda: stdin.readline().rstrip()

n, x = map(int, read().split())
sum = sum(map(int, read().split()))

if sum % x == 0:
    print(1)
else:
    print(0)
