# Complex type inference tests: Mixed complex scenarios
# These tests combine multiple inference patterns in realistic scenarios

def test_builder_pattern() -> int:
    """Simulate builder pattern with progressive construction"""
    print("Test: builder pattern")
    builder = {}

    # Add components step by step
    builder[1] = []
    builder[1].append(10)
    builder[1].append(20)

    builder[2] = []
    builder[2].append(30)

    # builder: dict[int, list[int]]
    total: int = 0
    for key in builder:
        for value in builder[key]:
            total = total + value

    print("Total:", total)
    return total  # Expected: 60

def test_data_transformation_pipeline() -> int:
    """Multi-stage data transformation"""
    print("Test: transformation pipeline")

    # Stage 1: Raw data
    raw = [1, 2, 3, 4, 5]

    # Stage 2: Double each value
    doubled = []
    for val in raw:
        doubled.append(val * 2)

    # Stage 3: Group into pairs
    pairs = []
    i: int = 0
    while i < len(doubled):
        if i + 1 < len(doubled):
            pair = []
            pair.append(doubled[i])
            pair.append(doubled[i + 1])
            pairs.append(pair)
        i = i + 2

    # pairs: list[list[int]]
    # Access pairs[1][0] = doubled[2] = 6
    result: int = pairs[1][0]
    print("Result:", result)
    return result  # Expected: 6

def test_sparse_matrix() -> int:
    """Simulate sparse matrix with dict of dicts"""
    print("Test: sparse matrix")
    matrix = {}

    # Set specific cells
    row0 = {}
    row0[5] = 100
    matrix[0] = row0

    row3 = {}
    row3[2] = 200
    row3[7] = 300
    matrix[3] = row3

    # matrix: dict[int, dict[int, int]]
    result: int = matrix[3][7]
    print("Matrix[3][7]:", result)
    return result  # Expected: 300

def test_graph_adjacency_list() -> int:
    """Graph represented as adjacency list"""
    print("Test: graph adjacency list")
    graph = {}

    # Node 1 connects to 2, 3
    neighbors1 = []
    neighbors1.append(2)
    neighbors1.append(3)
    graph[1] = neighbors1

    # Node 2 connects to 3, 4
    neighbors2 = []
    neighbors2.append(3)
    neighbors2.append(4)
    graph[2] = neighbors2

    # graph: dict[int, list[int]]
    # Count edges from node 2
    edge_count: int = len(graph[2])
    print("Edges from node 2:", edge_count)
    return edge_count  # Expected: 2

def test_histogram_building() -> int:
    """Build histogram from data"""
    print("Test: histogram building")
    data = [1, 2, 2, 3, 3, 3, 4, 4, 4, 4]
    histogram = {}

    for value in data:
        # Check if key exists (use get-or-default pattern)
        if value in histogram:
            histogram[value] = histogram[value] + 1
        else:
            histogram[value] = 1

    # histogram: dict[int, int]
    freq_of_3: int = histogram[3]
    print("Frequency of 3:", freq_of_3)
    return freq_of_3  # Expected: 3

def test_nested_grouping() -> int:
    """Group data by multiple criteria"""
    print("Test: nested grouping")
    by_category = {}

    # Category 1
    cat1_items = []
    cat1_items.append(10)
    cat1_items.append(11)
    by_category[1] = cat1_items

    # Category 2
    cat2_items = []
    cat2_items.append(20)
    cat2_items.append(21)
    cat2_items.append(22)
    by_category[2] = cat2_items

    # Count total items
    total_items: int = 0
    for category in by_category:
        total_items = total_items + len(by_category[category])

    print("Total items:", total_items)
    return total_items  # Expected: 5

def test_state_machine_transitions() -> int:
    """State machine transition table"""
    print("Test: state machine")
    transitions = {}

    # State 0 transitions
    from_state_0 = {}
    from_state_0[1] = 1  # Input 1 -> State 1
    from_state_0[2] = 2  # Input 2 -> State 2
    transitions[0] = from_state_0

    # State 1 transitions
    from_state_1 = {}
    from_state_1[1] = 2
    from_state_1[2] = 0
    transitions[1] = from_state_1

    # transitions: dict[int, dict[int, int]]
    next_state: int = transitions[0][1]
    print("Next state:", next_state)
    return next_state  # Expected: 1

