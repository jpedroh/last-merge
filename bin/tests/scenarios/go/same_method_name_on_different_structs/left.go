package example

type User struct{}

type Product struct{}

func (u *User) Validate() bool {
	return true
}
