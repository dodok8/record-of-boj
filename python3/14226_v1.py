# 이모티콘

from sys import stdin
from collections import deque

read = lambda: stdin.readline().rstrip()

s = int(input())

travel_q = deque()
travel_q.append(((1, 0), 0))

MAX = 2000

visited = [[False for _ in range(MAX)] for _idx in range(MAX)]

while len(travel_q) > 0:
    (display, clipboard), curr_t = travel_q.popleft()
    if display == s:
        print(curr_t)
        break

    if 0 < display < MAX:
        if not visited[display][display]:
            visited[display][display] = True
            travel_q.append(((display, display), curr_t + 1))
        if not visited[display - 1][clipboard]:
            visited[display - 1][clipboard] = True
            travel_q.append(((display - 1, clipboard), curr_t + 1))

    if 0 < clipboard and display + clipboard < MAX:
        if not visited[display + clipboard][clipboard]:
            visited[display + clipboard][clipboard] = True
            travel_q.append(((display + clipboard, clipboard), curr_t + 1))
