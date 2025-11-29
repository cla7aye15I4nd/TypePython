# Cannot right shift None and Set[int]
x: None = None >> {1, 2, 3}
