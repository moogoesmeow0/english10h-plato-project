import matplotlib.pyplot as plt
import numpy as np
import csv
import math

fields = []
rows = []

with open("sitting.csv", "r") as csvfile:
    csvreader = csv.reader(csvfile)

    fields = next(csvreader)
    for row in csvreader:
        rows.append(row)

    print(f"total num of rows: {csvreader.line_num}")



x = []
y = []

for row in rows:
    for i, col in enumerate(row):
        if (i - 1) * 4 == 12:
            x.append(col)
        elif (i - 1) * 4 == 12:
            y.append(col)
            



fig, ax = plt.subplots()

assert(len(x) == len(y))

distances = []

for i in range(len(x)):
    distances.append(math.sqrt(x[i]^2 + y[i]^2))


ax.plot(distances, range(0, len(distances)))
plt.show()

