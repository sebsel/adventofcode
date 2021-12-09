f = open('input.txt', 'r')
lines = list(map(lambda line : line.strip(), f.readlines()))
max_x = len(lines)
max_y = len(lines[0])

def is_lowest(x, y, lines):
    current = lines[x][y]
    if x - 1 >= 0 and lines[x - 1][y] <= current: return False
    if y - 1 >= 0 and lines[x][y - 1] <= current: return False
    if x + 1 < max_x and lines[x + 1][y] <= current: return False
    if y + 1 < max_y and lines[x][y + 1] <= current: return False
    return True

total = 0
for x, row in enumerate(lines):
    for y, col in enumerate(row):
        if is_lowest(x, y, lines) == True:
            total += 1 + int(col)

            print(f'\N{ESCAPE}[31m{col}\N{ESCAPE}[37m', end ='')
        else:
            print(col, end ='')
    print('')

print('Answer:', total)
