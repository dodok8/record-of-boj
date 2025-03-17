from collections import deque

n, k = map(int, input().split())

dists = [-1 for idx in range(100001)]
parents = [0 for idx in range(100001)]
parents[n] = -1
dists[n] = 0

tq = deque()
tq.append((0, n))

while tq:
    dist, x = tq.popleft()

    for nx in [x - 1, x + 1, x * 2]:
        if nx < 0 or nx >= 100000:
            continue
        if dists[nx] == -1:
            dists[nx] = dist + 1
            parents[nx] = x
            tq.append((dists[nx], nx))

print(dists[k])

x = k
ans = list()
ans.append(x)
while parents[x] != -1:
    ans.append(parents[x])
    x = parents[x]
ans.reverse()

print(*ans)
