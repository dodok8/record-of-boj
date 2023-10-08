from sys import stdin
from collections import deque
from copy import deepcopy


def get_connected_points(point: tuple[int, int]) -> list[tuple[int, int]]:
    global result, m, n
    connected_points = []
    x = point[0]
    y = point[1]
    for x0, y0 in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
        if (0 <= x + x0 < n) and (0 <= y + y0 < m):
            if result[x + x0][y + y0] == 0:
                connected_points.append((x + x0, y + y0))
    return connected_points


def get_start_point() -> list[tuple[int, int]]:
    global board
    result = []
    for idx, col in enumerate(board):
        for jdx, item in enumerate(col):
            if item == 1:
                result.append((idx, jdx))
    return result


def check_ready() -> bool:
    global result
    for col in result:
        for item in col:
            if item == 0:
                return False
    return True


read = lambda: stdin.readline().rstrip()

m, n = map(int, read().split())
board = [list(map(int, read().split())) for _ in range(n)]
result = deepcopy(board)

start_points = get_start_point()
travel_queue = deque()


for start_point in start_points:
    travel_queue.append(start_point)

end_point: tuple[int, int] = (0, 0)

while True:
    try:
        curr_point = travel_queue.popleft()
        connected_points = get_connected_points(curr_point)
        for point in connected_points:
            result[point[0]][point[1]] = (
                result[curr_point[0]][curr_point[1]] + 1
            )
        travel_queue.extend(connected_points)

        if len(travel_queue) == 0:
            end_point = curr_point
            if check_ready():
                print(result[end_point[0]][end_point[1]] - 1)
            else:
                print(-1)
            break
    except IndexError:
        print(-1)  # 익은 토마토가 없는 경우
        break
