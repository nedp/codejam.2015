# Forgot to record the time for this one, but it was a very long time.
# I didn't know how to deal with the imprecision of float comparisons
# (use an epsilon to make the acceptable range larger).
# Won't make the same mistake again.

import math

ep = 1e-14

def min_chops(forest, focus):
    n = len(forest) - 1
    if n < 2:
        return 0
    m = 2 * n
    
    # Find and sort angles from the focus to other trees.
    angles = [angle(focus, tree) for tree in forest if tree is not focus]
    angles.sort()
    angles = angles + [a + 2*math.pi for a in angles]
    
    # Treat all trees in a (slightly under) 1pi arc
    # about the focus as needing to be cut down.
    # Find the arc with the minimum number of trees.
    # Arc to chop goes from i to j exclusive.
    min_chops = n-1 # Worst possible result.
    j = 1
    for i in range(n):
        while j < i + n - 1 and angles[j] < angles[i] + math.pi - ep:
            j += 1
        
        if j == i:
            j += 1
        elif (j-1 - i) < min_chops:
            if j-1 == i: # Best possible result.
                return 0
            else:
                min_chops = j-1 - i
    
    return min_chops

def angle(focus, tree):
    x0, y0 = focus
    x, y = tree
    return math.atan2(y - y0, x - x0)

def test():
    cases = []
    cases.append([
        ((0, 0), 0),
        ((10, 0), 0),
        ((10, 10), 0),
        ((0, 10), 0),
        ((5, 5), 1),
    ])
    cases.append([
            ((0, 0), 0),
            ((5, 0), 0),
            ((10, 0), 0),
            ((0, 5), 0),
            ((5, 5), 3),
            ((10, 5), 0),
            ((0, 10), 0),
            ((5, 10), 0),
            ((10, 10), 0),
    ])
    cases.append([
            ((0, 0), 0),
            ((5, 0), 0),
            ((10, 0), 0),
            ((0, 5), 0),
            ((3, 3), 2),
            ((3, 5), 1),
            ((5, 5), 1),
            ((10, 5), 0),
            ((0, 10), 0),
    ])
    cases.append([
            ((0, 0), 0),
            ((5, 0), 0),
            ((10, 0), 0),
            ((0, 5), 0),
            ((5, 5), 0),
            ((10, 5), 0),
    ])
    cases.append([
        ((0, 0), 0),
        ((5, 0), 0),
    ])
    cases.append([
        ((0, 0), 0),
        ((5, 0), 0),
        ((10, 0), 0),
    ])
    cases.append([
        ((0, 0), 0),
        ((5, 0), 0),
        ((10, 0), 0),
        ((20, 0), 0),
    ])
    cases.append([
        ((0, 0), 0),
        ((5, 0), 0),
        ((10, 2), 1),
        ((10, 5), 0),
        ((20, 0), 0),
    ])
    

    for (i, case) in enumerate(cases):
        did_fail = False
        forest, wants = zip(*case)
        for tree, want in zip(forest, wants):
            got = min_chops(forest, tree)
            try:
                assert want == got
            except AssertionError:
                print('tree {} failed. want: {}, got: {}'.format(tree, want,got))
                did_fail = True
        
        result = 'FAILED' if did_fail else 'PASSED'
        
        print('Case {}: {}'.format(i, result))

if __name__ == '__main__':
    t = int(input())
    
    for i in range(1, t+1):
        print('Case #{}:'.format(i))
        n = int(input())
        forest = []
        for _ in range(n):
            x, y = tuple(int(o) for o in input().split())
            forest.append((x, y))
        
        for tree in forest:
            print(min_chops(forest, tree))
