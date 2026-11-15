"""Run the examples of the README."""

import re
from pathlib import Path

import pytest

README = (Path(__file__).resolve().parents[1] / "README.md").read_text()
EXAMPLES = re.findall(r"```python\n(.*?)```", README, re.DOTALL)


@pytest.mark.parametrize("example", EXAMPLES)
def test_example(example: str) -> None:
    exec(compile(example, "README.md", "exec"), {})
