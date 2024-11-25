from os import environ


def shoot(vx, vy, x_target, y_target):
    x, y, max_y = 0, 0, 0
    while x < x_target[1] and y > y_target[0]:
        x += vx
        y += vy
        max_y = y if y > max_y else max_y
        if x >= x_target[0] and x <= x_target[1] and y >= y_target[0] and y <= y_target[1]:
            return max_y
        vx = vx - 1 if vx > 0 else vx+1 if vx < 0 else 0
        vy -= 1
    return None


with open('input.txt') as f:
    target_str = f.read().split(": ")[1].split(", ")
    x_target = sorted([int(x)
                      for x in target_str[0].replace("x=", "").split("..")])
    y_target = sorted([int(y)
                      for y in target_str[1].replace("y=", "").split("..")])

max_y, hits = 0, 0
for vx in range(max(x_target)+1):
    for vy in range(min(y_target), abs(min(y_target))):
        y = shoot(vx, vy, x_target, y_target)
        if y is not None:
            hits += 1
            if y > max_y:
                max_y = y

if environ.get('part') == "part1":
    print(max_y)
else:
    print(hits)
