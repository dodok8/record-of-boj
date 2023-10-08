from sys import stdin

read = lambda: stdin.readline().rstrip()


def get_amount_of_tree(pivot):
    return sum(map(lambda x: 0 if x < pivot else x - pivot, trees))


def search_pivot(left, right):
    pivot = (right + left) // 2
    if left == right or left == right - 1:
        return left
    amount = get_amount_of_tree(pivot)
    if amount > m:
        return search_pivot(pivot, right)
    elif amount < m:
        return search_pivot(left, pivot)
    else:
        return pivot


n, m = map(int, read().split())
trees = list(map(int, read().split()))
trees.sort(reverse=True)


print(search_pivot(0, trees[0]))
