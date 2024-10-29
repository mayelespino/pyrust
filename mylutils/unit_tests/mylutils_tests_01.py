import unittest
import mylutils

class TestMylUtils(unittest.TestCase):

    def test_readtxt(self):
        lines = mylutils.read_txt("test.txt")
        self.assertEqual(len(lines), 10)

    def test_readcsv(self):
        lines = mylutils.read_csv("test.csv")
        self.assertEqual(len(lines), 4)
        for line in lines:
            self.assertTrue(len(line) > 1)

if __name__ == '__main__':
    unittest.main()
