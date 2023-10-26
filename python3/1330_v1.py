from sys import stdin

read = lambda: stdin.readline().rstrip()

a, b = map(int, read().split())
if a > b:
    print(">")
if a < b:
    print("<")
if a == b:
    print("==")
