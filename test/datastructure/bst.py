# Binary Search Tree implementation

class TreeNode:
    value: int
    has_left: int
    has_right: int
    left_idx: int   # Index in nodes list
    right_idx: int  # Index in nodes list

    def __init__(self, val: int) -> None:
        self.value = val
        self.has_left = 0
        self.has_right = 0
        self.left_idx = 0
        self.right_idx = 0
        return


class BinaryTree:
    nodes: list[TreeNode]
    size: int
    has_root: int

    def __init__(self) -> None:
        dummy: TreeNode = TreeNode(0)
        self.nodes = [dummy]
        self.size = 0
        self.has_root = 0

    def insert(self, value: int) -> None:
        node: TreeNode = TreeNode(value)
        if self.size == 0:
            self.nodes[0] = node
        else:
            self.nodes.append(node)
        new_idx: int = self.size
        self.size = self.size + 1

        if self.has_root == 0:
            self.has_root = 1
        else:
            # Find position in tree
            curr_idx: int = 0
            done: int = 0
            while done == 0:
                curr: TreeNode = self.nodes[curr_idx]
                if value < curr.value:
                    if curr.has_left == 1:
                        curr_idx = curr.left_idx
                    else:
                        curr.has_left = 1
                        curr.left_idx = new_idx
                        done = 1
                else:
                    if curr.has_right == 1:
                        curr_idx = curr.right_idx
                    else:
                        curr.has_right = 1
                        curr.right_idx = new_idx
                        done = 1

    def contains(self, value: int) -> int:
        if self.has_root == 0:
            return 0

        curr_idx: int = 0
        done: int = 0
        found: int = 0
        while done == 0:
            curr: TreeNode = self.nodes[curr_idx]
            if curr.value == value:
                found = 1
                done = 1
            else:
                if value < curr.value:
                    if curr.has_left == 1:
                        curr_idx = curr.left_idx
                    else:
                        done = 1
                else:
                    if curr.has_right == 1:
                        curr_idx = curr.right_idx
                    else:
                        done = 1
        return found

    def get_size(self) -> int:
        return self.size


def test_bst_insert() -> int:
    tree: BinaryTree = BinaryTree()
    tree.insert(50)
    tree.insert(30)
    tree.insert(70)
    tree.insert(20)
    tree.insert(40)
    return tree.get_size()  # Expected: 5


def test_bst_contains() -> int:
    tree: BinaryTree = BinaryTree()
    tree.insert(50)
    tree.insert(30)
    tree.insert(70)
    tree.insert(20)
    tree.insert(40)
    result: int = tree.contains(30) + tree.contains(70) + tree.contains(99)
    return result  # Expected: 2
