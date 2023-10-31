from sys import stdin

read = lambda: stdin.readline().rstrip()
round = lambda x: int(x + 0.5)

n = int(read())
nums = list()
if n == 0:
    print(0)
else:
    for _ in range(n):
        nums.append(int(read()))
    nums.sort()
    d = round(n * 0.15)
    print(round(sum(nums[d : n - d]) / len(nums[d : n - d])))
