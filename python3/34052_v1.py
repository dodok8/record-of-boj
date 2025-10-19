from sys import stdin

read = lambda: stdin.readline().rstrip()

ans = 300

for _ in range(4):
    ans += int(read())


if ans <= 1800:
    print("Yes")
else:
    print("No")
