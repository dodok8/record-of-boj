# A와 B 2

import sys

S = input()
T = input()


def dfs(word):
    if word == S:
        print(1)
        sys.exit()
    if len(word) == 0:
        return
    if word[-1] == "A":
        dfs(word[:-1])
    if word[0] == "B":
        dfs(word[1:][::-1])


dfs(T)
print(0)
