import requests
import sys

if len(sys.argv) != 4:
    raise ValueError(
        "Error: Missing Parameters. Command should follow: python3 fetchInputs.py <folder-path> <year> <cookie>"
    )

folder = sys.argv[1]
year = sys.argv[2]
cookie = sys.argv[3]

headers = {"cookie": cookie}

for i in range(25):
    day = i + 1
    space = " " if day < 10 else ""
    print("Retrieving " + str(year) + "-" + str(day) + ": " + space, end="")
    url = "https://adventofcode.com/" + str(year) + "/day/" + str(day) + "/input"
    x = requests.get(url, headers=headers)
    if x.ok:
        print("✅")
        f = open(folder + "/" + str(day) + ".txt", "w")
        f.write(x.text)
        f.close()
    else:
        print("❌", x.reason)
