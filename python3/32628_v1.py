# 두 스택

from sys import stdin, maxsize


def read():
    return stdin.readline().rstrip()


n, k = map(int, read().split())

a = list(map(int, read().split()))
b = list(map(int, read().split()))

# 누적합 배열
a_prefix = [0]
b_prefix = [0]

for i in range(n):
    a_prefix.append(a_prefix[-1] + a[i])
    b_prefix.append(b_prefix[-1] + b[i])

min_won = maxsize

for kdx in range(k + 1):
    if kdx > n or (k - kdx) > n:
        continue

    idx = n - kdx
    jdx = n - k + kdx

    won = max(a_prefix[idx], b_prefix[jdx])

    if won < min_won:
        min_won = won

print(min_won)
