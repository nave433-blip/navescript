package contracts

import (
	"fmt"
)

// Contract defines a validator for data
type Contract struct {
	Name      string
	Validator func(interface{}) bool
}

var registry = make(map[string]*Contract)

// Register a new contract
func Register(name string, validator func(interface{}) bool) {
	registry[name] = &Contract{Name: name, Validator: validator}
}

// Check validates data against a contract
func Check(name string, data interface{}) error {
	c, ok := registry[name]
	if !ok {
		return fmt.Errorf("contract %s not found", name)
	}
	if !c.Validator(data) {
		return fmt.Errorf("contract %s violation", name)
	}
	return nil
}

// Built-in Contracts
func init() {
	Register("non_empty_string", func(val interface{}) bool {
		s, ok := val.(string)
		return ok && len(s) > 0
	})
	
	Register("positive_number", func(val interface{}) bool {
		n, ok := val.(int)
		return ok && n > 0
	})
}
