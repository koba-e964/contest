import qsharp

from qsharp import Result
from Solution import TestI, TestS

res = TestI.simulate(count=100)
print(res)
res = TestS.simulate(count=100)
print(res)
