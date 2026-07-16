package main

type Calculator interface {
	Sum(x int, y int) int
	Subtract(x int, y int) int
}
