"""
Make a chain of dominoes.
"""

from collections import defaultdict

class Domino:

    def __init__(self, left: int, right: int) -> None:
        self.left = left
        self.right = right
    
    def flip(self):
        self.left, self.right = self.right, self.left
    
    def __repr__(self) -> str:
        return f'[{self.left}|{self.right}]'

def contains_odd(dominoes: list[tuple[int]]) -> bool:
    counts = defaultdict(int)
    for domino in dominoes:
        for num in domino:
            counts[num] += 1
    for num in counts:
        if counts[num] % 2 == 1:
            return True
    return False

def dominoify_input(dominoes: list[tuple[int]]) -> list[Domino]:
    result = []
    for domino in dominoes:
        new_domino = Domino(domino[0], domino[1])
        result.append(new_domino)
    return result

def can_chain(input_dominoes: list[tuple[int]]) -> list[tuple[int]]:
    """return valid chain of dominoes if possible"""
    if len(input_dominoes) == 0:
        return []
    if len(input_dominoes) == 1:
        if len(set(input_dominoes[0])) == 1:
            return input_dominoes
        return None
    if contains_odd(input_dominoes):
        return None
    dominoes = dominoify_input(input_dominoes)
    chain = [dominoes.pop(0)]
    count = len(dominoes)
    while dominoes:
        count -= 1
        for index, domino in enumerate(dominoes):
            if domino.right == chain[-1].right:
                domino.flip()
            if len(dominoes) == 1:
                if domino.right == chain[0].left:
                    chain.append(domino)
                    return [(domino.left, domino.right) for domino in chain] 
                return None  
            if domino.left == chain[-1].right and domino.right != chain[0].left:
                chain.append(dominoes.pop(index))
                continue
            if count == 0:
                return None
    return None



if __name__ == "__main__":
    # print(can_chain([(1, 2), (2, 1), (3, 4), (4, 3)]))
    # print(can_chain([(1, 2), (3, 1), (3, 2)]))
    # print(can_chain([(1,1)]))
    # print(can_chain([]))
    print(can_chain([(1, 2), (2, 3), (3, 1), (1, 1), (2, 2), (3, 3)]))
