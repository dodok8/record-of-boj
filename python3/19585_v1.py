# 전설

from sys import stdin

read = lambda: stdin.readline().rstrip()


class Trie:
    def __init__(self, is_word):
        self.tree = {}
        self.is_word = is_word

    def insert(self, word: str):
        curr_node = self
        for char in word:
            if char not in curr_node.tree:
                curr_node.tree[char] = Trie(False)
            curr_node = curr_node.tree[char]
        curr_node.is_word = True

    def find(self, word: str, result: list[int]):
        # 닉네임에서 색 이름이 아니게 되는 인덱스를 result 에 넣기
        curr_node = self
        for idx, char in enumerate(word):
            if curr_node.is_word:
                result.append(idx - 1)
            if char in curr_node.tree:
                curr_node = curr_node.tree[char]
            else:
                break


num_c, num_n = map(int, read().split())

color_trie = Trie(False)

for idx in range(num_c):
    color = read()
    color_trie.insert(color)

nickname_set = set()

for idx in range(num_n):
    nickname = read()
    nickname_set.add(nickname)

num_q = int(read())

for _ in range(num_q):
    team = read()

    color_result = []
    color_trie.find(team, color_result)

    result = False

    for cdx in color_result:
        remaining = team[cdx + 1 :]
        if remaining in nickname_set:
            result = True
            break

    if not result:
        print("No")
    else:
        print("Yes")
