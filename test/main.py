# Main test runner for PyCompiler
import basic.test_runner as basic_test
import imports.test_runner as imports_test
import algorithm.test_runner as algorithm_test
import datastructure.test_runner as datastructure_test
import stresstest.test_runner as stresstest_test
import rng.test_runner as rng_test
import inheritance.test_runner as inheritance_test
import exceptions.test_runner as exceptions_test

print("Running PyCompiler tests...", -1, 0xFF, True, False, [1, 2], b'11')
basic_test.test()
imports_test.test()
algorithm_test.test()
datastructure_test.test()
stresstest_test.test()
rng_test.test()
inheritance_test.test()
exceptions_test.test()
