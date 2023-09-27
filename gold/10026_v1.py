from sys import stdin
from collections import deque

read = lambda: stdin.readline().rstrip()
Point = tuple[int, int]


def get_connected_points_per_color(point: Point, color: str, blinded: bool):
    connected_points = []
    x = point[0]
    y = point[1]
    for x0, y0 in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
        if (0 <= x + x0 < n) and (0 <= y + y0 < n):
            if blinded and (color == "R" or color == "G"):
                if not visited[x + x0][y + y0] and (
                    board[x + x0][y + y0] == "R"
                    or board[x + x0][y + y0] == "G"
                ):
                    connected_points.append((x + x0, y + y0))
            else:
                if (
                    not visited[x + x0][y + y0]
                    and board[x + x0][y + y0] == color
                ):
                    connected_points.append((x + x0, y + y0))
    return connected_points


n = int(read())
board = [list(read()) for _ in range(n)]

visited = [[False for __ in range(n)] for _ in range(n)]
num_component = 0

for idx in range(n):
    for jdx in range(n):
        point = (idx, jdx)
        if visited[point[0]][point[1]]:
            continue

        num_component += 1
        travel_queue = deque()
        travel_queue.append(point)

        while len(travel_queue) != 0:
            curr_point = travel_queue.popleft()
            connected_points = get_connected_points_per_color(
                curr_point, board[curr_point[0]][curr_point[1]], False
            )
            for point in connected_points:
                visited[point[0]][point[1]] = True
            travel_queue.extend(connected_points)

visited = [[False for __ in range(n)] for _ in range(n)]
blinded_component = 0

for idx in range(n):
    for jdx in range(n):
        point = (idx, jdx)
        if visited[point[0]][point[1]]:
            continue

        blinded_component += 1
        travel_queue = deque()
        travel_queue.append(point)

        while len(travel_queue) != 0:
            curr_point = travel_queue.popleft()
            connected_points = get_connected_points_per_color(
                curr_point, board[curr_point[0]][curr_point[1]], True
            )
            for point in connected_points:
                visited[point[0]][point[1]] = True
            travel_queue.extend(connected_points)

print(f"{num_component} {blinded_component}")
