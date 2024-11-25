from os import environ


def flash(x, y, energy, flashed):
    flashed.append((x, y))
    ds = [-1, 0, 1]
    for dx in ds:
        for dy in ds:
            if (dx == 0 and dy == 0) or (x+dx, y+dy) in flashed or x+dx < 0 or x+dx > len(energy)-1 or y+dy < 0 or y+dy > len(energy[x+dx])-1:
                continue
            energy[x+dx][y+dy] += 1
            if energy[x+dx][y+dy] > 9:
                flash(x+dx, y+dy, energy, flashed)


with open('input.txt') as f:
    energy = [[int(d) for d in x if d != "\n"] for x in f.readlines()]
sum = 0
step = 0
while True:
    energy = [[e+1 for e in row] for row in energy]
    flashes = [(x, y) for x, row in enumerate(energy)
               for y, e in enumerate(row) if e > 9]
    flashed = []
    for (x, y) in flashes:
        if (x, y) not in flashed:
            flash(x, y, energy, flashed)
    sum += len(flashed)
    energy = [[0 if (x, y) in flashed else e for y, e in enumerate(row)]
              for x, row in enumerate(energy)]
    step += 1
    if environ.get('part') == "part1" and step == 100:
        print(sum)
        break
    elif len(flashed) == len(energy)*len(energy[0]):
        print(step)
        break
