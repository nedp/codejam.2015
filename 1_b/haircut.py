# Large Start: 21:07
# Large End: 21:21

def n_assigned(ms, t):
    if t < 0:
        return 0
    n = len(ms)
    for m in ms:
        n += t // m
    return n

def haircut(ms, t):
    sms = sorted(ms)
    min_m = sms[0]
    
    t = min_m * n // 2
    d = t // 2
    while d:
        n_a = n_assigned(ms, t)
        if n_a > n:
            t -= d
        elif n_a <= n:
            t += d
        d //= 2
    
    # Ensure that n_assigned(t) is at least n
    while n_assigned(ms, t) < n:
        t += 1
    while n_assigned(ms, t-1) >= n:
        t -= 1
    
    assert n_assigned(ms, t) >= n
    n_diff = n - n_assigned(ms, t-1)
    
    for (i, m) in enumerate(ms):
        if t % m == 0:
            n_diff -= 1
            if n_diff == 0:
                return i + 1
    
    return None

if __name__ == '__main__':
    t = int(input())
    for i in range(1, t+1):
        b, n = map(int, input().split(' '))
        ms = tuple(map(int, input().split(' ')))
        
        assert b == len(ms)
        result = haircut(ms, n)
        print('Case #{}: {}'.format(i, result))

# Small Start: 20:11
# Small End: 20:51
        
# def small(ms, b, n):
#     if n <= b:
#         return n
#     ends = list(ms)
#     n -= b
#     t = 0
#     
#     while n > 0:
#         t += 1
#         for (i, end) in enumerate(ends):
#             if end == t:
#                 n -= 1
#                 if n == 0:
#                     return i+1 # 1 indexed
#                 else:
#                     ends[i] = t + ms[i]
# 
# def large(ms, n):
#     #print('starting: {} {}'.format(ms, n))
#     b = len(ms)
#     if n <= b:
#         return n
#     
#     period = lcm_list(ms)
#     
#     per_period = 0
#     
#     for m in ms:
#         per_period += period // m
#     
#     n_mod = int(n % per_period)
#     if n_mod == 0:
#         n_mod = per_period
#     #print('n_mod: {}'.format(n_mod))
#     
#     return small(ms, b, n_mod)
#     
# 
# def gcd(a, b):
#     # Euclid's algorithm
#     while b:
#         a, b = b, a % b
#     
#     return a
# 
# def lcm(a, b):
#     # find product, divide by common factor
#     return (a * b) // gcd(a, b)
# 
# def lcm_list(xs):
#     if len(xs) == 0:
#         return None
#     if len(xs) == 1:
#         return xs[0]
#     
#     running_lcm = 1
#     ls = list(xs)
#     while ls:
#         running_lcm = lcm(running_lcm, ls.pop())
#     
#     return running_lcm
