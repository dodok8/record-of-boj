from sys import stdin

read = lambda: stdin.readline().rstrip()


def check_candies(candies):
    max_ans = 0
    for idx in range(n):
        ans = 1
        for jdx in range(1, n):
            if candies[idx][jdx - 1] == candies[idx][jdx]:
                ans += 1
            else:
                ans = 1
            if max_ans <= ans:
                max_ans = ans
    for idx in range(n):
        ans = 1
        for jdx in range(1, n):
            if candies[jdx - 1][idx] == candies[jdx][idx]:
                ans += 1
            else:
                ans = 1
            if max_ans <= ans:
                max_ans = ans
    return max_ans


n = int(read())

candies = list()

for _idx in range(n):
    candies.append(list(read()))

ans = 0

for idx in range(n):
    for jdx in range(n):
        if idx != n - 1 and candies[idx][jdx] != candies[idx + 1][jdx]:
            candies[idx][jdx], candies[idx + 1][jdx] = candies[idx + 1][jdx], candies[idx][jdx]
            ans = max(ans, check_candies(candies))
            candies[idx][jdx], candies[idx + 1][jdx] = candies[idx + 1][jdx], candies[idx][jdx]
        if jdx != n - 1 and candies[idx][jdx] != candies[idx][jdx + 1]:
            candies[idx][jdx], candies[idx][jdx + 1] = candies[idx][jdx + 1], candies[idx][jdx]
            ans = max(ans, check_candies(candies))
            candies[idx][jdx], candies[idx][jdx + 1] = candies[idx][jdx + 1], candies[idx][jdx]
print(ans)
