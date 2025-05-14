# 전설

from sys import stdin, setrecursionlimit

setrecursionlimit(10**6)

read = lambda: stdin.readline().rstrip()


class Trie:
    def __init__(self, is_word):
        self.tree = {}
        self.is_word = is_word

    def insert(self, word: list[int]):
        if len(word) == 0:
            return

        if word[0] not in self.tree:
            self.tree[word[0]] = Trie(False)

        if len(word) == 1:
            self.tree[word[0]].is_word = True

        self.tree[word[0]].insert(word[1:])

    def find(self, word: list[int], idx: int, result: list[int]):
        # 닉네임에서 색 이름이 아니게 되는 인덱스를 result 에 넣기
        if self.is_word:
            result.append(idx - 1)

        if idx >= len(word):
            return

        if word[idx] in self.tree:
            self.tree[word[idx]].find(word, idx + 1, result)


num_c, num_n = map(int, read().split())

color_trie = Trie(False)

for idx in range(num_c):
    color = list(map(ord, read()))
    color_trie.insert(color)

nickname_set = set()

for idx in range(num_n):
    nickname = list(map(ord, read()))
    nickname_set.add(tuple(nickname))

num_q = int(read())

for _ in range(num_q):
    team = list(map(ord, read()))

    color_result = []
    color_trie.find(team, 0, color_result)

    result = False

    for cdx in color_result:
        remaining = team[cdx + 1 :]
        if tuple(remaining) in nickname_set:
            result = True
            break

    if not result:
        print("No")
    else:
        print("Yes")
