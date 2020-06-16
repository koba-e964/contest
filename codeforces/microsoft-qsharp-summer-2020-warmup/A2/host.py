import qsharp

from qsharp import Result
from Solution import TestI, TestZ

res = TestI.simulate(count=100)
print(res)
res = TestZ.simulate(count=100)
print(res)
