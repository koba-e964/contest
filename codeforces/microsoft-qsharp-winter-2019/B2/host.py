import qsharp

from qsharp import Result
from Solution import Test

for k in range(3):
    res = Test.simulate(count=100, kind=k)
    print(k, res)
