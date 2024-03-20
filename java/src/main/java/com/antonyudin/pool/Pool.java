
package com.antonyudin.pool;

import java.util.List;
import java.util.ArrayList;
import java.util.NoSuchElementException;

import java.util.function.Supplier;
import java.util.function.Consumer;

import java.util.concurrent.locks.ReentrantLock;




public class Pool<T> {

	public interface AutoCloseableNoExceptions extends AutoCloseable {
		@Override
		public void close();
	}

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
	private final ReentrantLock lock = new ReentrantLock();


	public Pool(int capacity, final Supplier<T> supplier) {
		this.capacity = capacity;
		this.supplier = supplier;
	}

	protected AutoCloseableNoExceptions lock() {
		lock.lock();
		return lock::unlock;
	}

	protected Resource<T> wrap(final T value) {
		return new Resource<T>(
			supplier.get(),
			(r) -> { try (var _ = lock()) {
				available.add(r.get());
			}}
		);
	}

	public Resource<T> get() throws NoSuchElementException {
		try (var _ = lock()) {
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

