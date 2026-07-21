package example

type User struct{}

type Product struct{}

func (p *Product) Validate() bool {
	return true
}
