# Test raise and catch functionality

def test_raise():
    try:
        raise Exception("test error")
    except:
        print(1)
    print(2)

test_raise()
print(3)
