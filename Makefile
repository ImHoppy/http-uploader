all:
	clang++ -march=native -o fb_cdn main.cpp -std=c++11 -lpthread
