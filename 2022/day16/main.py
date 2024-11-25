from os import environ
valves = None
with open('input.txt') as input:
    valves_info = input.read().split("\n")
    valves = {valve_info[6:8]:
              (int(valve_info[valve_info.find("=")+1: valve_info.find(";")]),
               [to for to in valve_info[valve_info.find("to valve")+(10 if "valves" in valve_info else 9):].split(", ")])
              for valve_info in valves_info}

print(valves)
print(1 if environ.get('part') == 'part2' else 2)

print(max_released)
