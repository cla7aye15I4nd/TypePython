# Minimal test - try with immediate raise
# Note: Local variables inside try at module level not yet supported
try:
    raise Exception("test")
except:
    print(99)
