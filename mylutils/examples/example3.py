import mylutils

cpu_stats = mylutils.read_proc_stat_cpu()

def is_cpu_idle(threshold=50):    
    total_time = cpu_stats['total_time']
    idle_time = cpu_stats['idle_time']
    return((idle_time/total_time)*100 >= threshold)

def is_cpu_user(threshold=50):    
    total_time = cpu_stats['total_time']
    user_time = cpu_stats['user_time']
    return((user_time/total_time)*100 >= threshold)

def is_cpu_system(threshold=50):    
    total_time = cpu_stats['total_time']
    system_time = cpu_stats['system_time']
    return((system_time/total_time)*100 >= threshold)

def is_cpu_iowait(threshold=50):    
    total_time = cpu_stats['total_time']
    iowait_time = cpu_stats['iowait_time']
    return((iowait_time/total_time)*100 >= threshold)

print(f"\nis_cpu_idle(25)): {is_cpu_idle(25)}", )
print(f"\nis_cpu_user(50): {is_cpu_user(50)}", )
print(f"\nis_cpu_system(50): {is_cpu_system(50)}", )
print(f"\nis_cpu_iowait(5): {is_cpu_iowait(5)}", )