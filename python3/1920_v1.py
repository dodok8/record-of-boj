from sys import stdin

read = lambda: stdin.readline().rstrip()

n = int(read())
nums = set(map(int, read().split()))
m = int(read())
for i in map(int, read().split()):
    print(1 if i in nums else 0)
