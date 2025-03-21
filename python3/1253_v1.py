# 좋다
from sys import stdin


def read():
    return stdin.readline().rstrip()


n = int(read())
nums = list(map(int, read().split()))
nums.sort()


cnt = 0

for idx in range(n):
    target = nums[idx]
    ldx = 0
    rdx = n - 1

    while ldx < rdx:
        if nums[ldx] + nums[rdx] == target:
            if ldx == idx:
                ldx += 1
            elif rdx == idx:
                rdx -= 1
            else:
                cnt += 1
                break
        elif nums[ldx] + nums[rdx] < target:
            ldx += 1
        else:
            rdx -= 1

print(cnt)
