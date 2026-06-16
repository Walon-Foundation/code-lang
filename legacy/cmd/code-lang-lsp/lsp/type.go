package lsp

type Request struct {
	RPC    string `json:"jsonrpc"`
	ID     int    `json:"id"`
	Method string `json:"method"`

	//Params .....
}

type Response struct {
	RPC string `json:"jsonrpc"`
	ID  *int   `json:"id,omitempty"`

	//Result .....
	// Error ....
}

type Notification struct {
	RPC    string `json:"jsonrpc"`
	Method string `json:"method"`

	//Params .............
}

type ClientInfo struct {
	Name    string `json:"name"`
	Version string `json:"version"`
}

type InitializeParams struct {
	ClientInfo *ClientInfo `json:"clientInfo"`
	RootUri    string      `json:"rootUri"`

	//.... more to come
}

type TextDocumentSyncOptions struct {
	OpenClose bool `json:"openClose"`
	Change    int  `json:"change"`
}

type ServerCapabilities struct {
	TextDocumentSync TextDocumentSyncOptions `json:"textDocumentSync"`
	HoverProvider bool `json:"hoverProvider"`
	CompletionProvider *CompletionOptions `json:"completionProvider,omitempty"`
	DefinitionProvider bool `json:"definitionProvider,omitempty"`
	DeclarationProvider bool `json:"declarationProvider,omitempty"`
	ImplementationProvider bool `json:"implementationProvider,omitempty"`
	DocumentSymbolProvider bool `json:"documentSymbolProvider,omitempty"`
	ReferencesProvider bool `json:"referencesProvider,omitempty"`
	RenameProvider bool `json:"renameProvider,omitempty"`
	CodeActionProvider bool `json:"codeActionProvider,omitempty"`
}

type CompletionOptions struct {
	ResolveProvider bool `json:"resolveProvider,omitempty"`
}

type ServerInfo struct {
	Name    string  `json:"name"`
	Version *string `json:"version"`
}

type InitializeResult struct {
	Capabilities ServerCapabilities `json:"capabilities"`
	ServerInfo   *ServerInfo        `json:"serverInfo"`
}

// type for the initialize request
type InitializeRequest struct {
	Request
	Params InitializeParams `json:"params"`
}

// type for initialize response
type InitializeResponse struct {
	Response
	Result InitializeResult `json:"result"`
}

// type TextDocumentItem
type TextDocumentItem struct {
	URI        string `json:"uri"`
	LanguageID string `json:"languageId"`
	Version    int    `json:"version"`
	Text       string `json:"text"`
}

type DidOpenTextDocumentParams struct {
	TextDocument TextDocumentItem `json:"textDocument"`
}

type TextDocumentIdentifier struct {
	URI string `json:"uri"`
}

type VersionTextDocumentIdentifier struct {
	TextDocumentIdentifier
	Version int `json:"version"`
}

type TextDocumentContentChange struct {
	Text string `json:"text"`
}

type DidChangeTextDocumentParams struct {
	TextDocument   VersionTextDocumentIdentifier `json:"textDocument"`
	ContentChanges []TextDocumentContentChange   `json:"contentChanges"`
}

// type for DidopenTextDocumentNotification
type DidOpenTextDocumentNotification struct {
	Notification
	Params DidOpenTextDocumentParams `json:"params"`
}

// type for didchangetextdocument notification
type DidChangeTextDocumentNotification struct {
	Notification
	Params DidChangeTextDocumentParams `json:"params"`
}

type Position struct {
	Line int `json:"line"`
	Character int `json:"character"`
}

type TextDocumentPositionParams struct {
	TextDocument TextDocumentIdentifier `json:"textDocument"`
	Position Position `json:"position"`
}

// type WorkDoneProgressParams struct {
// 	WorkDoneToken int `json:"workDoneToken"`
// }

type HoverParams struct {
	TextDocumentPositionParams
	// WorkDoneProgressParams
}

// type Range struct {
// 	Start Position `json:"start"`
// 	End Position `json:"end"`
// }

//type for the hover request
type HoverRequest struct {
	Request
	Params HoverParams `json:"params"`
}

// type for hover response
type HoverResponse struct {
	Response
	Result Hover `json:"result"`
}

type Hover struct {
	Contents string `json:"contents"`
	// Range Range `json:"range"`
}

