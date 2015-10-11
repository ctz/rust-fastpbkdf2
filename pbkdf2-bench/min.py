
all = []

for i in range(1, 6):
    lines = open('out.%d' % i).readlines()
    lines = lines[1:]
    lines = [l.strip() for l in lines]
    all.append(lines)

best = []

for i in range(len(all[0])):
    here = [x[i] for x in all]
    here.sort()
    best.append(here[0])

def extract(x):
    name, val = x.split(' = ')
    val = val.replace('ms', '')
    return int(val)

def diff(x, y):
    return float(extract(y)) / extract(x)

def compare(b, i):
    if i < 3:
        return ''
    if i < 6:
        return diff(best[i - 3], best[i])
    if i < 9:
        return diff(best[i - 6], best[i])

for i, b in enumerate(best):
    print b, compare(best, i)
