# 웜홀

import heapq as pq
from sys import stdin, maxsize
from typing import TypeVar, Generic, TypeAlias

read = lambda: stdin.readline().rstrip()

T = TypeVar("T")
Weight: TypeAlias = int
Vertex: TypeAlias = int
Edge: TypeAlias = tuple[Weight, Vertex]


class NotConnectedGraphException(Exception):
    pass


class MinusCycleException(Exception):
    pass


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


def check_no_minus_cycle_bellman_ford(
    start_v: Vertex, edges: list[list[Edge]]
):
    num_v = len(edges)
    upper_bound: list[Weight] = [maxsize for _ in range(num_v)]
    upper_bound[start_v] = 0
    is_updated = False
    for count in range(num_v):
        is_updated = False
        for curr_v in range(0, num_v):
            for weight, adj_v in edges[curr_v]:
                if upper_bound[adj_v] > upper_bound[curr_v] + weight:
                    if (
                        upper_bound[adj_v] == maxsize
                        and upper_bound[curr_v] == maxsize
                    ):
                        continue
                    upper_bound[adj_v] = upper_bound[curr_v] + weight
                    is_updated = True
        if not is_updated:
            break
        if is_updated and count == num_v - 1:
            return False
    return True


for _ in range(int(read())):
    num_v, num_road, num_wormhole = map(int, read().split())
    edges: list[list[Edge]] = [list() for _ in range(num_v + 1)]
    for _ in range(num_road):
        start, end, weight = map(int, read().split())
        edges[start].append((weight, end))
        edges[end].append((weight, start))
    for _ in range(num_wormhole):
        start, end, abs_weight = map(int, read().split())
        edges[start].append((-abs_weight, end))

    for vertex in range(1, num_v + 1):
        edges[0].append((0, vertex))
    """
    0번째 점(가상의 점)을 출발점으로  잡고, 이를 모든 점과 연결하는 0짜리 간선들을 추가한다.
    0번째 점을 탐색해서 밝혀지는 음수 사이클은 다 이미 기존에 존재하는 사이클이고,
    0번째 점은 모든 점과 연결되어 있다.
    따라서 연결 그래프를 아닐 경우를 대비해서 모든 점에서 시작해서 체크할 필요가 없다.
    """
    start_v = 0
    if check_no_minus_cycle_bellman_ford(start_v, edges):
        print("NO")
    else:
        print("YES")
