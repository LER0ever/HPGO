import sys
sys.path.append("../../contrib/torchgpipe/")

from random import random
import blockpartition

NUM_BLOCKS=10000
NUM_PARTITIONS=5

nums = []
for i in range(NUM_BLOCKS):
    nums.append(random())
print(nums)

res = blockpartition.solve(nums, NUM_PARTITIONS)
res_sum = []
for x in res:
    res_sum.append(sum(x))

print(res)
print(res_sum)
    