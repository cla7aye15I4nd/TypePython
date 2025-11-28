# Main program to test deep imports with cyclic dependencies
# This imports module_a which triggers a chain of 20 interdependent modules

import module_a
import module_t

# Test calling functions from different modules
print(b"Testing deep import chain with cyclic dependencies...")
print(b"module_t.test() =", module_t.test())
print(b"module_a.test() =", module_a.test())
print(b"Deep import test completed successfully!")
