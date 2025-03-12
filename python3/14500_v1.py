# 테트로미노

from sys import stdin

read = lambda: stdin.readline().rstrip()

max_sum = 0

tetrominos = [
    # 1자 형 바
    [(0, 0), (0, 1), (0, 2), (0, 3)],
    [(0, 0), (1, 0), (2, 0), (3, 0)],
    # 네모
    [(0, 0), (0, 1), (1, 0), (2, 2)],
    # L 자바
    [(0, 0), (1, 0), (2, 0), (2, 1)],
    [(0, 0), (0, 1), (0, 2), (1, 0)],
    [(0, 0), (0, 1), (1, 1), (2, 1)],
    [(0, 0), (1, 0), (1, -1), (1, -2)],
    [(0, 0), (1, 0), (2, 0), (2 , -1)],
    [(0, 0), (1, 0), (1, 1), (1, 2)],
    [(0, 0), (0, 1), (1, 0), (0, 2)],
    [(0, 0), (0, 1), (0, 2), (1, 2)],
    # ㄹ 자 형, 대칭 인 것 제외
    [(0, 0), (1, 0), (1, 1), (2, 1)],
    [(0, 0), (0, -1), (-1, -1), (-2, -1)],
    [(0, 0), (1, 0), (1, -1), (2, -1)],
    [(0, 0), (0, 1), (1, 1), (1, 2)],
    # ㅜ 자형, 대칭 제외
    [(0, 0), (0, 1), (0, 2), (1, 1)],
    [(0, 0), (1, 0), (2, 0), (1, -1)],
    [(0, 0), (0, -1), (-1, -1), (0, -2)],
    [(0, 0), (-1, 0), (-1, 1), (-2, 0)],
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
