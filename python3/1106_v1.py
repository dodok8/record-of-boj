# 호텔

from sys import stdin, maxsize


def read():
    return stdin.readline().rstrip()


c, n = map(int, read().split())
dp = [maxsize for _ in range(c + 100)]
dp[0] = 0

costs = list()
people = list()

for _ in range(n):
    cost, person = map(int, read().split())
    costs.append(cost)
    people.append(person)

for idx in range(1, c + 100):
    for jdx in range(0, n):
        if idx >= people[jdx]:
            dp[idx] = min(dp[idx - people[jdx]] + costs[jdx], dp[idx])

print(min(dp[c:]))
