# bytearray() with too many arguments
def main() -> None:
    ba = bytearray(b'hello', b'world')  # bytearray() takes at most 1 argument
