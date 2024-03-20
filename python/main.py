
import threading
import time


class Resource:
    def __init__(self, value, pool):
        self.value = value
        self.pool = pool

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, exc_tb):
        self.pool.release(self.value)

class ResourceNotAvailable(Exception):
    pass

class Pool:
    def __init__(self, capacity, creator):
        self.capacity = capacity
        self.available = []
        self.creator = creator
        self.size = 0
        self.lock = threading.Lock()

    def get(self):
        with self.lock:
            if self.available:
                return Resource(self.available.pop(), self)
            if self.size < self.capacity:
                self.size = self.size + 1
                return Resource(self.creator(), self)
            raise ResourceNotAvailable(self.capacity)

    def release(self, value):
        print(f"release {value}")
        with self.lock:
            self.available.append(value)


class Worker(threading.Thread):
    def __init__(self, pool):
        threading.Thread.__init__(self)
        self.pool = pool
        self.result = None
    def run(self):
        try:
            with self.pool.get() as resource:
                print(f"got resource {resource}")
                time.sleep(1)
                self.result = resource.value.value * 10
        except ResourceNotAvailable:
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

    pool = Pool(5, createValue)
    threads = []

    for i in range(0, 10):
        threads.append(Worker(pool))
        threads[i].start()

    for thread in threads:
        thread.join()
        print(f"got result {thread.result}")

    print("done.")

main()

