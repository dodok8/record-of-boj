from collections import deque
from sys import stdin, maxsize

read = lambda: stdin.readline().rstrip()

dist = [maxsize for _ in range(100001)]

start_v, end_v = map(int, read().split())

travel_dq = deque()
travel_dq.append((0, start_v))
dist[start_v] = 0

while len(travel_dq) != 0:
    curr_dist, curr_v = travel_dq.popleft()
    if curr_v > 100000 or curr_v < 0:
        continue
    if curr_dist > dist[curr_v]:
        continue
    for weight, adj_v in [(1, curr_v + 1), (1, curr_v - 1), (0, curr_v * 2)]:
        if adj_v > 100000 or adj_v < 0:
            continue
        if dist[adj_v] > curr_dist + weight:
            dist[adj_v] = curr_dist + weight
            if weight == 1:
                travel_dq.append((dist[adj_v], adj_v))
            else:
                travel_dq.appendleft((dist[adj_v], adj_v))

print(dist[end_v])
