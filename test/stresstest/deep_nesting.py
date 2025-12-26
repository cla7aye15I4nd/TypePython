# Deep nesting tests - complex nested class/list structures

# Level 1: Basic class
class Item:
    value: int

    def __init__(self, v: int) -> None:
        self.value = v

    def get_value(self) -> int:
        return self.value


# Level 2: Class containing Item
class Box:
    item: Item
    label: int

    def __init__(self, v: int, lbl: int) -> None:
        self.item = Item(v)
        self.label = lbl

    def get_item_value(self) -> int:
        return self.item.value


# Level 3: Class containing Box (3-level nesting: Container->Box->Item)
class Container:
    box: Box
    id: int

    def __init__(self, v: int, lbl: int, cid: int) -> None:
        self.box = Box(v, lbl)
        self.id = cid

    def get_deep_value(self) -> int:
        return self.box.item.value


# Class with list field
class Inventory:
    items: list[Item]
    count: int

    def __init__(self) -> None:
        dummy: Item = Item(0)
        self.items = [dummy]
        self.count = 0

    def add_item(self, v: int) -> None:
        item: Item = Item(v)
        if self.count == 0:
            self.items[0] = item
        else:
            self.items.append(item)
        self.count = self.count + 1

    def get_item_value(self, idx: int) -> int:
        return self.items[idx].value

    def sum_all(self) -> int:
        total: int = 0
        i: int = 0
        while i < self.count:
            total = total + self.items[i].value
            i = i + 1
        return total


# Class with list of classes that contain classes (list[Box] where Box has Item)
class Warehouse:
    boxes: list[Box]
    size: int

    def __init__(self) -> None:
        dummy: Box = Box(0, 0)
        self.boxes = [dummy]
        self.size = 0

    def add_box(self, v: int, lbl: int) -> None:
        b: Box = Box(v, lbl)
        if self.size == 0:
            self.boxes[0] = b
        else:
            self.boxes.append(b)
        self.size = self.size + 1

    def get_box_item_value(self, idx: int) -> int:
        return self.boxes[idx].item.value

    def sum_labels(self) -> int:
        total: int = 0
        i: int = 0
        while i < self.size:
            total = total + self.boxes[i].label
            i = i + 1
        return total


# Very deep: Class containing list of Container (list[Container] where Container->Box->Item)
class Storage:
    containers: list[Container]
    num: int

    def __init__(self) -> None:
        dummy: Container = Container(0, 0, 0)
        self.containers = [dummy]
        self.num = 0

    def add_container(self, v: int, lbl: int, cid: int) -> None:
        c: Container = Container(v, lbl, cid)
        if self.num == 0:
            self.containers[0] = c
        else:
            self.containers.append(c)
        self.num = self.num + 1

    def get_deepest_value(self, idx: int) -> int:
        # Access: containers[idx].box.item.value (4 levels!)
        return self.containers[idx].box.item.value


# Outer list of class containing list: list[Inventory] where Inventory has list[Item]
class Company:
    inventories: list[Inventory]
    inv_count: int

    def __init__(self) -> None:
        dummy: Inventory = Inventory()
        self.inventories = [dummy]
        self.inv_count = 0

    def add_inventory(self) -> int:
        inv: Inventory = Inventory()
        if self.inv_count == 0:
            self.inventories[0] = inv
        else:
            self.inventories.append(inv)
        idx: int = self.inv_count
        self.inv_count = self.inv_count + 1
        return idx

    def add_item_to_inventory(self, inv_idx: int, value: int) -> None:
        self.inventories[inv_idx].add_item(value)

    def get_item_from_inventory(self, inv_idx: int, item_idx: int) -> int:
        # Access: inventories[inv_idx].items[item_idx].value
        return self.inventories[inv_idx].items[item_idx].value


# ============================================================================
# Test functions
# ============================================================================

def test_three_level_nesting() -> int:
    # Container -> Box -> Item
    c: Container = Container(42, 10, 1)
    return c.box.item.value  # Expected: 42


