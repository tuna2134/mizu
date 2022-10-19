from time import time

from pulldown_py import parser


with open("../README.md", "r") as f:
    content = f.read()
    

times = 0

for i in range(10000):
    first = time()
    parser(content)
    times += time() - first

print(times)
