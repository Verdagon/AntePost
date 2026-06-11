from dataclasses import dataclass

class Empty:
    pass

@dataclass
class Node:
    color: str
    left: object
    val: object
    right: object

def balance(t):
    match t:
        case Node('B', Node('R', Node('R', a, x, b), y, c), z, d) \
           | Node('B', Node('R', a, x, Node('R', b, y, c)), z, d) \
           | Node('B', a, x, Node('R', Node('R', b, y, c), z, d)) \
           | Node('B', a, x, Node('R', b, y, Node('R', c, z, d))):
            return Node('R', Node('B', a, x, b), y, Node('B', c, z, d))
        case other:
            return other

def insert(tree, val):
    def ins(t):
        match t:
            case Empty():
                return Node('R', Empty(), val, Empty())
            case Node(color, left, v, right) if val < v:
                return balance(Node(color, ins(left), v, right))
            case Node(color, left, v, right) if val > v:
                return balance(Node(color, left, v, ins(right)))
            case _:
                return t  # already exists

    result = ins(tree)
    result.color = 'B'  # root is always black
    return result

def contains(tree, val):
    match tree:
        case Empty():
            return False
        case Node(_, left, v, right):
            if val < v:
                return contains(left, val)
            elif val > v:
                return contains(right, val)
            else:
                return True

def to_sorted_list(tree):
    match tree:
        case Empty():
            return []
        case Node(_, left, v, right):
            return to_sorted_list(left) + [v] + to_sorted_list(right)

def print_tree(tree, prefix="", is_left=True):
    match tree:
        case Empty():
            return
        case Node(color, left, v, right):
            connector = "├── " if is_left else "└── "
            if prefix:
                print(f"{prefix}{connector}{v}({color})")
            else:
                print(f"{v}({color})")
            child_prefix = prefix + ("│   " if is_left else "    ")
            if not isinstance(left, Empty) or not isinstance(right, Empty):
                print_tree(left, child_prefix, True)
                print_tree(right, child_prefix, False)

if __name__ == "__main__":
    tree = Empty()

    values = [7, 3, 18, 10, 22, 8, 11, 26, 2, 6, 13]
    print("Inserting:", values)
    for v in values:
        tree = insert(tree, v)

    print("\nTree structure:")
    print_tree(tree)

    print("\nSorted:", to_sorted_list(tree))

    for v in [10, 5, 22]:
        print(f"contains({v}) = {contains(tree, v)}")

    print("\n--- Empty tree ---")
    empty = Empty()
    assert to_sorted_list(empty) == []
    assert contains(empty, 1) == False
    print("Empty tree: OK")

    print("\n--- Single element ---")
    single = insert(Empty(), 42)
    assert to_sorted_list(single) == [42]
    assert contains(single, 42) == True
    assert contains(single, 0) == False
    assert single.color == 'B'
    print("Single element: OK")

    print("\n--- Duplicates ---")
    t = Empty()
    for v in [5, 5, 5, 3, 3]:
        t = insert(t, v)
    assert to_sorted_list(t) == [3, 5]
    print("Duplicates ignored: OK")

    print("\n--- Ascending insert 1..20 ---")
    t = Empty()
    for v in range(1, 21):
        t = insert(t, v)
    assert to_sorted_list(t) == list(range(1, 21))
    print("Ascending: OK")

    print("\n--- Descending insert 20..1 ---")
    t = Empty()
    for v in range(20, 0, -1):
        t = insert(t, v)
    assert to_sorted_list(t) == list(range(1, 21))
    print("Descending: OK")

    print("\n--- Checking RB invariants on a large tree ---")
    import random
    random.seed(42)
    vals = random.sample(range(1000), 200)
    t = Empty()
    for v in vals:
        t = insert(t, v)
    assert to_sorted_list(t) == sorted(vals)

    def check_invariants(tree):
        match tree:
            case Empty():
                return 1
            case Node(color, left, val, right):
                if color == 'R':
                    if isinstance(left, Node) and left.color == 'R':
                        raise AssertionError(f"Red node {val} has red left child")
                    if isinstance(right, Node) and right.color == 'R':
                        raise AssertionError(f"Red node {val} has red right child")
                lh = check_invariants(left)
                rh = check_invariants(right)
                if lh != rh:
                    raise AssertionError(f"Black-height mismatch at {val}: {lh} vs {rh}")
                return lh + (1 if color == 'B' else 0)

    assert t.color == 'B', "Root must be black"
    bh = check_invariants(t)
    print(f"200-element tree: OK (black-height={bh})")

    print("\nAll tests passed!")
