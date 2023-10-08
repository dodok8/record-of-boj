from sys import stdin

read = lambda: stdin.readline().rstrip()

N, K = map(int, read().split())
values = [
    0,
]
weights = [
    0,
]

for _ in range(N):
    w, v = map(int, read().split())
    weights.append(w)
    values.append(v)

sum_cache = [[-1 for _ in range(K + 1)] for __ in range(N + 1)]

for idx in range(K + 1):
    sum_cache[0][idx] = 0
for jdx in range(N + 1):
    sum_cache[jdx][0] = 0


def get_sum(n: int, w: int) -> int:
    """현재 가방 제한이 w일 때 n번째 아이템까지 채운 최적의 가치합"""
    if sum_cache[n][w] != -1:
        return sum_cache[n][w]
    if w - weights[n] >= 0:
        sum_cache[n][w] = max(
            get_sum(n - 1, w), get_sum(n - 1, w - weights[n]) + values[n]
        )
    else:
        sum_cache[n][w] = get_sum(n - 1, w)
    return sum_cache[n][w]


print(get_sum(N, K))
