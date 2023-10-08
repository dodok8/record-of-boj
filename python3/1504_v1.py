import heapq as pq
from sys import stdin, maxsize
from typing import TypeVar, Generic

read = lambda: stdin.readline().rstrip()

T = TypeVar("T")


class Heap(Generic[T]):
    def __init__(self, data: list[T] = []) -> None:
        self.data = data.copy()
        pq.heapify(self.data)

    def push(self, item: T):
        pq.heappush(self.data, item)
        return self

    def pop(self) -> T:
        return pq.heappop(self.data)

    def extend(self, items: list[T]):
        self.data.extend(items)
        pq.heapify(self.data)
        return self

    def __len__(self) -> int:
        return len(self.data)


def get_shortest_path(start_v: int, end_v: int, edges: list[tuple[int, int]]):
    dist = [maxsize for _ in range(num_v + 1)]
    travel_pq = Heap()
    travel_pq.push((0, start_v))
    dist[start_v] = 0

    while len(travel_pq) != 0:
        curr_dist, curr_v = travel_pq.pop()
        if curr_dist > dist[curr_v]:
            continue
        for weight, adj_v in edges[curr_v]:
            if dist[adj_v] > curr_dist + weight:
                dist[adj_v] = curr_dist + weight
                travel_pq.push((curr_dist + weight, adj_v))
    print(dist)
    return dist[end_v]


num_v, num_e = map(int, read().split())
edges = [list() for _ in range(num_v + 1)]

for _ in range(num_e):
    start, end, weight = map(int, read().split())
    edges[start].append((weight, end))
    edges[end].append((weight, start))

first_mid_v, second_mid_v = map(int, read().split())

print("-----")

print(
    get_shortest_path(1, first_mid_v, edges)
    + get_shortest_path(first_mid_v, second_mid_v, edges)
    + get_shortest_path(second_mid_v, num_v, edges)
)

print(f"{first_mid_v} {second_mid_v} {num_v}")

print(
    f"{get_shortest_path(1, first_mid_v, edges)} {get_shortest_path(first_mid_v, second_mid_v, edges)} {get_shortest_path(second_mid_v, num_v, edges)}"
)

# https://www.acmicpc.net/board/view/124591
# 연결이 안된 경우에 대해서 핸들링 필요(-1 출력)
# 5 4
# 1 4 1
# 1 3 1
# 3 2 1
# 2 5 1
# 3 4
# 답이 5가 나와야 하는데 7이 나옴. 중복 경로가 원인
