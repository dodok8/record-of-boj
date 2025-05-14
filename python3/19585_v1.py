# 전설

from sys import stdin

read = lambda: stdin.readline().rstrip()
convert = lambda x: ord(x) - 97


class Trie:
    def __init__(self, is_word):
        self.tree = [None for _ in range(27)]
        self.is_word = is_word

    def insert(self, word: list[int]):
        curr_node = self
        for char in word:
            if curr_node.tree[char] is None:
                curr_node.tree[char] = Trie(False)
            curr_node = curr_node.tree[char]
        curr_node.is_word = True

    def find(self, word: list[int]):
        # 닉네임에서 색 이름이 아니게 되는 인덱스를 result 에 넣기

        result = list()

        curr_node = self
        for idx, char in enumerate(word):
            if curr_node.is_word:
                result.append(idx - 1)
            if curr_node.tree[char] is not None:
                curr_node = curr_node.tree[char]
            else:
                break

        return result


num_c, num_n = map(int, read().split())

color_trie = Trie(False)

for idx in range(num_c):
    color = list(map(convert, read()))
    color_trie.insert(color)

nickname_set = set()

for idx in range(num_n):
    nickname = tuple(map(convert, read()))
    nickname_set.add(nickname)

num_q = int(read())

for _ in range(num_q):
    team = list(map(convert, read()))

    color_result = []
    color_result = color_trie.find(team)

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
