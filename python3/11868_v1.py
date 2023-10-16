from sys import stdin
from functools import reduce

read = lambda: stdin.readline().rstrip()

N = int(read())
num_stones: list[int] = list(map(int, stdin.readline().split()))
ans = reduce(lambda acc, curr: acc ^ curr, num_stones)
if ans == 0:
    print("cubelover")
else:
    print("koosaga")
