package main

type Reader interface {
}

type Document interface {
	Reader
	GetContents() (string, error)
	GetId() (string, error)
	Writer
}

type Writer interface {
}