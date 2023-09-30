# 데익스트라로 풀어보기
# 데익스트라의 핵심은 우선순위 큐이다. 그리고 이건 파이썬에서 제공한다.

from queue import PriorityQueue
from sys import stdin, maxsize

read = lambda: stdin.readline().rstrip()

num_v, num_e = map(int, read().split())
start_v = int(read())
edges = [list() for _ in range(num_v + 1)]

for _ in range(num_e):
    start, end, weight = map(int, read().split())
    edges[start].append((weight, end))

dist = [maxsize for _ in range(num_v + 1)]

travel_pq = PriorityQueue()
travel_pq.put((0, start_v))
dist[start_v] = 0

while not travel_pq.empty():
    curr_dist, curr_v = travel_pq.get()
    if curr_dist > dist[curr_v]:
        continue
    for weight, end_v in edges[curr_v]:
        if dist[end_v] > curr_dist + weight:
            dist[end_v] = curr_dist + weight
            travel_pq.put((curr_dist + weight, end_v))

for idx, final_dist in enumerate(dist):
    if idx == 0:
        continue
    if final_dist == maxsize:
        print("INF")
    else:
        print(final_dist)
