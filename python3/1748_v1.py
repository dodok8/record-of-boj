from sys import stdin

read = lambda: stdin.readline().rstrip()

n = int(read())
power = 0

temp = n
while temp > 0:
    temp //= 10
    power += 1
power -= 1
ans = 0

for idx in range(power):
    ans += (idx + 1) * (10 ** (idx + 1) - 10 ** (idx))

ans += (power + 1) * (n - (10**power) + 1)
print(ans)
