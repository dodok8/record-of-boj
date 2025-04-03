# IQ Test

from sys import stdin

read = lambda: stdin.readline().rstrip()

n = int(read())
nums = list(map(int, read().split()))

if n == 1:
    print("A")
elif n == 2:
    if nums[0] == nums[1]:
        print(nums[0])
    else:
        print("A")
else:
    a = 0
    b = 0
    if nums[1] == nums[0]:
        a = 1
    else:
        a = (nums[2] - nums[1]) // (nums[1] - nums[0])
    b = nums[1] - a * nums[0]

    valid = True
    for idx in range(n - 1):
        new_val = a * nums[idx] + b
        if nums[idx + 1] != new_val:
            print("B")
            valid = False
            break

    if valid:
        print(a * nums[n - 1] + b)
