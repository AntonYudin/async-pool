module;

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

export module pool;

export namespace pool {

	template <class T> class Resource {

		std::unique_ptr<T> value;
		const std::function<void(std::unique_ptr<T>)> onDestroy;

		public:
			Resource(std::unique_ptr<T> v, std::function<void(std::unique_ptr<T>)> d) : value(std::move(v)), onDestroy(d) {
				std::cout << "Resource()" << std::endl;
			}

			const T* get() const {return value.get();}

			~Resource() {
				std::cout << "resource.destructor()" << std::endl;
				onDestroy(std::move(value));
			}
	};


	template <class T> class Pool {

		std::list<std::unique_ptr<T>> available;
		std::mutex synchronization;

		public:
			Pool(const int capacity, const std::function<T*()> creator) {
				std::cout << "Pool(" << capacity << ")" << std::endl;
				for (int i = 0; i < capacity; i++)
					available.push_back(std::unique_ptr<T>(creator()));
			}

			std::unique_ptr<Resource<T>> get() {
				std::lock_guard<std::mutex> lock(synchronization);

				if (!available.empty()) {
					auto reference = std::move(available.back());
					available.pop_back();
					return std::unique_ptr<Resource<T>>(
						new Resource<T>(
							std::move(reference),
							[this] (std::unique_ptr<T> resource) {
								std::cout << "resource onDestroy()" << std::endl;
								this->release(std::move(resource));
							}
						)
					);
				}

				throw std::invalid_argument("no more resources available");
			}

			void release(std::unique_ptr<T> resource) {
				std::cout << "release resource()" << std::endl;
				std::lock_guard<std::mutex> lock(synchronization);
				available.push_back(std::move(resource));
			}
	};

}
