from sys import stdin

read = lambda: stdin.readline().rstrip()


def bubble_sort(given_arr):
    arr = given_arr.copy()
    for i in range(len(arr) - 1, 0, -1):
        for j in range(i):
            if arr[j] > arr[j + 1]:
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
    return arr


def wrong_sort(given_nums: list[int]):
    nums = given_nums.copy()
    N = len(nums)
    for i in range(N - 1, 0, -1):
        for j in range(i - 1, -1, -1):
            if nums[j] > nums[j + 1]:
                temp = nums[j]
                nums[j] = nums[j + 1]
                nums[j + 1] = temp
    return nums


def make_counter(length: int):
    result = list()
    result.append(10000)
    result.append(0)
    for i in range(length - 1, 1, -1):
        result.append(i)
    return result


print("hi")
for i in range(3, 1000):
    nums = make_counter(i)
    wrong = wrong_sort(nums)
    good = bubble_sort(nums)
    if wrong == good:
        print(f"{i}는 not okay")

counters = make_counter(int(read()))

for i in counters:
    print(i, end=" ")
