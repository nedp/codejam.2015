# Started: 19:40
# Ended: 20:00

def test_case(quantities):
    intervals = list(zip(quantities[:-1], quantities[1:]))
    return variable_eats(intervals), fixed_eats(intervals)

def variable_eats(intervals):
    n_eats = 0
    for (a, b) in intervals:
        diff = b - a
        if diff < 0:
            n_eats -= diff
    
    return n_eats

def fixed_eats(intervals):
    max_eat = 0
    for (a, b) in intervals:
        diff = b - a
        if -diff > max_eat:
            max_eat = -diff
    
    n_eats = 0
    for (a, b) in intervals:
        n_eats += min(a, max_eat)
    
    return n_eats

if __name__ == '__main__':
    n_cases = int(input())
    for i in range(1, n_cases+1):
        n_quantities = input()
        quantities = list(map(int, input().split(' ')))
        
        variable, fixed = test_case(quantities)
        print('Case #{}: {} {}'.format(i, variable, fixed))
