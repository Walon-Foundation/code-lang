package analysis

type State struct {
	Documents map[string]*Document
}

func NewState()State{
	return State{
		make(map[string]*Document),
	}
}

func(s *State)OpenDocument(uri, text string){
	s.Documents[uri] = Analyze(uri, text)
}

func (s *State)UpdateDocument(uri, text string){
	s.Documents[uri] = Analyze(uri, text)
}

func (s *State)CloseDocument(uri string){
	delete(s.Documents, uri)
}

func (s *State)GetDocument(uri string)*Document{
	return s.Documents[uri]
}
