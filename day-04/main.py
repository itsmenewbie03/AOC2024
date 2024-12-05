from typing import TypeAlias

Point: TypeAlias = tuple[int, int]
Word: TypeAlias = list[Point]


def is_horizontal(word: Word) -> bool:
    """Checks if a word is horizontal."""
    y_coord = word[0][1]
    return all(point[1] == y_coord for point in word)


def is_vertical(word: Word) -> bool:
    """Checks if a word is vertical."""
    x_coord = word[0][0]
    return all(point[0] == x_coord for point in word)


def get_distance(a: Point, b: Point):
    return abs(a[0] - b[0]) + abs(a[1] - b[1])


def is_valid_pair(a: Word, b: Word):
    return a[1] == b[1] and get_distance(a[0], b[0]) == 2


def find_pairs(valid_idx: list[list[tuple[int, int]]]):
    diag_idx = list(
        filter(lambda x: not is_horizontal(x) and not is_vertical(x), valid_idx)
    )
    pair_count = 0
    for idx_list in diag_idx:
        pair = next(
            (points for points in diag_idx if is_valid_pair(idx_list, points)),
            None,
        )
        if pair:
            pair_count += 1
    return pair_count // 2


def grid_search(grid, query):
    valid_idx: list[list[tuple[int, int]]] = []
    rows, cols = len(grid), len(grid[0])
    directions = [
        (0, 1),  # HORIZONTAL
        (0, -1),  # HORIZONTAL
        (1, 0),  # VERTICAL
        (-1, 0),  # VERTICAL
        (1, 1),  # DIAGONAL
        (-1, -1),  # DIAGONAL
        (1, -1),  # ANTI-DIAGONAL
        (-1, 1),  # ANTI-DIAGONAL
    ]

    def is_valid(r, c):
        """
        Check for range validity
        """
        return 0 <= r < rows and 0 <= c < cols

    for row in range(rows):
        for col in range(cols):
            # Check for start
            if grid[row][col] == query[0]:
                for direction in directions:
                    temp: list[tuple[int, int]] = []
                    for idx, char in enumerate(query):
                        r, c = row + direction[0] * idx, col + direction[1] * idx
                        if not is_valid(r, c):
                            break
                        if grid[r][c] == char:
                            temp.append((r, c))
                    if len(temp) == len(query):
                        valid_idx.append(temp)

    return valid_idx


def redact(grid, valid_idx: list[list[tuple[int, int]]]):
    rows, cols = len(grid), len(grid[0])
    for row in range(rows):
        for col in range(cols):
            if any((row, col) in idx_list for idx_list in valid_idx):
                print(grid[row][col], end="")
            else:
                print(".", end="")
        print()


content = open("input.txt", "r").read().strip().splitlines()
valid_idx = grid_search(content, "MAS")
print(len(valid_idx))
print(find_pairs(valid_idx))
