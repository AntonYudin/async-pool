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

