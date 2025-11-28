# Main program to test deep imports with cyclic dependencies
# This imports module_a which triggers a chain of 20 interdependent modules

import module_a
import module_t

# Test calling functions from different modules
print("Testing deep import chain with cyclic dependencies...")
print("module_t.test() =", module_t.test())
print("module_a.test() =", module_a.test())
print("Deep import test completed successfully!")
