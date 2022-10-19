from time import time

from misaka import html


with open("../README.md", "r") as f:
    content = f.read()
    

times = 0

for i in range(10000):
    first = time()
    html(content)
    times += time() - first

print(times)
