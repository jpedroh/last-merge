package main

type Calculator interface {
	Sum(x int, y int) int
	Multiply(x int, y int) int
}
