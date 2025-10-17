# 랜선 자르기

from sys import stdin

read = lambda: stdin.readline().rstrip()

k, n = map(int, read().split())

wires = [int(read()) for _ in range(k)]

start = 0
end = max(wires)

max_p = 0
while start <= end:
    mid = (start + end) // 2
    if mid == 0:
        break
    num_wire = 0
    for wire in wires:
        num_wire += wire // mid
    if num_wire >= n:
        start = mid + 1
        if mid > max_p:
            max_p = mid
    else:
        end = mid - 1

print(max_p)
