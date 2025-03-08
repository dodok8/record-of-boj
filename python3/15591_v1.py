# MooTube (Silver)

from sys import stdin
from collections import deque
import sys

read = lambda: stdin.readline().rstrip()

n, q = map(int, read().split())
graphs: list[list[tuple[int, int]]] = [list() for _ in range(n + 1)]

for idx in range(n - 1):
    start, end, weight = map(int, read().split())
    graphs[start].append((end, weight))
    graphs[end].append((start, weight))

for jdx in range(q):
    ans = 0
    visited = [False for _ in range(n + 1)]
    k, v = map(int, read().split())

    visited[v] = True

    travel_q: deque[tuple[int, int]] = deque()
    travel_q.append((v, sys.maxsize))

    while travel_q:
        curr_v, weight = travel_q.popleft()

        for next_v, next_weight in graphs[curr_v]:
            next_weight = min(weight, next_weight)
            if next_weight >= k and not visited[next_v]:
                ans += 1
                visited[next_v] = True
                travel_q.append((next_v, next_weight))

    print(ans)