def test_cache_with_nested_data() -> int:
    """Cache storing complex nested structures"""
    print("Test: cache with nested data")
    cache = {}

    # Cache entry 1: list of values
    entry1 = []
    entry1.append(100)
    entry1.append(200)
    cache[1] = entry1

    # Cache entry 2: different list
    entry2 = []
    entry2.append(300)
    cache[2] = entry2

    # Lookup and sum
    sum_entry1: int = cache[1][0] + cache[1][1]
    print("Sum of entry 1:", sum_entry1)
    return sum_entry1  # Expected: 300

def test_event_log_processing() -> int:
    """Process event log with categorization"""
    print("Test: event log")
    events = [1, 2, 1, 3, 2, 1, 2, 3, 3]
    event_lists = {}

    # Initialize lists for each event type
    type1 = []
    type2 = []
    type3 = []
    event_lists[1] = type1
    event_lists[2] = type2
    event_lists[3] = type3

    # Track positions where each event occurs
    position: int = 0
    for event_type in events:
        event_lists[event_type].append(position)
        position = position + 1

    # event_lists: dict[int, list[int]]
    # Count occurrences of event type 1
    count_type1: int = len(event_lists[1])
    print("Type 1 count:", count_type1)
    return count_type1  # Expected: 3

def test_matrix_multiplication_prep() -> int:
    """Prepare data structures for matrix multiplication"""
    print("Test: matrix mult prep")

    # Matrix A: 2x3
    a = []
    row_a0 = [1, 2, 3]
    row_a1 = [4, 5, 6]
    a.append(row_a0)
    a.append(row_a1)

    # Matrix B: 3x2
    b = []
    row_b0 = [7, 8]
    row_b1 = [9, 10]
    row_b2 = [11, 12]
    b.append(row_b0)
    b.append(row_b1)
    b.append(row_b2)

    # Access element for computation: a[0][1] * b[1][0]
    result: int = a[0][1] * b[1][0]
    print("Product:", result)
    return result  # Expected: 18

def test_index_inversion() -> int:
    """Build inverted index structure"""
    print("Test: inverted index")
    # Documents containing words
    docs = []

    doc0 = []
    doc0.append(1)  # word id 1
    doc0.append(2)  # word id 2
    docs.append(doc0)

    doc1 = []
    doc1.append(2)
    doc1.append(3)
    docs.append(doc1)

    # Build inverted index: word -> list of doc ids
    inverted = {}

    doc_id: int = 0
    for doc in docs:
        for word_id in doc:
            if word_id in inverted:
                inverted[word_id].append(doc_id)
            else:
                new_list = []
                new_list.append(doc_id)
                inverted[word_id] = new_list
        doc_id = doc_id + 1

    # inverted: dict[int, list[int]]
    # Word 2 appears in how many docs?
    appear_count: int = len(inverted[2])
    print("Word 2 appears in docs:", appear_count)
    return appear_count  # Expected: 2

def test_recursive_structure_simulation() -> int:
    """Simulate recursive structure with explicit levels"""
    print("Test: recursive structure simulation")

    # Level 3: leaf values
    level3 = []
    level3.append(42)

    # Level 2: contains level 3
    level2 = []
    level2.append(level3)

    # Level 1: contains level 2
    level1 = []
    level1.append(level2)

    # Navigate down and modify
    level1[0][0].append(84)

    # Sum all values in deepest level
    total: int = level1[0][0][0] + level1[0][0][1]
    print("Total:", total)
    return total  # Expected: 126

def test_streaming_aggregation() -> int:
    """Simulate streaming data aggregation"""
    print("Test: streaming aggregation")
    stream = [1, 5, 3, 8, 2, 9, 4, 7, 6]
    buckets = {}

    # Bucket values: 0-3, 4-6, 7-9
    bucket_0_3 = []
    bucket_4_6 = []
    bucket_7_9 = []
    buckets[0] = bucket_0_3
    buckets[1] = bucket_4_6
    buckets[2] = bucket_7_9

    for value in stream:
        if value <= 3:
            buckets[0].append(value)
        elif value <= 6:
            buckets[1].append(value)
        else:
            buckets[2].append(value)

    # buckets: dict[int, list[int]]
    high_bucket_count: int = len(buckets[2])
    print("High bucket count:", high_bucket_count)
    return high_bucket_count  # Expected: 3
