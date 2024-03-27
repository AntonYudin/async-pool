# async-pool
An asynchronous pool of resources implemented in Rust, Java, C++, Go, and Python.

### The goals are:
- learn Rust
- compare the amount of boilerplate code
- see how ugly java is (or is it?)
- compare the level of access control that each language provides (immutable/const/private etc)
- attempt to be as idiomatic as possible
- the resource that is being pooled (the `Value` class or struct in the examples) should not be copied or cloned

### Running

Rust:
```sh
cargo run --bin main
```

Java 21:
```sh
mvn clean package exec:java
```

C++ on EvilOS
```cmd
nmake -f Makefile.nmake clean run
```

C++ on other OSs:
```sh
make clean run
```

Go:
```sh
go run main
```

Python:
```sh
python main.py
```

### Findings:
- Java is not ugly: here is the implementation of the `get()` method of the `Pool`
```java
protected Resource<T> wrap(final T value) {
	return new Resource<T>(
		supplier.get(),
		(r) -> { synchronized(available) {
			available.add(r.get());
		}}
	);
}

public Resource<T> get() throws NoSuchElementException {
	synchronized (available) {
		if (!available.isEmpty())
			return wrap(available.remove(0));

		if (size < capacity) {
			size++;
			return wrap(supplier.get());
		}
	}
	throw new NoSuchElementException();
}
```
The problem is that the `synchronized` keyword does not work with virtual threads.

The solution is to use `java.util.concurrent.locks.ReentrantLock`.

The lock can be wrapped into `java.lang.AutoCloseable` to be compatible with the try-with-resources block.

```java
protected AutoCloseableNoExceptions lock() {
	lock.lock();
	return lock::unlock;
}

protected Resource<T> wrap(final T value) {
	return new Resource<T>(
		supplier.get(),
		(r) -> { try (var l = lock()) {
			available.add(r.get());
		}}
	);
}

public Resource<T> get() throws NoSuchElementException {
	try (var l = lock()) {
		if (!available.isEmpty())
			return wrap(available.remove(0));

		if (size < capacity) {
			size++;
			return wrap(supplier.get());
		}
	}
	throw new NoSuchElementException();
}
```

- Go's approach to Mutexes does not look cute 
```go
lock.Lock()
defer lock.Unlock()
```
- It looks like it is impossible to return a resource back into a pool using a Drop trait in Rust without using std::mem:{replace,swap,take}.
A closure can be used to demarcate the resource access code though.
Here is a Rust implementation of the pool's get/process method:
```rust
pub fn process_as_mut<R, FF: FnMut(&mut Resource<T>) -> R>(&mut self, mut callback: FF) -> Result<R, PoolError> {
	let mut resource: Resource<T>;

	{
	    let mut resources = self.resources.lock().unwrap();
	    if resources.len() > 0 {
		    resource = resources.pop().unwrap();
	    } else if self.capacity > self.size.load(Ordering::Relaxed) {
		    resource = Resource {
			value: (self.creator)()?,
			on_drop: Arc::new(|| {})
		    };
		    self.size.fetch_add(1, Ordering::Relaxed);
	    } else {
		return Err(PoolError::AtCapacity(AtCapacityError {capacity: self.capacity}));
	    }
	}

	let result = callback(&mut resource);
	{
	    let mut resources = self.resources.lock().unwrap();
	    resources.push(resource);
	}

	Ok(result)
}
```
- The Python's approach to returning data from a thread seems clumsy: you are expected to inherit a custom Thread class and store the results as members of the object. Would using queue.Queue be considered more idiomatic in Python?
Go and Rust use channels to return data from threads.
Java and C++ use futures.
- C++, Go, Java and Rust have standard atomic "int" implementations. Python requires using Mutexes for a thread-safe counter.
- [RAII](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization)  is supported by C++ and Rust.
Java and Python have a different approach to automatically release resources: `try-with-resources` in Java and `with` statement in Python.
Go uses ``defer`` to accomplish this. It is less flexible because it defers the call to the end of the function. It is not possible to have a defer call that will work with a scope inside of a function instead of the function's scope.
