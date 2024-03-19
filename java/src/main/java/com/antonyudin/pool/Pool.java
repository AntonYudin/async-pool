
package com.antonyudin.pool;

import java.util.List;
import java.util.ArrayList;
import java.util.NoSuchElementException;

import java.util.function.Supplier;
import java.util.function.Consumer;



public class Pool<T> {

	public static class Resource<T> implements AutoCloseable {

		private final T value;
		private final Consumer<Resource<T>> onClose;

		public Resource(
			final T value,
			final Consumer<Resource<T>> onClose
		) {
			this.value = value;
			this.onClose = onClose;
		}

		public T get() {
			return value;
		}

		@Override
		public void close() {
			onClose.accept(this);
		}
	}

	private final List<T> available = new ArrayList<>();
	private final Supplier<T> supplier;
	private final int capacity;
	private int size = 0;


	public Pool(int capacity, final Supplier<T> supplier) {
		this.capacity = capacity;
		this.supplier = supplier;
	}

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

}

