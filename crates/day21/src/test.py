
def gcd(a, b):
    if a == 0:
        return b
    return gcd(b % a, a)

def lcm(a, b):
    return (a * b) / gcd(a, b)

print(gcd(1024, 64))
# print(lcm(1, 1))