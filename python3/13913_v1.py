from collections import deque

n, k = map(int, input().split())

if n > k:
    print(n - k)
    print(*[int(x) for x in range(n, k - 1, -1)])

else:
    dists = [-1 for idx in range(100001)]
    parents = [-1 for idx in range(100001)]
    dists[n] = 0

    tq = deque()
    tq.append((0, n))

    while tq:
        dist, x = tq.popleft()

        if x == k:
            break

        for nx in [x - 1, x + 1, x * 2]:
            if nx < 0 or nx >= 100001:
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
