# 평범한 배낭
# 메모이제이션 없이 구현하면서 그 형태를 알아보자

from sys import stdin


def read():
    return stdin.readline().rstrip()


N, K = map(int, read().split())

dp = [[0 for __ in range(K + 1)] for _ in range(N + 1)]

weights = [
    0,
]
values = [
    0,
]

for idx in range(N):
    w, v = map(int, read().split())
    weights.append(w)
    values.append(v)

for item in range(1, N + 1):
    for curr_weight in range(1, K + 1):
        w = weights[item]
        v = values[item]

        if curr_weight - w < 0:
            dp[item][curr_weight] = dp[item - 1][curr_weight]
        else:
            dp[item][curr_weight] = max(
                dp[item - 1][curr_weight - w] + v, dp[item - 1][curr_weight]
            )

print(dp[N][K])
