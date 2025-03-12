# 차이를 최대로

from sys import stdin

read = lambda: stdin.readline().rstrip()


def calc_diff_sum(arr):
    sum = 0
    for idx in range(len(arr) - 1):
        sum += abs(arr[idx] - arr[idx + 1])
    return sum


n = int(read())
nums = list(map(int, read().split()))


visited = [False for _ in range(n)]
max_sum = 0
arr = list()


def back_track(depth: int):
    global max_sum, n, arr
    if depth == n:
        max_sum = max(max_sum, calc_diff_sum(arr))
    for idx in range(n):
        if not visited[idx]:
            visited[idx] = True
            arr.append(nums[idx])
            back_track(depth + 1)
            visited[idx] = False
            arr.pop()


back_track(0)

print(max_sum)
