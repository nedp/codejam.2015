# Start: 21:39
# End: 22:14

def logging(trees):
    return [min_cut(trees, tree) for tree in trees]

def min_cut(trees, tree):
    min_cut = len(trees)
    for other in trees:
        if other == tree:
            continue
        above, below = n_trees_outside(trees, tree, other)
        min_cut = min(min_cut, above, below)
    
    return min_cut

def n_trees_outside(trees, a, b):
    ax, ay = a
    bx, by = b
    
    above = 0
    below = 0
    
    if ax == bx:
        for (x, y) in trees:
            if x == ax:
                continue
            if x > ax:
                above += 1
            if x < ax:
                below += 1
        return above, below
    
    m = (by - ay) / (bx - ax)
    
    for (x, y) in trees:
        target_y = ay + m * (x - ax)
        if y == target_y:
            continue
        elif y > target_y:
            above += 1
        elif y < target_y:
            below += 1
    
    return above, below


if __name__ == '__main__':
    t = int(input())
    
    for i in range(1, t+1):
        print('Case #{}:'.format(i))
        n_trees = int(input())
        trees = [tuple(map(int, input().split(' '))) for _ in range(n_trees)]
        
        for n in logging(trees):
            print(n)
