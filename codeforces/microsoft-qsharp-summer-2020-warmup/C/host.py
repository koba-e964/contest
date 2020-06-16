import qsharp

from qsharp import Result
from Solution import Test

res = Test.simulate(count=100)
print(res)
