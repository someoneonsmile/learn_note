#!/user/bin/python3
class fibs:
    def __init__(self):
        self.a = 0
        self.b = 1
    def __iter__(self):
        return self
    def next(self):
        self.a, self.b = self.b, self.a + self.b
        return self.a
f = fibs()
fibsList = [f.next() for x in range(3)]
print(fibsList)
