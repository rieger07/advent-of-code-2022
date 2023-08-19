from string import ascii_letters
from collections import Counter

with open("day3/input.in") as f:
	lines = f.readlines()

lookup = dict(zip(ascii_letters, range(1,53)))

part one
total = 0
for line in lines:
	first = set(line[0:len(line)//2])
	second = set(line[len(line)//2:-1])
	intersection = first.intersection(second).pop()
	total += lookup[intersection]
print(total)

groups = list()
group = set()
for idx, line in enumerate(lines, 1):

	group.add("".join(set(line.strip())))
	if idx % 3 == 0:
		groups.append(list(group))
		group = set()

total = 0
for group in groups:
	c = Counter([item for sublist in group for item in sublist])
	total += lookup[c.most_common()[0][0]]
print(total)