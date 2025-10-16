# 정수 제곱근

from sys import stdin

read = lambda: stdin.readline().rstrip()

n = int(read())

start = 0
end = n

ans = n

while start <= end:
    mid = (start + end) // 2

    if mid**2 >= n:
        start = start
        end = mid - 1
        ans = mid
    else:
        start = mid + 1
        end = end

print(ans)
