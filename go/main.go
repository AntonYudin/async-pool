package main

import "fmt"
import "sync"
import "sync/atomic"
import "time"


type Value struct {
	value int
}

func newValue(value int) *Value {
	var result = new(Value)
	result.value = value
	return result
}


func main() {

	var index atomic.Int32

	var pool = newPool[Value](5, func () *Value {
		defer index.Add(1)
		return newValue(int(index.Load()))
	})

	var waitGroup sync.WaitGroup

	var channels [10]chan int

	for i := 0; i < 10; i++ {
		channels[i] = make(chan int)
		waitGroup.Add(1)
		go func(channel chan int, index int) {
			defer waitGroup.Done()
			defer close(channel)

			fmt.Printf("thread [%v] started\n", index)
			var resource, successful = pool.get()
			if successful {
				defer resource.release()
				fmt.Printf("got resource: %v\n", resource.get().value)
				time.Sleep(1 * time.Second)
				channel <- (resource.get().value * 10)
			} else {
				println("Could not obtain resource")
			}
			fmt.Printf("thread [%v] ended\n", index)
		} (channels[i], i)
	}


	for i := 0; i < 10; i++ {
		for result := range channels[i] {
			fmt.Printf("result for thread [%v]: [%v]\n", i, result)
		}
	}

	waitGroup.Wait()

	fmt.Printf("done\n")
}
