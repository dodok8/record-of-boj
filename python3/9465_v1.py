# 스티커

from sys import stdin

read = lambda: stdin.readline().rstrip()

UP = 0
DOWN = 1

t = int(read())

for _ in range(t):
    n = int(read())
    stickers = []
    for __ in range(2):
        stickers.append(list(map(int, read().split())))

    dp = [[0 for _ in range(n)] for __ in range(2)]
    dp[UP][0] = stickers[UP][0]
    dp[DOWN][0] = stickers[DOWN][0]

    for k in range(1, n):
        dp[UP][k] = max(
            dp[DOWN][k - 1] + stickers[UP][k],
            max(dp[UP][k - 2], dp[DOWN][k - 2]) + stickers[UP][k],
        )
        dp[DOWN][k] = max(
            dp[UP][k - 1] + stickers[DOWN][k],
            max(dp[UP][k - 2], dp[DOWN][k - 2]) + stickers[DOWN][k],
        )

    ans = max(dp[DOWN][n - 1], dp[UP][n - 1])
    print(ans)
