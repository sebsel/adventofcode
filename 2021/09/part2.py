f = open('input.txt', 'r')
chart = list(map(lambda line : line.strip(), f.readlines()))
max_x = len(chart)
max_y = len(chart[0])

def neighbors(x, y):
    return [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]

def on_map(points):
    (x, y) = points
    if x < 0: return False
    if y < 0: return False
    if x >= max_x: return False
    if y >= max_y: return False
    return True

groups = []
for x, row in enumerate(chart):
    for y, col in enumerate(row):
        if col == '9': continue

        group_found = False
        for neighbor in neighbors(x, y):
            if not on_map(neighbor) or chart[neighbor[0]][neighbor[1]] == '9':
                continue

            for i, group in enumerate(groups, 1):
                if neighbor in group:
                    if not group_found:
                        group_found = group
                        group.add((x, y))
                    elif group != group_found:
                        group_found.update(group)
                        groups.remove(group)

        if not group_found:
            groups.append({(x, y)})


# Sort the groups by length
groups.sort(key=len)
groups.reverse()


# Print the groups on a pretty map!
for x, row in enumerate(chart):
    for y, col in enumerate(row):
        c = 0
        for i, group in enumerate(groups, 1):
            if (x, y) in group:
                c = min(i, 4)
                print(f'\N{ESCAPE}[3{c}m{col}\N{ESCAPE}[37m', end ='')
                break
        if not c:
            print(col, end ='')
    print('')


# Get the answer
answer = 1
for group in groups[0:3]:
    answer *= len(group)

print('Answer:', answer)
