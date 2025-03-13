# 모든 순열

n = int(input())


def get_permutations(items, n):
    answers = list()

    def permutation_helper(curr_items):
        if len(curr_items) == n:
            answers.append(curr_items.copy())
            return

        for item in items:
            if item not in curr_items:
                curr_items.append(item)
                permutation_helper(curr_items)
                curr_items.pop()

    permutation_helper([])
    return answers


for permutation in get_permutations(list(range(1, n + 1)), n):
    print(" ".join(map(str, permutation)))
