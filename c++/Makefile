
#CC=c++
CC=clang++
CCFLAGS=-std=c++20


default: main

pool.pcm: pool.cppm
	$(CC) $(CCFLAGS) pool.cppm --precompile -o pool.pcm

pool.o: pool.cppm pool.pcm
	$(CC) $(CCFLAGS) pool.pcm -c -o pool.o

main.o: main.cpp pool.pcm
	$(CC) $(CCFLAGS) -fprebuilt-module-path=. main.cpp -c -o main.o

main: main.o pool.pcm pool.o
	$(CC) $(CCFLAGS) -fprebuilt-module-path=. main.o pool.o -o main

clean:
	rm -f pool.pcm
	rm -f pool.o
	rm -f main.o
	rm -f main

run: main
	./main

