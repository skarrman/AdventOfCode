import cpmpy as cp
from os import environ
import re

def parse_input():
    pattern = r"^(-?\d+),\s+(-?\d+),\s+(-?\d+)\s+@\s+(-?\d+),\s+(-?\d+),\s+(-?\d+)$"
    hailstones = []
    with open("input.txt") as file:
        for line in file.readlines():
            (x, y, z, dx, dy, dz) = re.match(pattern, line).groups()
            hailstones.append((int(x), int(y), int(z), int(dx), int(dy), int(dz)))
    return hailstones

def solve_first(hailstones, min, max):
    intersections = 0
    for (i, (x1, y1, _, dx1, dy1, _)) in enumerate(hailstones[:-1]):
        for (x2,y2,_, dx2,dy2,_) in hailstones[i+1:]:
            if dx1 * dy2 == dy1 * dx2:
                continue
            
            t2 = (dx1 * y1 + dy1 * x2 - dy1 * x1 - dx1 * y2) / (dx1 * dy2 - dy1 * dx2)
            t1 = (x2 + dx2 * t2 - x1) / dx1
            x = x1 + dx1 * t1
            y = y1 + dy1 * t1

            if 0 <= t1 and 0 <= t2 and min <= x and x <= max and min <= y and y <= max:
                intersections += 1
    return intersections

def min_vec(v):
    return sum(v)

def solve_second(hailstones):
    MAX = 2**64
    MAX_DIR = 2**64
    MAX_TIME = 2**64

    x = cp.IntVar(name="x", lb=-MAX, ub=MAX)
    y = cp.IntVar(name="y", lb=-MAX, ub=MAX)
    z = cp.IntVar(name="z", lb=-MAX, ub=MAX)
    dx = cp.IntVar(name="dx", lb=-MAX_DIR, ub=MAX_DIR)
    dy = cp.IntVar(name="dy", lb=-MAX_DIR, ub=MAX_DIR)
    dz = cp.IntVar(name="dz", lb=-MAX_DIR, ub=MAX)

    model = cp.Model()
    for (i, (px, py, pz, vx, vy, vz)) in enumerate(hailstones[:3]):
        ti = cp.IntVar(name=f't_{i}', lb=0, ub=MAX_TIME)
        model += x + dx * ti == px + vx * ti,
        model += y + dy * ti == py + vy * ti,
        model += z + dz * ti == pz + vz * ti,

    model.solve(solver="z3")
    
    return x.value() + y.value()+ z.value()

input = parse_input()

if environ.get('part') == 'part2':
    print(solve_second(input))
else:
    print(solve_first(input, 200000000000000, 400000000000000))
