from sys import stdin

read = lambda: stdin.readline().rstrip()

n = int(read())

times = list()
pays = list()

dp = [0 for _ in range(n + 1)]  # n +1 일까지 벌수 있는 최대값

for _idx in range(n):
    t, p = map(int, read().split())
    times.append(t)
    pays.append(p)

times.append(0)
pays.append(0)

max_pay = 0

for idx in range(n + 1):
    dp[idx] = max(max_pay, dp[idx])  # 아무 일도 안하면 이전 최대값이 그대로 유지

    if times[idx] + idx <= n:
        jdx = times[idx] + idx
        dp[jdx] = max(dp[jdx], dp[idx] + pays[idx])
    max_pay = max(max_pay, dp[idx])  # 유지되는 최대로 벌 수 있는 돈 업데이트

print(dp[-1])
