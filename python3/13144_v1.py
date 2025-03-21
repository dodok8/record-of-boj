# List of Unique Numbers

from sys import stdin


def read():
    return stdin.readline().rstrip()


n = int(input())
nums = list(map(int, read().split()))

ldx = 0
rdx = 0

hash_set = set()
ans = 0
while ldx < n and rdx < n:
    if nums[rdx] not in hash_set:
        hash_set.add(nums[rdx])
        rdx += 1
        ans += rdx - ldx
    else:
        hash_set.remove(nums[ldx])
        ldx += 1

print(ans)
