from os import environ
import numpy as np

with open('input.txt') as f:
    contents = f.read()

trans = np.dot(
    np.array([-618, -824, -621]), np.array([686, 422, 578]))
print(trans)


if environ.get('part') == "part1":
    print(1)