def test_three_level_assign() -> int:
    # Modify deepest nested field
    c: Container = Container(1, 2, 3)
    c.box.item.value = 99
    return c.box.item.value  # Expected: 99


def test_three_level_method() -> int:
    # Method that accesses 3-level nesting
    c: Container = Container(77, 5, 1)
    return c.get_deep_value()  # Expected: 77


def test_class_with_list_field() -> int:
    # Inventory has list[Item]
    inv: Inventory = Inventory()
    inv.add_item(10)
    inv.add_item(20)
    inv.add_item(30)
    return inv.sum_all()  # Expected: 60


def test_list_in_class_access() -> int:
    # Access element in class's list field
    inv: Inventory = Inventory()
    inv.add_item(5)
    inv.add_item(15)
    inv.add_item(25)
    return inv.get_item_value(1)  # Expected: 15


def test_list_in_class_modify() -> int:
    # Modify element in class's list field
    inv: Inventory = Inventory()
    inv.add_item(100)
    inv.items[0].value = 200
    return inv.items[0].value  # Expected: 200


def test_list_of_nested_class() -> int:
    # list[Box] where Box contains Item
    wh: Warehouse = Warehouse()
    wh.add_box(11, 1)
    wh.add_box(22, 2)
    wh.add_box(33, 3)
    return wh.get_box_item_value(1)  # Expected: 22


def test_list_of_nested_modify() -> int:
    # Modify deeply nested: boxes[i].item.value
    wh: Warehouse = Warehouse()
    wh.add_box(1, 10)
    wh.boxes[0].item.value = 999
    return wh.boxes[0].item.value  # Expected: 999


def test_four_level_access() -> int:
    # Storage -> containers[i] -> box -> item -> value
    st: Storage = Storage()
    st.add_container(111, 1, 1)
    st.add_container(222, 2, 2)
    return st.get_deepest_value(1)  # Expected: 222


def test_four_level_modify() -> int:
    # Modify at 4th level: containers[i].box.item.value = x
    st: Storage = Storage()
    st.add_container(1, 1, 1)
    st.containers[0].box.item.value = 888
    return st.containers[0].box.item.value  # Expected: 888


def test_list_in_list_in_class() -> int:
    # Company -> inventories[i] -> items[j] -> value
    co: Company = Company()
    idx0: int = co.add_inventory()
    idx1: int = co.add_inventory()
    co.add_item_to_inventory(idx0, 10)
    co.add_item_to_inventory(idx0, 20)
    co.add_item_to_inventory(idx1, 30)
    co.add_item_to_inventory(idx1, 40)
    # Access inventories[1].items[1].value
    return co.get_item_from_inventory(1, 1)  # Expected: 40


def test_list_in_list_modify() -> int:
    # Modify: inventories[i].items[j].value = x
    co: Company = Company()
    idx: int = co.add_inventory()
    co.add_item_to_inventory(idx, 5)
    co.inventories[0].items[0].value = 777
    return co.inventories[0].items[0].value  # Expected: 777


def test_complex_expression() -> int:
    # Complex arithmetic with multiple nested accesses
    c1: Container = Container(10, 1, 1)
    c2: Container = Container(20, 2, 2)
    # (c1.box.item.value + c2.box.item.value) * 2
    result: int = c1.box.item.value + c2.box.item.value
    return result + result  # Expected: 60


def test_nested_loop_access() -> int:
    # Loop through list, summing nested values
    wh: Warehouse = Warehouse()
    wh.add_box(10, 1)
    wh.add_box(20, 2)
    wh.add_box(30, 3)
    total: int = 0
    i: int = 0
    while i < wh.size:
        total = total + wh.boxes[i].item.value
        i = i + 1
    return total  # Expected: 60


def test_modify_in_loop() -> int:
    # Modify nested values in a loop
    inv: Inventory = Inventory()
    inv.add_item(1)
    inv.add_item(2)
    inv.add_item(3)
    i: int = 0
    while i < inv.count:
        inv.items[i].value = inv.items[i].value * 10
        i = i + 1
    return inv.sum_all()  # Expected: 60 (10+20+30)
