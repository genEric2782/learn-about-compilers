package main

import "sync"

type Stack struct {
	items []interface{} //interface{} empty interface (can hold generic type)
	mu    sync.Mutex
}

func (s *Stack) Push(T interface{}) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.items = append(s.items, T)
}

func (s *Stack) Pop() (interface{}, bool) {
	s.mu.Lock()
	defer s.mu.Unlock()
	if len(s.items) == 0 {
		return nil, false
	}
	val := s.items[len(s.items)-1]
	s.items = s.items[:len(s.items)-1]
	return val, true
}

var GlobalStack = &Stack{}
