# async-pool
An asynchronous pool of resources implemented in Rust, Java, C++, Go, and Python.

### The goals are:
- learn Rust
- compare the amount of boilerplate code
- see how ugly java is (or is it?)
- compare the level of access control that each language provides (immutable/const/private etc)
- attempt to be as idiomatic as possible
- the resource that is being pooled (the `Value` class or struct in the examples) should not be copied or cloned

### Findings:
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
- Go's approach to Mutexes does not look cute 
```
lock.Lock()
defer lock.Unlock()
```
- It looks like it is impossible to return a resource back into a pool using a Drop trait in Rust without using std::mem:{replace,swap,take}.
A closure can be used to demarcate the resource access code through.
Here is a Rust implementation of the pool's get/process method:
```
pub fn process<R, FF: FnMut(&Resource<T>) -> R>(&mut self, mut callback: FF) -> Option<R> {
   let resource: Option<Resource<T>>;
   {
       let mut resources = self.resources.lock().unwrap();
       resource = resources.pop();
       println!("popped resource");
   }

   match resource {
       Some(resource) => {
           let result = Some(callback(&resource));
           {
               let mut resources = self.resources.lock().unwrap();
               resources.push(resource);
               println!("pushed resource");
           }
           result
       },
       None => None
   }
}
```
- The Python's approach to returning data from a thread seems clumsy: you are expected to inherit a custom Thread class and store the results as members of the object. Would using queue.Queue be considered more idiomatic?
Go and Rust use channels to return data from threads.
Java and C++ use futures.
- C++, Go, Java and Rust have standard atomic "int" implementations. Python requires using Mutexes for a thread-safe counter.
- RAII is supported by C++ and Rust.
Java and Python have a different approach to automatically release resources: `try-with-resources` in Java and `with` statement in Python.
Go uses ``defer`` to accomplish this. It is less flexible because it defers the call to the end of the function. It is not possible to have a defer call that will work with a scope inside of a function instead of the function's scope.
