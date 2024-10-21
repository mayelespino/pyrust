import mylutils

print("\nmylutils.read_txt(test.txt)\n")

for line in mylutils.read_txt("test.txt"):
   print(line)

print("\nmylutils.read_csv(test.csv)\n")
for line in mylutils.read_csv("test.csv"):
   for col in line:
       print(col,end=",")
   print()


print("\nmylutils.read_proc_stat_cpu()\n")

cpu_stats = mylutils.read_proc_stat_cpu()

total_time = cpu_stats['total_time']
user_time = cpu_stats['user_time']
nice_time = cpu_stats['nice_time']
system_time = cpu_stats['system_time']
idle_time = cpu_stats['idle_time']
iowait_time = cpu_stats['iowait_time']
irq_time = cpu_stats['irq_time']
softirq_time = cpu_stats['softirq_time']
iowait_time = cpu_stats['iowait_time']

print("total_time", total_time)
print("")
print("user_time", user_time)
print("nice_time", nice_time)
print("system_time", system_time)
print("idle_time", idle_time)
print("iowait_time", iowait_time)
print("irq_time", irq_time)
print("softirq_time", softirq_time)
print("")
print("user_percent", (user_time/total_time)*100)
print("nice_percent", (nice_time/total_time)*100)
print("system_percent", (system_time/total_time)*100)
print("idle_percent", (idle_time/total_time)*100)
print("iowait_percent", (iowait_time/total_time)*100)
print("irq_percent", (irq_time/total_time)*100)
print("softirq_percent", (softirq_time/total_time)*100)

def is_cpu_idle(threshold=50):
    cpu_stats = mylutils.read_proc_stat_cpu()
    return((idle_time/total_time)*100 < threshold)

print(f"\nis_cpu_idle(99): {is_cpu_idle(99)}", )
