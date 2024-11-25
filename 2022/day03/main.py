from os import environ
with open('input.txt') as input:
    packings = input.read().split("\n")
    print(sum([sum([ord(item) - (38 if ord(item) < 91 else 96) for item in set(packings[i]) if ord(item) != 10 and item in packings[i+1] and item in packings[i+2]]) for i in range(0, len(packings)-2, 3)]) 
    if environ.get('part')== 'part2' else 
    sum([sum([ord(item) - (38 if ord(item) < 91 else 96) for item in set(packing[:len(packing)//2]) if item in packing[len(packing)//2:]]) for packing in packings]))