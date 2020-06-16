import qsharp

from qsharp import Result
from Solution import TestI, TestX

res = TestI.simulate(count=100)
print(res)
res = TestX.simulate(count=100)
print(res)
