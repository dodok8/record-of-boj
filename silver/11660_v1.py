from sys import stdin

read = lambda: stdin.readline().rstrip()


n, test_count = map(int, read().split())
board = [list(map(int, read().split())) for _ in range(n)]

sum_cache = [[-1 for _ in range(n)] for __ in range(n)]
sum_cache[0][0] = board[0][0]
for idx in range(n):
    for jdx in range(n):
        sum_cache[idx][jdx] = (
            (sum_cache[idx - 1][jdx] if idx > 0 else 0)
            + (sum_cache[idx][jdx - 1] if jdx > 0 else 0)
            - (sum_cache[idx - 1][jdx - 1] if idx > 0 and jdx > 0 else 0)
            + board[idx][jdx]
        )

for _test in range(test_count):
    x1, y1, x2, y2 = map(lambda x: int(x) - 1, read().split())
    # print(sum_cache[x2][y2])
    # print(f"-{sum_cache[x2][y1 - 1] if y1 > 0 else 0}")
    # print(f"-{sum_cache[x1 - 1][y2] if x1 > 0 else 0}")
    # print(f"+{sum_cache[x1 - 1][y1 - 1] if x1 > 0 and y1 > 0 else 0}")
    print(
        sum_cache[x2][y2]
        - (sum_cache[x2][y1 - 1] if y1 > 0 else 0)
        - (sum_cache[x1 - 1][y2] if x1 > 0 else 0)
        + (sum_cache[x1 - 1][y1 - 1] if x1 > 0 and y1 > 0 else 0)
    )


# 시간 초과 뜸
