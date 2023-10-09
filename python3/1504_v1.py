import heapq as pq
from sys import stdin, maxsize as 최고값
from typing import TypeVar as 타입변수, Generic as 제네릭


읽기 = lambda: stdin.readline().rstrip()
길이 = len
ㅌ = 타입변수("T")
출력하기 = print
정수 = int
범위 = range
예외 = Exception


class 연결_안됨_예외(예외):
    pass


class 힙(제네릭[ㅌ]):
    def __init__(self, data: list[ㅌ] = []) -> None:
        self.data = data.copy()
        pq.heapify(self.data)

    def 넣기(self, item: ㅌ):
        pq.heappush(self.data, item)
        return self

    def 꺼내기(self) -> ㅌ:
        return pq.heappop(self.data)

    def 확장하기(self, items: list[ㅌ]):
        self.data.extend(items)
        pq.heapify(self.data)
        return self

    def __len__(self) -> int:
        return len(self.data)


def 최단거리_구하기(시작_정점: int, 끝_정점: int, 간선들: list[tuple[int, int]]):
    global 갯수_정점
    거리들 = [최고값 for _ in 범위(갯수_정점 + 1)]
    여행_힙 = 힙()
    여행_힙.넣기((0, 시작_정점))
    거리들[시작_정점] = 0

    while 길이(여행_힙) != 0:
        현재_거리, 현재_정점 = 여행_힙.꺼내기()
        if 현재_거리 > 거리들[현재_정점]:
            continue
        for 거리, 이웃_정점 in 간선들[현재_정점]:
            if 거리들[이웃_정점] > 현재_거리 + 거리:
                거리들[이웃_정점] = 현재_거리 + 거리
                여행_힙.넣기((현재_거리 + 거리, 이웃_정점))
    if 거리들[끝_정점] == 최고값:
        raise 연결_안됨_예외
    return 거리들[끝_정점]


갯수_정점, 갯수_간선 = map(정수, 읽기().split())
간선들 = [list() for _ in 범위(갯수_정점 + 1)]

for _ in 범위(갯수_간선):
    시작, 끝, 거리 = map(정수, 읽기().split())
    간선들[시작].append((거리, 끝))
    간선들[끝].append((거리, 시작))

첫_경유점, 두번째_경유점 = map(정수, 읽기().split())

try:
    첫_경로 = (
        최단거리_구하기(1, 첫_경유점, 간선들)
        + 최단거리_구하기(첫_경유점, 두번째_경유점, 간선들)
        + 최단거리_구하기(두번째_경유점, 갯수_정점, 간선들)
    )

    두번째_경로 = (
        최단거리_구하기(1, 두번째_경유점, 간선들)
        + 최단거리_구하기(두번째_경유점, 첫_경유점, 간선들)
        + 최단거리_구하기(첫_경유점, 갯수_정점, 간선들)
    )

    if 첫_경로 > 두번째_경로:
        출력하기(두번째_경로)
    else:
        출력하기(첫_경로)

except 연결_안됨_예외:
    출력하기(-1)
