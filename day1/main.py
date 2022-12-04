import math

with open("./day1/input.in") as f:
	index = 0
	lines = f.readlines()

elf_calories = [0 for _ in range(lines.count("\n")+1)]
for line in lines:
	if line != "\n":
		elf_calories[index] += int(line)
	else:
		index += 1
# solution to part one
print(max(elf_calories))
# solution to part two
elf_calories.sort(reverse=True)
print(elf_calories)
print(sum(elf_calories[0:3]))