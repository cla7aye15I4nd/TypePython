# Exception() with too many arguments
def main() -> None:
    e = Exception("msg1", "msg2")  # Exception() takes at most 1 argument
