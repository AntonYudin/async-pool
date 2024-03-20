# async-pool
An asynchronous pool of resources implemented in Rust, Java, C++, Go, and Python.

The goals are:
- learn Rust
- compare the amount of boilerplate code
- see how ugly java is (or is it?)
- compare the level of access control that each language provides (immutable/const/private etc)
- attempt to be as idiomatic as possible

Findings:
- Java is not ugly: here is the implementation of the `get()` method of the `Pool`
```
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

