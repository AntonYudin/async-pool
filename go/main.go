package main

import "fmt"
import "sync"
import "sync/atomic"
import "time"

import "main/pool"


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

	var pool = pool.NewPool[Value](5, func () *Value {
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
			defer fmt.Printf("thread [%v] ended\n", index)

			fmt.Printf("thread [%v] started\n", index)
			var resource, successful = pool.Get()
			if successful {
				defer resource.Release()
				fmt.Printf("got resource: %v\n", resource.Get().value)
				time.Sleep(1 * time.Second)
				// simulating an error by not writing to the channel
				if index == 1 {
					return
				}
				channel <- (resource.Get().value * 10)
			} else {
				println("Could not obtain resource")
			}
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

