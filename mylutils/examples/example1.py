import mylutils

print("\nmylutils.read_txt(test.txt)\n")

for line in mylutils.read_txt("test.txt"):
   print(line)

print("\nmylutils.read_csv(test.csv)\n")
for line in mylutils.read_csv("test.csv"):
   for col in line:
       print(col,end=",")
   print()
