# 피갤컵
from sys import stdin

read = lambda: stdin.readline().rstrip()

n = int(read())

year, month = 2024, 8
month += (n - 1) * 7
if month % 12 == 0:
    year += (month // 12) - 1
    month = 12
else:
    year += month // 12
    month %= 12
print(f"{year} {month}")
