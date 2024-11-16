# 동아리비 횡령

import re
from sys import stdin

read = lambda: stdin.readline().rstrip()

exp = re.compile(r"^([1-9][0-9]*)\+([1-9][0-9]*)$")
match = exp.match(read())

if match and match.group(1) == match.group(2):
    print("CUTE")
else:
    print("No Money")
