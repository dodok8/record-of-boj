import re

from sys import stdin

read = lambda: stdin.readline().rstrip()

s = read()

pattern = re.compile(r'^"[a-zA-Z\s]+"$')


if bool(pattern.match(s)):
    print(s.strip('"'))
else:
    print("CE")
