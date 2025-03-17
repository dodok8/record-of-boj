import sys
from collections import deque


def read():
    return sys.stdin.readline().rstrip()


def make_wall(cnt):
    if cnt == 3:
        spread()
        return
    for idx in range(n):
        for jdx in range(m):
            if graph[idx][jdx] == 0:
                graph[idx][jdx] = 1
                make_wall(cnt + 1)
                graph[idx][jdx] = 0


def spread():
    tq = deque()
    visited = [[0 for jdx in range(m)] for idx in range(n)]
    dx = [0, 0, 1, -1]
    dy = [1, -1, 0, 0]

    for point in v_points:
        tq.append(point)
        visited[point[0]][point[1]] = 1

    while tq:
        x, y = tq.popleft()

        for idx in range(4):
            nx = x + dx[idx]
            ny = y + dy[idx]

            if nx < 0 or nx >= n or ny < 0 or ny >= m:
                continue
            if visited[nx][ny]:
                continue
            if graph[nx][ny] == 1:
                continue

            tq.append((nx, ny))
            visited[nx][ny] = 1

    cnt = 0
    for idx in range(n):
        for jdx in range(m):
            if graph[idx][jdx] == 0 and not visited[idx][jdx]:
                cnt += 1
    global max_cnt
    max_cnt = max(cnt, max_cnt)


n, m = map(int, read().split())

graph = list()

for idx in range(n):
    graph.append(list(map(int, read().split())))

v_points = list()
max_cnt = 0

for idx in range(n):
    for jdx in range(m):
        if graph[idx][jdx] == 2:
            v_points.append((idx, jdx))

make_wall(0)

print(max_cnt)
