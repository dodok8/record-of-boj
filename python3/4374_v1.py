# 지프의 법칙

from collections import defaultdict

alphabets = set()

for idx in range(97, 123):
    alphabets.add(chr(idx))

while True:
    try:
        n = int(input())
        words_dict = defaultdict(int)
        words = set()
        new_word = list()

        while True:
            line = input()
            if line == "EndOfText":
                break
            for idx, letter in enumerate(line):
                letter = letter.lower()
                if letter not in alphabets or idx == len(line) - 1:
                    if len(new_word) == 0:
                        continue
                    else:

                        if idx == len(line) - 1 and letter in alphabets:
                            new_word.append(letter)
                        word = "".join(new_word)
                        new_word.clear()
                        words_dict[word] += 1
                        words.add(word)

                else:
                    new_word.append(letter)

        words = sorted(list(words))

        word_exist = False
        for word in words:
            if words_dict[word] == n:
                print(word)
                word_exist = True
        if not word_exist:
            print("There is no such word.")
        print()
    except:
        break
