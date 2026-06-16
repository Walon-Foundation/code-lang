package lsp

var version = "0.0.1"

func NewInitializeResponse(id int) InitializeResponse {
	return InitializeResponse{
		Response: Response{
			RPC: "2.0",
			ID:  &id,
		},
		Result: InitializeResult{
			ServerInfo: &ServerInfo{
				Name:    "code-lang-lsp",
				Version: &version,
			},
			Capabilities: ServerCapabilities{
				TextDocumentSync: TextDocumentSyncOptions{
					Change:    1,
					OpenClose: true,
				},
				HoverProvider: true,
				CompletionProvider: &CompletionOptions{
					ResolveProvider: false,
				},
				DefinitionProvider: true,
				DeclarationProvider: true,
				ImplementationProvider: true,
				DocumentSymbolProvider: true,
				ReferencesProvider: true,
				RenameProvider: true,
				CodeActionProvider: true,
			},
		},
	}
}

func HoverResponseMessage(id int)HoverResponse{
	return HoverResponse{
		Response: Response{
			RPC: "2.0",
			ID: &id,
		},
		Result: Hover{
			Contents: "Hello world",
		},
	}
}

func CompletionResponseMessage(id int) CompletionResponse {
	return CompletionResponse{
		Response: Response{
			RPC: "2.0",
			ID:  &id,
		},
		Result: CompletionList{
			IsIncomplete: false,
			Items: []CompletionItem{
				{Label: "let", Detail: "keyword"},
				{Label: "const", Detail: "keyword"},
				{Label: "fn", Detail: "keyword"},
				{Label: "if", Detail: "keyword"},
				{Label: "else", Detail: "keyword"},
				{Label: "elseif", Detail: "keyword"},
				{Label: "while", Detail: "keyword"},
				{Label: "for", Detail: "keyword"},
				{Label: "return", Detail: "keyword"},
			},
		},
	}
}

func DefinitionResponseMessage(id int, uri string) DefinitionResponse {
	return DefinitionResponse{
		Response: Response{
			RPC: "2.0",
			ID:  &id,
		},
		Result: []Location{
			{
				URI: uri,
				Range: Range{
					Start: Position{Line: 0, Character: 0},
					End:   Position{Line: 0, Character: 1},
				},
			},
		},
	}
}

func DeclarationResponseMessage(id int, uri string) DeclarationResponse {
	return DeclarationResponse{
		Response: Response{
			RPC: "2.0",
			ID:  &id,
		},
		Result: []Location{
			{
				URI: uri,
				Range: Range{
					Start: Position{Line: 0, Character: 0},
					End:   Position{Line: 0, Character: 1},
				},
			},
		},
	}
}

func ImplementationResponseMessage(id int, uri string) ImplementationResponse {
	return ImplementationResponse{
		Response: Response{
			RPC: "2.0",
			ID:  &id,
		},
		Result: []Location{
			{
				URI: uri,
				Range: Range{
					Start: Position{Line: 0, Character: 0},
					End:   Position{Line: 0, Character: 1},
				},
			},
		},
	}
}

func DocumentSymbolResponseMessage(id int) DocumentSymbolResponse {
	return DocumentSymbolResponse{
		Response: Response{
			RPC: "2.0",
			ID:  &id,
		},
		Result: []DocumentSymbol{
			{
				Name: "main",
				Kind: 12,
				Range: Range{
					Start: Position{Line: 0, Character: 0},
					End:   Position{Line: 0, Character: 4},
				},
				SelectionRange: Range{
					Start: Position{Line: 0, Character: 0},
					End:   Position{Line: 0, Character: 4},
				},
			},
		},
	}
}

func ReferenceResponseMessage(id int, uri string) ReferenceResponse {
	return ReferenceResponse{
		Response: Response{
			RPC: "2.0",
			ID:  &id,
		},
		Result: []Location{
			{
				URI: uri,
				Range: Range{
					Start: Position{Line: 0, Character: 0},
					End:   Position{Line: 0, Character: 1},
				},
			},
		},
	}
}

func RenameResponseMessage(id int, uri string, newName string) RenameResponse {
	return RenameResponse{
		Response: Response{
			RPC: "2.0",
			ID:  &id,
		},
		Result: WorkspaceEdit{
			Changes: map[string][]TextEdit{
				uri: {
					{
						Range: Range{
							Start: Position{Line: 0, Character: 0},
							End:   Position{Line: 0, Character: 1},
						},
						NewText: newName,
					},
				},
			},
		},
	}
}

func CodeActionResponseMessage(id int) CodeActionResponse {
	return CodeActionResponse{
		Response: Response{
			RPC: "2.0",
			ID:  &id,
		},
		Result: []CodeAction{
			{
				Title: "Demo: No-op",
				Kind:  "quickfix",
			},
		},
	}
}

func PublishDiagnosticsMessage(uri string) PublishDiagnosticsNotification {
	return PublishDiagnosticsNotification{
		Notification: Notification{
			RPC:    "2.0",
			Method: "textDocument/publishDiagnostics",
		},
		Params: PublishDiagnosticsParams{
			URI:         uri,
			Diagnostics: []Diagnostic{},
		},
	}
}
