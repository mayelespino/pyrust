import mylutils

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

print("\nmylutils.read_proc_meminfo()")
meminfo = mylutils.read_proc_meminfo()
for i in meminfo:
    print(i, meminfo[i])

print("\nCalculations ")
available = int(meminfo['MemAvailable'].replace("kB",""))
memfree = int(meminfo['MemFree'].replace("kB",""))
active = int(meminfo['Active'].replace("kB",""))
active_file = int(meminfo['Active(file)'].replace("kB",""))
inactive = int(meminfo['Inactive'].replace("kB",""))
unevictable = int(meminfo['Unevictable'].replace("kB",""))

print("Active memory percent", (active/available)*100 )
print("Inactive memory percent", (inactive/available)*100 )
print("Active(file) memory percent", (active_file/available)*100 )
print("MemAvailable percent", (memfree/available)*100 )
print("Unevictable percent", (unevictable/available)*100 )


print("\nmylutils.read_proc_vmstat()")
vmstat = mylutils.read_proc_vmstat()
for i in vmstat:
    print(i, vmstat[i])
