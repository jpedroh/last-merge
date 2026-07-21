package main

type Writer interface {
}

type Document interface {
	GetId() (string, error)
	Writer
}
