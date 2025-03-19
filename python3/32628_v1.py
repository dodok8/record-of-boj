# 두 스택

from sys import stdin, maxsize


def read():
    return stdin.readline().rstrip()


n, k = map(int, read().split())

a = list(map(int, read().split()))
b = list(map(int, read().split()))

# 누적합 배열

for idx in range(1, n):
    a[idx] = a[idx - 1] + a[idx]
    b[idx] = b[idx - 1] + b[idx]

min_won = maxsize

for xdx in range(0, k + 1):
    # 가능한 경우 수는 k + 1 개
    for kdx in range(0, xdx + 1):
        idx = n - 1 - kdx
        jdx = n - 1 - k + kdx

        won = max(a[idx], b[jdx])

        if won < min_won:
            min_won = won

print(min_won)
