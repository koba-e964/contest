import qsharp

from qsharp import Result
from Solution import TestZ, TestmZ

res = TestZ.simulate(count=100)
print(res)
res = TestmZ.simulate(count=100)
print(res)