type Range struct {
	Start Position `json:"start"`
	End Position `json:"end"`
}

type Location struct {
	URI string `json:"uri"`
	Range Range `json:"range"`
}

type Diagnostic struct {
	Range    Range  `json:"range"`
	Severity int    `json:"severity,omitempty"`
	Source   string `json:"source,omitempty"`
	Message  string `json:"message"`
}

type PublishDiagnosticsParams struct {
	URI         string       `json:"uri"`
	Diagnostics []Diagnostic `json:"diagnostics"`
}

type PublishDiagnosticsNotification struct {
	Notification
	Params PublishDiagnosticsParams `json:"params"`
}

type CompletionItem struct {
	Label string `json:"label"`
	Kind  int    `json:"kind,omitempty"`
	Detail string `json:"detail,omitempty"`
	InsertText string `json:"insertText,omitempty"`
}

type CompletionList struct {
	IsIncomplete bool             `json:"isIncomplete"`
	Items        []CompletionItem `json:"items"`
}

type CompletionParams struct {
	TextDocumentPositionParams
}

type CompletionRequest struct {
	Request
	Params CompletionParams `json:"params"`
}

type CompletionResponse struct {
	Response
	Result CompletionList `json:"result"`
}

type DefinitionParams struct {
	TextDocumentPositionParams
}

type DefinitionRequest struct {
	Request
	Params DefinitionParams `json:"params"`
}

type DefinitionResponse struct {
	Response
	Result []Location `json:"result"`
}

type DeclarationParams struct {
	TextDocumentPositionParams
}

type DeclarationRequest struct {
	Request
	Params DeclarationParams `json:"params"`
}

type DeclarationResponse struct {
	Response
	Result []Location `json:"result"`
}

type ImplementationParams struct {
	TextDocumentPositionParams
}

type ImplementationRequest struct {
	Request
	Params ImplementationParams `json:"params"`
}

type ImplementationResponse struct {
	Response
	Result []Location `json:"result"`
}

type DocumentSymbolParams struct {
	TextDocument TextDocumentIdentifier `json:"textDocument"`
}

type DocumentSymbolRequest struct {
	Request
	Params DocumentSymbolParams `json:"params"`
}

type DocumentSymbol struct {
	Name           string   `json:"name"`
	Kind           int      `json:"kind"`
	Range          Range    `json:"range"`
	SelectionRange Range    `json:"selectionRange"`
}

type DocumentSymbolResponse struct {
	Response
	Result []DocumentSymbol `json:"result"`
}

type ReferenceParams struct {
	TextDocumentPositionParams
}

type ReferenceRequest struct {
	Request
	Params ReferenceParams `json:"params"`
}

type ReferenceResponse struct {
	Response
	Result []Location `json:"result"`
}

type TextEdit struct {
	Range   Range  `json:"range"`
	NewText string `json:"newText"`
}

type WorkspaceEdit struct {
	Changes map[string][]TextEdit `json:"changes"`
}

type RenameParams struct {
	TextDocument TextDocumentIdentifier `json:"textDocument"`
	Position     Position              `json:"position"`
	NewName      string                `json:"newName"`
}

type RenameRequest struct {
	Request
	Params RenameParams `json:"params"`
}

type RenameResponse struct {
	Response
	Result WorkspaceEdit `json:"result"`
}

type CodeActionContext struct {
	Diagnostics []Diagnostic `json:"diagnostics"`
}

type CodeActionParams struct {
	TextDocument TextDocumentIdentifier `json:"textDocument"`
	Range        Range                  `json:"range"`
	Context      CodeActionContext      `json:"context"`
}

type CodeActionRequest struct {
	Request
	Params CodeActionParams `json:"params"`
}

type CodeAction struct {
	Title string        `json:"title"`
	Kind  string        `json:"kind,omitempty"`
	Edit  *WorkspaceEdit `json:"edit,omitempty"`
}

type CodeActionResponse struct {
	Response
	Result []CodeAction `json:"result"`
}

type DidCloseTextDocumentParams struct {
	TextDocument TextDocumentIdentifier `json:"textDocument"`
}

type DidCloseTextDocumentNotification struct {
	Notification
	Params DidCloseTextDocumentParams `json:"params"`
}
