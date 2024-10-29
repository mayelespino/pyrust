import unittest
import mylutils

class TestMylUtils(unittest.TestCase):

    def test_proc_read_stat_cpu(self):
        cpu_stats = mylutils.read_proc_stat_cpu()
        self.assertTrue('total_time' in cpu_stats)
        self.assertTrue('user_time' in cpu_stats)
        self.assertTrue('nice_time' in cpu_stats)
        self.assertTrue('system_time' in cpu_stats)
        self.assertTrue('idle_time' in cpu_stats)
        self.assertTrue('iowait_time' in cpu_stats)
        self.assertTrue('irq_time' in cpu_stats)
        self.assertTrue('softirq_time' in cpu_stats)
        self.assertTrue('iowait_time' in cpu_stats)

    def test_proc_read_stat_meminfo(self):
        meminfo = mylutils.read_proc_meminfo()
        self.assertTrue('MemAvailable' in meminfo)
        self.assertTrue('MemFree' in meminfo)
        self.assertTrue('Active' in meminfo)
        self.assertTrue('Inactive' in meminfo)
        self.assertTrue('Unevictable' in meminfo)

    def test_proc_read_vmstat(self):
        vmstat = mylutils.read_proc_vmstat()
        self.assertTrue('nr_unevictable' in vmstat)
        self.assertTrue('nr_mapped' in vmstat)
        self.assertTrue('nr_active_file' in vmstat)
        self.assertTrue('nr_active_anon' in vmstat)
        self.assertTrue('nr_written' in vmstat)
        self.assertTrue('nr_kernel_stack' in vmstat)
        self.assertTrue('nr_zone_inactive_file' in vmstat)
        self.assertTrue('pgfree' in vmstat)
        self.assertTrue('pgfault' in vmstat)
        self.assertTrue('pglazyfree' in vmstat)
        self.assertTrue('pgpgin' in vmstat)
        self.assertTrue('pgpgout' in vmstat)
        self.assertTrue('pgreuse' in vmstat)
        self.assertTrue('pgreuse' in vmstat)
        self.assertTrue('pgalloc_normal' in vmstat)
        self.assertTrue('numa_hit' in vmstat)
        self.assertTrue('numa_local' in vmstat)


if __name__ == '__main__':
    unittest.main()
