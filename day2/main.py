from enum import IntEnum
class Choice(IntEnum):
	ROCK = 1
	PAPER = 2
	SCISS = 3

class Result(IntEnum):
	WIN = 6
	DRAW = 3
	LOSE = 0

def opponent_choice(character):
	if character == "A":
		return Choice.ROCK
	if character == "B":
		return Choice.PAPER
	if character == "C":
		return Choice.SCISS

def my_choice(character):
	if character == "X":
		return Choice.ROCK
	if character == "Y":
		return Choice.PAPER
	if character == "Z":
		return Choice.SCISS

def result(lhs: Choice, rhs: Choice) -> Result:
	if lhs == rhs:
		return Result.DRAW
	elif lhs == Choice.ROCK and rhs == Choice.PAPER:
		return Result.WIN
	elif lhs == Choice.PAPER and rhs == Choice.SCISS:
		return Result.WIN
	elif lhs == Choice.SCISS and rhs == Choice.ROCK:
		return Result.WIN
	else:
		return Result.LOSE

def get_result_from_str(character):
	if character == "X":
		return Result.LOSE
	if character == "Y":
		return Result.DRAW
	if character == "Z":
		return Result.WIN

def my_choice_based_on_result(lhs: Choice, rhs: Result) -> Choice:
	if rhs == Result.DRAW:
		return lhs
	elif rhs == Result.WIN:
		if lhs == Choice.ROCK:
			return Choice.PAPER
		elif lhs == Choice.PAPER:
			return Choice.SCISS
		else:
			return Choice.ROCK
	else:
		return my_choice_based_on_result(my_choice_based_on_result(lhs, Result.WIN),Result.WIN)

def score(result: Result, my_choice: Choice) -> int:
	return result + my_choice


with open("day2/input.in") as f:
	lines = f.readlines()

# part one
total = 0
for line in lines:
	temp = line.split()
	opponent = opponent_choice(temp[0])
	me = my_choice(temp[1])
	total += score(result(opponent, me), me)
print(total)

# part two
total = 0
for line in lines:
	temp = line.split()
	opponent = opponent_choice(temp[0])
	desired_result = get_result_from_str(temp[1])
	me = my_choice_based_on_result(opponent, desired_result)
	#print(opponent, me)
	total += score(result(opponent, me), me)
print(total)
