from sys import stdin

read = lambda: stdin.readline().rstrip()

while True:
    edges = list(map(int, input().split()))
    if edges[0] == 0 and edges[1] == 0 and edges[2] == 0:
        break
    edges.sort()
    if edges[2] ** 2 == edges[0] ** 2 + edges[1] ** 2:
        print("right")
    else:
        print("wrong")
