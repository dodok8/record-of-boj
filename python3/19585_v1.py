# 전설

from sys import stdin

read = lambda: stdin.readline().rstrip()


class Trie:
    def __init__(self, is_word):
        self.tree = {}
        self.is_word = is_word

    def insert(self, word: list[int]):
        curr_node = self
        for char in word:
            if char not in curr_node.tree:
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
            if char in curr_node.tree:
                curr_node = curr_node.tree[char]
            else:
                break

        return result


num_c, num_n = map(int, read().split())

color_trie = Trie(False)

for idx in range(num_c):
    color = list(map(ord, read()))
    color_trie.insert(color)

# 닉네임을 Trie에 저장
nickname_trie = Trie(False)
for idx in range(num_n):
    nickname = list(map(ord, read()))
    nickname_trie.insert(nickname)

num_q = int(read())

for _ in range(num_q):
    team = list(map(ord, read()))

    color_result = color_trie.find(team)

    result = False

    for cdx in color_result:
        # 남은 부분이 닉네임인지 확인
        remaining = team[cdx + 1 :]

        # 남은 문자열이 nickname_trie에 있는지 확인
        curr_node = nickname_trie
        is_nickname = True

        for char in remaining:
            if char not in curr_node.tree:
                is_nickname = False
                break
            curr_node = curr_node.tree[char]

        if is_nickname and curr_node.is_word:
            result = True
            break

    if not result:
        print("No")
    else:
        print("Yes")
