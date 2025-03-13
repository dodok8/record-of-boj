from sys import stdin

read = lambda: stdin.readline().rstrip()


def check(letters):
    vowel = {"a", "e", "i", "o", "u"}
    v_cnt = 0
    c_cnt = 0

    for letter in letters:
        if letter in vowel:
            v_cnt += 1
        else:
            c_cnt += 1

    if v_cnt >= 1 and c_cnt >= 2:
        return True
    else:
        return False


l, c = map(int, read().split())
letters = sorted(read().split())


arr = list()


def back_track(idx):
    if len(arr) == l and check(arr):
        print("".join(arr))

    for idx in range(idx, c):
        arr.append(letters[idx])
        back_track(idx + 1)
        arr.pop()


back_track(0)
