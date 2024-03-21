
#ifndef _MSC_VER
	module;
#endif

#include <iostream>
#include <list>
#include <functional>
#include <mutex>
#include <stdexcept>
#include <memory>
#include <thread>
#include <chrono>
#include <future>
#include <atomic>

#ifndef _MSC_VER
	export module main;
#endif

import pool;

class Value {
	public:
		Value(const int v): value(v) {
			std::cout << "Value(" << v << ")" << std::endl;
		}
		int value;
		~Value() {
			std::cout << "Value.destructor(" << value << ")" << std::endl;
		}
	private:
		Value(const Value&);
		Value& operator = (const Value&);
};

int main() {

	std::atomic<int> index {0};

	pool::Pool<Value> pool(5, [&index]() { return new Value(index++); });

	std::list<std::future<int>> futures;

	for (int i = 0; i < 10; i++) {
		auto f = [&pool] (int index) {
			std::cerr << "thread started\n";

			auto resource = pool.get();
			std::cout << "got resource: " << resource->get()->value << std::endl;

			// simulating an error by throwing an exception
			//if (index == 1)
			//	throw std::invalid_argument("error");

			std::this_thread::sleep_for(std::chrono::seconds(1));
			std::cerr << "thread ended\n";

			return (resource->get()->value * 10);
		};

		futures.push_back(
			std::async(
				std::launch::async, f, i
			)
		);
	}

	std::cout << "started all threads\n";

	for (auto & future: futures) {
		std::cout << "checking future\n";
		try {
			std::cout << "result " << future.get() << std::endl;
		} catch (...) {
			std::cout << "exception\n";
		}
	}

	std::cout << "end." << std::endl;
}

