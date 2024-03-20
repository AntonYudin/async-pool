package pool

import "fmt"
import "sync"


type Resource[T any] struct {
	value *T
	pool *Pool[T]
}

func (self Resource[T]) Release() {
	fmt.Printf("Resource.release()\n")
	self.pool.release(self.value)
}

func (self Resource[T]) Get() T {
	return *self.value
}

type Pool[T any] struct {
	resources []*T
	available []bool
	lock sync.Mutex
}

type Creator[T any] func () *T

func NewPool[T any](capacity int, creator Creator[T]) *Pool[T] {

	var result = new(Pool[T])

	result.resources = make([]*T, capacity)
	result.available = make([]bool, capacity)

	for i := 0; i < capacity; i++ {
		result.resources[i] = creator()
		result.available[i] = true
	}

	return result
}

func (self Pool[T]) getCapacity() int {
	return len(self.resources)
}


func (self Pool[T]) Get() (*Resource[T], bool) {
	self.lock.Lock()
	defer self.lock.Unlock()
	for i := 0; i < len(self.available); i++ {
		if self.available[i] {
			self.available[i] = false
			return &Resource[T] { self.resources[i], &self }, true
		}
	}
	return nil, false
}

func (self Pool[T]) release(value *T) {
	self.lock.Lock()
	defer self.lock.Unlock()
	for i := 0; i < len(self.resources); i++ {
		if self.resources[i] == value {
			self.available[i] = true
		}
	}
}



