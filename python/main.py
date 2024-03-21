
import threading
import time

import pool


class Error(Exception):
    pass

class Worker(threading.Thread):
    def __init__(self, pool, index):
        threading.Thread.__init__(self)
        self.pool = pool
        self.result = None
        self.index = index
    def run(self):
        try:
            with self.pool.get() as resource:
                print(f"got resource {resource}")
                time.sleep(1)
                # simulate an error by throwing an exception
                if self.index == 1:
                    raise Error("error")
                self.result = resource.value.value * 10
        except pool.ResourceNotAvailable:
            print(f"could not get resource")

class Value:
    def __init__(self, value):
        self.value = value


class Counter:
    def __init__(self):
        self.value = 0
        self.lock = threading.Lock()
    def getAndIncrement(self):
        with self.lock:
            result = self.value
            self.value = self.value + 1
            return result

counter = Counter()

def createValue():
    global counter
    return Value(counter.getAndIncrement())


def main():

    p = pool.Pool(5, createValue)
    threads = []

    for i in range(0, 10):
        threads.append(Worker(p, i))
        threads[i].start()

    for thread in threads:
        thread.join()
        print(f"got result {thread.result}")

    print("done.")

main()

