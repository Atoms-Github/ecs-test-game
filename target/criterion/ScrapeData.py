import os
import json

Results = {}

for X in os.listdir("."):
    if not os.path.isdir(X):
        continue

    for Y in os.listdir(X):
        if os.path.isdir(os.path.join(X, Y)) and not Y.isdigit() and Y != "report":
            for Z in os.listdir(os.path.join(X, Y)):
                if os.path.isdir(os.path.join(X, Y, Z)) and Z.isdigit():
                    estimates_file = os.path.join(X, Y, Z, "base", "estimates.json")
                    if os.path.isfile(estimates_file):
                        with open(estimates_file) as f:
                            J = json.load(f)
                            key = X + Y + Z
                            Results[key] = J["mean"]["point_estimate"]

with open("results.json", "w") as f:
    json.dump(Results, f)
