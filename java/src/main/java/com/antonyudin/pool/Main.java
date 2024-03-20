
package com.antonyudin.pool;

import java.util.List;
import java.util.ArrayList;
import java.util.NoSuchElementException;

import java.util.concurrent.Executors;
import java.util.concurrent.FutureTask;
import java.util.concurrent.atomic.AtomicInteger;



public class Main {

	public record Value (int value) {}

	public static void main(final String[] argv) {

		final var index = new AtomicInteger(0);

		final var pool = new Pool<Value>(5, () -> new Value(index.getAndIncrement()));

		final var tasks = new ArrayList<FutureTask<Integer>>();
		final var executor = Executors.newVirtualThreadPerTaskExecutor();

		for (var i = 0; i < 10; i++) {
			final var task = new FutureTask<>(() -> {
				try (final var resource = pool.get()) {
					System.out.format("got resource: [%d]\n", resource.get().value);
					Thread.sleep(1000);
					// simulating an error by throwing an exception
					if (resource.get().value == 1)
						throw new IllegalArgumentException();
					return resource.get().value * 10;
				}
			});
			tasks.add(task);
			executor.execute(task);
		}

		for (final var task: tasks) {
			try {
				System.out.format("result: %d\n", task.get());
			} catch (final Exception exception) {
				switch (exception.getCause()) {
					case NoSuchElementException e -> System.out.println("resource not available");
					default -> System.out.format("Execution failed: %s\n", exception);
				}
			}
		}
	}

}

