# Small Start: 20:11
# Small End: 20:51


# THIS IS ONLY WORKING FOR SMALL INPUT

def small(ms, b, n):
    if n <= b:
        return n
    ends = list(ms)
    n -= b
    t = 0
    
    while n > 0:
        t += 1
        for (i, end) in enumerate(ends):
            if end == t:
                n -= 1
                if n == 0:
                    return i+1 # 1 indexed
                else:
                    ends[i] = t + ms[i]

def large(ms, n):
    #print('starting: {} {}'.format(ms, n))
    b = len(ms)
    if n <= b:
        return n
    
    period = lcm_list(ms)
    
    per_period = 0
    
    for m in ms:
        per_period += period // m
    
    n_mod = int(n % per_period)
    if n_mod == 0:
        n_mod = per_period
    #print('n_mod: {}'.format(n_mod))
    
    return small(ms, b, n_mod)
    

def gcd(a, b):
    # Euclid's algorithm
    while b:
        a, b = b, a % b
    
    return a

def lcm(a, b):
    # find product, divide by common factor
    return (a * b) // gcd(a, b)

def lcm_list(xs):
    if len(xs) == 0:
        return None
    if len(xs) == 1:
        return xs[0]
    
    running_lcm = 1
    ls = list(xs)
    while ls:
        running_lcm = lcm(running_lcm, ls.pop())
    
    return running_lcm

if __name__ == '__main__':
    t = int(input())
    for i in range(1, t+1):
        b, n = map(int, input().split(' '))
        ms = tuple(map(int, input().split(' ')))
        
        assert b == len(ms)
        result = large(ms, n)
        print('Case #{}: {}'.format(i, result))
