# 1, 2, 3 더하기 4

from sys import stdin

input = lambda: stdin.readline().rstrip()

dp = [[0] * 4 for _ in range(10001)]

dp[1][1] = 1
dp[2][1] = 1
dp[2][2] = 1
dp[3][1] = 1
dp[3][2] = 1
dp[3][3] = 1

for idx in range(4, 10001):
    dp[idx][1] = dp[idx - 1][1]
    dp[idx][2] = dp[idx - 2][2] + dp[idx - 2][1]
    dp[idx][3] = dp[idx - 3][1] + dp[idx - 3][2] + dp[idx - 3][3]

for _tdx in range(int(input())):
    n = int(input())
    print(dp[n][1] + dp[n][2] + dp[n][3])
