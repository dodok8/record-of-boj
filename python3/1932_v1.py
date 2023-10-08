from sys import stdin

read = lambda: stdin.readline().rstrip()


def get_sum(x: int, y: int):
    global given_triangle, sum_cache
    if sum_cache[x][y] != -1:
        return sum_cache[x][y]
    sum_cache[x][y] = max(
        get_sum(x + 1, y) + given_triangle[x][y],
        get_sum(x + 1, y + 1) + given_triangle[x][y],
    )
    return sum_cache[x][y]


n = int(read())
sum_cache = [[-1 for _ in range(1, i + 1)] for i in range(1, n + 1)]
given_triangle = [list(map(int, read().split())) for _ in range(n)]

for i in range(n):
    sum_cache[n - 1][i] = given_triangle[n - 1][i]

print(get_sum(0, 0))
