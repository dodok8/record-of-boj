# 테트로미노

from sys import stdin

read = lambda: stdin.readline().rstrip()

max_sum = 0

tetrominos = [
    # I 테트로미노 (2가지)
    [(0, 0), (0, 1), (0, 2), (0, 3)],
    [(0, 0), (1, 0), (2, 0), (3, 0)],
    # O 테트로미노 (1가지)
    [(0, 0), (0, 1), (1, 0), (1, 1)],
    # L 테트로미노 (8가지: 회전 + 대칭)
    [(0, 0), (1, 0), (2, 0), (2, 1)],
    [(0, 0), (0, 1), (0, 2), (1, 0)],
    [(0, 0), (0, 1), (1, 1), (1, 2)],
    [(0, 0), (0, 1), (1, 1), (2, 1)],
    [(0, 0), (1, 0), (1, -1), (1, -2)],
    [(0, 0), (1, 0), (2, 0), (2, -1)],
    [(0, 0), (1, 0), (1, 1), (1, 2)],
    [(0, 0), (0, 1), (1, 0), (2, 0)],
    [(0, 0), (0, 1), (0, 2), (1, 2)],
    # ㄹ 테트로미노 (4가지: 회전 + 대칭)
    [(0, 0), (1, 0), (1, 1), (2, 1)],
    [(0, 0), (0, 1), (1, -1), (1, 0)],
    [(0, 0), (1, 0), (1, -1), (2, -1)],
    [(0, 0), (0, 1), (1, 1), (1, 2)],
    # T 테트로미노 (4가지: 회전)
    [(0, 0), (1, 0), (2, 0), (1, 1)],
    [(0, 0), (1, 0), (1, 1), (1, -1)],
    [(0, 0), (1, 0), (2, 0), (1, -1)],
    [(0, 0), (0, 1), (0, 2), (1, 1)],
]

n, m = map(int, read().split())

graph = [list(map(int, read().split())) for _ in range(n)]

max_sum = 0

for cx in range(n):
    for cy in range(m):
        for tetronomi in tetrominos:
            is_valid = True
            sum = 0
            for dx, dy in tetronomi:
                nx = cx + dx
                ny = cy + dy
                if nx < 0 or nx >= n or ny < 0 or ny >= m:
                    is_valid = False
                    break
                sum += graph[nx][ny]
            if is_valid:
                max_sum = max(max_sum, sum)

print(max_sum)
