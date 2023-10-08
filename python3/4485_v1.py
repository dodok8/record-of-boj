import heapq as pq
from sys import stdin, maxsize
from typing import TypeVar, Generic

Point = tuple[int, int]
Board = list[list[int]]
read = lambda: stdin.readline().rstrip()
T = TypeVar("T")


def get_connected_points(point: Point, board: Board) -> list[Point]:
    r = len(board)
    c = len(board[0])
    x = point[0]
    y = point[1]

    connected_points = []
    for x0, y0 in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
        if (0 <= x + x0 < r) and (0 <= y + y0 < c):
            if board[x + x0][y + y0] != -1:
                connected_points.append((x + x0, y + y0))
    return connected_points


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


def get_weight_sum(p: Point, weight_sum: list[list[int]]) -> int:
    return weight_sum[p[0]][p[1]]


def get_weight(p: Point, board: list[list[int]]) -> int:
    return board[p[0]][p[1]]


test_count = 1
while True:
    n = int(read())
    if n == 0:
        break
    board: list[list[int]] = [list(map(int, read().split())) for _ in range(n)]
    start_point = (0, 0)
    travel_pq = Heap[tuple[int, Point]](
        [(get_weight(start_point, board), start_point)]
    )

    weight_sum = [[maxsize for _ in range(n)] for __ in range(n)]
    weight_sum[0][0] = get_weight(start_point, board)

    while len(travel_pq) != 0:
        curr_weight_sum, curr_point = travel_pq.pop()
        if curr_weight_sum > get_weight_sum(curr_point, weight_sum):
            continue
        for end_point in get_connected_points(curr_point, board):
            next_weight = curr_weight_sum + get_weight(end_point, board)
            if get_weight_sum(end_point, weight_sum) > next_weight:
                weight_sum[end_point[0]][end_point[1]] = next_weight
                travel_pq.push((next_weight, end_point))

    print(f"Problem {test_count}: {weight_sum[n-1][n-1]}")
    test_count += 1
