# Test iteration patterns in graph algorithms

# Graph represented as adjacency list
graph: dict[int, list[int]] = {
    0: [1, 2],
    1: [0, 3, 4],
    2: [0, 4],
    3: [1, 5],
    4: [1, 2, 5],
    5: [3, 4]
}

# BFS traversal
def bfs(graph: dict[int, list[int]], start: int) -> list[int]:
    visited: set[int] = set()
    queue: list[int] = [start]
    result: list[int] = []

    while len(queue) > 0:
        node: int = queue[0]
        queue = queue[1:]

        if node in visited:
            continue

        visited.add(node)
        result.append(node)

        for neighbor in graph[node]:
            if neighbor not in visited:
                queue.append(neighbor)

    return result

print(b"BFS from 0:", bfs(graph, 0))

# DFS traversal (iterative)
def dfs(graph: dict[int, list[int]], start: int) -> list[int]:
    visited: set[int] = set()
    stack: list[int] = [start]
    result: list[int] = []

    while len(stack) > 0:
        node: int = stack.pop()

        if node in visited:
            continue

        visited.add(node)
        result.append(node)

        for neighbor in reversed(graph[node]):
            if neighbor not in visited:
                stack.append(neighbor)

    return result

print(b"DFS from 0:", dfs(graph, 0))

# Find all paths between two nodes
def find_all_paths(graph: dict[int, list[int]], start: int, end: int) -> list[list[int]]:
    all_paths: list[list[int]] = []
    stack: list[tuple[int, list[int]]] = [(start, [start])]

    while len(stack) > 0:
        node, path = stack.pop()

        if node == end:
            all_paths.append(path)
            continue

        for neighbor in graph[node]:
            if neighbor not in path:
                new_path: list[int] = path[:]
                new_path.append(neighbor)
                stack.append((neighbor, new_path))

    return all_paths

print(b"All paths 0 to 5:", find_all_paths(graph, 0, 5))

# Check if path exists
def has_path(graph: dict[int, list[int]], start: int, end: int) -> bool:
    visited: set[int] = set()
    queue: list[int] = [start]

    while len(queue) > 0:
        node: int = queue[0]
        queue = queue[1:]

        if node == end:
            return True

        if node in visited:
            continue

        visited.add(node)

        for neighbor in graph[node]:
            if neighbor not in visited:
                queue.append(neighbor)

    return False

print(b"Path 0 to 5 exists:", has_path(graph, 0, 5))

# Shortest path (unweighted - BFS)
def shortest_path(graph: dict[int, list[int]], start: int, end: int) -> list[int]:
    visited: set[int] = set()
    queue: list[tuple[int, list[int]]] = [(start, [start])]

    while len(queue) > 0:
        node, path = queue[0]
        queue = queue[1:]

        if node == end:
            return path

        if node in visited:
            continue

        visited.add(node)

        for neighbor in graph[node]:
            if neighbor not in visited:
                new_path: list[int] = path[:]
                new_path.append(neighbor)
                queue.append((neighbor, new_path))

    return []

print(b"Shortest path 0 to 5:", shortest_path(graph, 0, 5))

# Count connected components
def count_components(graph: dict[int, list[int]]) -> int:
    visited: set[int] = set()
    count: int = 0

    for node in graph:
        if node not in visited:
            count = count + 1
            stack: list[int] = [node]
            while len(stack) > 0:
                current: int = stack.pop()
                if current in visited:
                    continue
                visited.add(current)
                for neighbor in graph[current]:
                    if neighbor not in visited:
                        stack.append(neighbor)

    return count

print(b"Connected components:", count_components(graph))

# Detect cycle (undirected graph)
def has_cycle(graph: dict[int, list[int]]) -> bool:
    visited: set[int] = set()

    for start in graph:
        if start in visited:
            continue

        stack: list[tuple[int, int]] = [(start, -1)]  # (node, parent)

        while len(stack) > 0:
            node, parent = stack.pop()

            if node in visited:
                return True

            visited.add(node)

            for neighbor in graph[node]:
                if neighbor != parent:
                    if neighbor in visited:
                        return True
                    stack.append((neighbor, node))

    return False

print(b"Has cycle:", has_cycle(graph))

# Topological sort (DAG)
dag: dict[int, list[int]] = {
    0: [1, 2],
    1: [3],
    2: [3, 4],
    3: [5],
    4: [5],
    5: []
}

def topological_sort(graph: dict[int, list[int]]) -> list[int]:
    in_degree: dict[int, int] = {}
    for node in graph:
        in_degree[node] = 0

    for node in graph:
        for neighbor in graph[node]:
            in_degree[neighbor] = in_degree.get(neighbor, 0) + 1

    queue: list[int] = []
    for node in in_degree:
        if in_degree[node] == 0:
            queue.append(node)

    result: list[int] = []
    while len(queue) > 0:
        node = queue[0]
        queue = queue[1:]
        result.append(node)

        for neighbor in graph[node]:
            in_degree[neighbor] = in_degree[neighbor] - 1
            if in_degree[neighbor] == 0:
                queue.append(neighbor)

    return result

print(b"Topological sort:", topological_sort(dag))

# Find all nodes at distance k
def nodes_at_distance(graph: dict[int, list[int]], start: int, k: int) -> list[int]:
    visited: set[int] = set()
    queue: list[tuple[int, int]] = [(start, 0)]
    result: list[int] = []

    while len(queue) > 0:
        node, dist = queue[0]
        queue = queue[1:]

        if node in visited:
            continue

        visited.add(node)

        if dist == k:
            result.append(node)
        elif dist < k:
            for neighbor in graph[node]:
                if neighbor not in visited:
                    queue.append((neighbor, dist + 1))

    return result

print(b"Nodes at distance 2 from 0:", nodes_at_distance(graph, 0, 2))

# Calculate degree of each node
def node_degrees(graph: dict[int, list[int]]) -> dict[int, int]:
    degrees: dict[int, int] = {}
    for node in graph:
        degrees[node] = len(graph[node])
    return degrees

print(b"Node degrees:", node_degrees(graph))
