# 벽 부수고 이동하기

from sys import stdin
from collections import deque

read = lambda: stdin.readline().rstrip()

n, m = map(int, read().split())

graphs = list()

for _idx in range(n):
    graphs.append(list(map(int, read())))

dists = [[[0 for _kdx in range(2)] for _jdx in range(m)] for _idx in range(n)]
dists[0][0][0] = 1

travel_q = deque()
travel_q.append((0, 0, 0))

dx = [-1, 1, 0, 0]
dy = [0, 0, -1, 1]

while travel_q:
    x, y, breakable = travel_q.popleft()

    if x == n - 1 and y == m - 1:
        print(dists[x][y][breakable])
        break

    for idx in range(4):
        nx = x + dx[idx]
        ny = y + dy[idx]

        if nx < 0 or nx >= n or ny < 0 or ny >= m:
            continue

        if graphs[nx][ny] == 1 and breakable == 0:
            dists[nx][ny][1] = dists[x][y][0] + 1
            travel_q.append((nx, ny, 1))
        elif graphs[nx][ny] == 0 and dists[nx][ny][breakable] == 0:
            dists[nx][ny][breakable] = dists[x][y][breakable] + 1
            travel_q.append((nx, ny, breakable))

if dists[n - 1][m - 1][0] == 0 and dists[n - 1][m - 1][1] == 0:
    print(-1)
