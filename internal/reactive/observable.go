package reactive

import (
	"sync"
)

// Observable represents a reactive data source
type Observable struct {
	value     interface{}
	observers []func(interface{})
	mu        sync.RWMutex
}

func NewObservable(val interface{}) *Observable {
	return &Observable{value: val}
}

func (o *Observable) Set(val interface{}) {
	o.mu.Lock()
	o.value = val
	o.mu.Unlock()
	o.notify(val)
}

func (o *Observable) Get() interface{} {
	o.mu.RLock()
	defer o.mu.RUnlock()
	return o.value
}

func (o *Observable) Subscribe(observer func(interface{})) {
	o.mu.Lock()
	o.observers = append(o.observers, observer)
	o.mu.Unlock()
}

func (o *Observable) notify(val interface{}) {
	o.mu.RLock()
	defer o.mu.RUnlock()
	for _, obs := range o.observers {
		go obs(val)
	}
}

// Computed creates a derived observable
func Computed(source *Observable, transform func(interface{}) interface{}) *Observable {
	derived := NewObservable(transform(source.Get()))
	source.Subscribe(func(val interface{}) {
		derived.Set(transform(val))
	})
	return derived
}
