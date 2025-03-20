# 치킨댄스를 추는 곰곰이를 본 임스 2
from sys import stdin


def read():
    return stdin.readline().rstrip()


n = int(read())
cnt = 0
for idx in range(n):
    num = int(read()[2::])
    if num <= 90:
        cnt += 1

print(cnt)
