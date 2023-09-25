from sys import stdin

read = lambda: stdin.readline().rstrip()

start, end = map(int, read().split())
curr = end
step = 0

while curr != 1 and curr != start:
    step += 1
    if curr % 10 == 1:
        curr = curr // 10
    elif curr % 2 == 0:
        curr = curr // 2
    else:
        break

if curr == start:
    print(step + 1)
else:
    print(-1)
