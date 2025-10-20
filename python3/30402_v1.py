# 감마선을 맞은 컴퓨터

from sys import stdin

read = lambda: stdin.readline().rstrip()

for _ in range(15):
    if "w" in read().split():
        print("chunbae")
        break
    elif "b" in read().split():
        print("nabi")
        break
    else:
        print("yeongcheol")
        break
