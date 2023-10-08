from sys import stdin

read = lambda: stdin.readline().rstrip()

N = int(read())
nums = list()
for _ in range(N):
    nums.append(int(read()))
nums.sort()
for num in nums:
    print(num)
