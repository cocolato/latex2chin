"""Smoke tests for the Python binding layer."""

from latex2chin import parse_latex

# Basic translation
assert parse_latex(r"\frac{1}{2}") == "2分之1", "frac failed"
assert parse_latex(r"\sqrt{2}") == "2的平方根", "sqrt failed"
assert parse_latex(r"\sqrt[3]{8}") == "8的立方根", "sqrt[3] failed"
assert parse_latex(r"\pi \approx 3.14") == "派约等于3.14", "pi approx failed"
assert parse_latex(r"x^2 + y^2 = 1") == "x的平方加y的平方等于1", "superscript failed"
assert parse_latex(r"\sin x") == "sinx", "sin failed"
assert parse_latex(r"a_1") == "a1", "subscript failed"

# Error handling: invalid input must raise ValueError
try:
    parse_latex("")
    raise AssertionError("empty input should raise ValueError")
except ValueError:
    pass

try:
    parse_latex("(1 + 2")
    raise AssertionError("unclosed paren should raise ValueError")
except ValueError:
    pass

print("All smoke tests passed.")
