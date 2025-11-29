# Cannot right shift None and Dict[str, int]
x: None = None >> {"a": 1}
