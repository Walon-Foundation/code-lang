package main

import (
	"bufio"
	"encoding/json"
	"io"
	"log"
	"os"
	"regexp"
	"strings"

	"github.com/walonCode/code-lang/cmd/code-lang-lsp/analysis"
	"github.com/walonCode/code-lang/cmd/code-lang-lsp/lsp"
	"github.com/walonCode/code-lang/cmd/code-lang-lsp/rpc"
	"github.com/walonCode/code-lang/internal/symbol"
	"github.com/walonCode/code-lang/internal/std/arrays"
	"github.com/walonCode/code-lang/internal/std/fs"
	"github.com/walonCode/code-lang/internal/std/general"
	"github.com/walonCode/code-lang/internal/std/hash"
	"github.com/walonCode/code-lang/internal/std/math"
	"github.com/walonCode/code-lang/internal/std/net"
	stdstrings "github.com/walonCode/code-lang/internal/std/strings"
	"github.com/walonCode/code-lang/internal/std/time"
	osModule"github.com/walonCode/code-lang/internal/std/os"	
	JsonModule"github.com/walonCode/code-lang/internal/std/json"
	"github.com/walonCode/code-lang/internal/object"
)

func logger(filename string) *log.Logger {
	logfile, err := os.OpenFile(filename, os.O_CREATE|os.O_TRUNC|os.O_WRONLY, 0664)
	if err != nil {
		// Fall back to stderr so the server doesn't crash if the log file
		// can't be created in the current working directory.
		return log.New(os.Stderr, "[code-lang-ls]", log.Ldate|log.Ltime|log.Lshortfile)
	}

	return log.New(logfile, "[code-lang-ls]", log.Ldate|log.Ltime|log.Lshortfile)
}

func main() {
	logger := logger("log.txt")
	logger.Println("started lsp")
	defer func() {
		if r := recover(); r != nil {
			logger.Printf("panic: %v", r)
		}
	}()
	scanner := bufio.NewScanner(os.Stdin)
	// Allow larger LSP messages than the default 64K token limit.
	scanner.Buffer(make([]byte, 0, 64*1024), 10*1024*1024)
	scanner.Split(rpc.Spilt)
	
	state := analysis.NewState()
	writer := os.Stdout
	
	for scanner.Scan(){
		msg := scanner.Bytes()
		method, content, err := rpc.DecodeMessage(msg)
		if err != nil {
			logger.Printf("Got an error: %s ",err)
		}
		handleMessage(logger, writer, state, method, content)
	}

	if err := scanner.Err(); err != nil {
		logger.Printf("scanner error: %v", err)
	}
}

func writeResponse(writer io.Writer, msg any){
	reply,_ := rpc.EncodeMessage(msg)
	writer.Write([]byte(reply))
}

func handleMessage(logger *log.Logger,writer io.Writer, state analysis.State, method string, content []byte){
	logger.Printf("Recieved msg with method: %s", method)
	
	switch method{
		case "initialize":
			var request lsp.InitializeRequest
			if err := json.Unmarshal(content, &request);err != nil {
				logger.Printf("Unable to parse the initialize request with err: %s", err)
			}

			clientName := "unknown"
			if request.Params.ClientInfo != nil && request.Params.ClientInfo.Name != "" {
				clientName = request.Params.ClientInfo.Name
			}
			logger.Printf("The client name is: %s", clientName)
			
			//reply
			msg := lsp.NewInitializeResponse(request.ID)
			writeResponse(writer,msg)
			
		case "textDocument/didOpen":
			var request lsp.DidOpenTextDocumentNotification
			if err := json.Unmarshal(content, &request);err != nil {
				logger.Printf("unable to parse the text document did open notificaton: %s",err)
			}
			
			logger.Printf("page uri: %s", request.Params.TextDocument.URI)
			state.OpenDocument(request.Params.TextDocument.URI, request.Params.TextDocument.Text)
			if doc := state.GetDocument(request.Params.TextDocument.URI); doc != nil {
				writeResponse(writer, lsp.PublishDiagnosticsNotification{
					Notification: lsp.Notification{
						RPC:    "2.0",
						Method: "textDocument/publishDiagnostics",
					},
					Params: lsp.PublishDiagnosticsParams{
						URI:         request.Params.TextDocument.URI,
						Diagnostics: doc.Diagnostics(),
					},
				})
			}
		case "textDocument/didChange":
			var request lsp.DidChangeTextDocumentNotification
			if err := json.Unmarshal(content, &request); err != nil {
				logger.Printf("unable to parse the text document did change notification: %s", err)
			}
			
			logger.Printf("changed uri: %s", request.Params.TextDocument.URI,)
			for _, change := range request.Params.ContentChanges {
				state.UpdateDocument(request.Params.TextDocument.URI, change.Text)
			}
			if doc := state.GetDocument(request.Params.TextDocument.URI); doc != nil {
				writeResponse(writer, lsp.PublishDiagnosticsNotification{
					Notification: lsp.Notification{
						RPC:    "2.0",
						Method: "textDocument/publishDiagnostics",
					},
					Params: lsp.PublishDiagnosticsParams{
						URI:         request.Params.TextDocument.URI,
						Diagnostics: doc.Diagnostics(),
					},
				})
			}
		case "textDocument/didClose":
			var request lsp.DidCloseTextDocumentNotification
			if err := json.Unmarshal(content, &request); err != nil {
				logger.Printf("unable to parse the text document did close notification: %s", err)
			}
			
			logger.Printf("closed uri: %s", request.Params.TextDocument.URI)
			state.CloseDocument(request.Params.TextDocument.URI)
		case "textDocument/hover":
			var request lsp.HoverRequest
			if err := json.Unmarshal(content, &request);err != nil {
				logger.Printf("Unable to parse the hover request with err: %s", err)
			}
			
			logger.Printf("The client name is: %s",request.Params.TextDocument.URI)
			
			contents := ""
			if doc := state.GetDocument(request.Params.TextDocument.URI); doc != nil {
				occ := doc.FindOccurrenceAt(request.Params.Position)
				if occ != nil {
					kind := occ.Kind.String()
					if occ.IsDefinition {
						contents = occ.Name + " (" + kind + ")"
					} else if occ.Def != nil {
						contents = occ.Name + " (" + kind + ")"
					} else {
						contents = occ.Name
					}
				} else if doc.Index != nil {
					if modName, member, ok := memberCompletionContext(doc.Text, request.Params.Position); ok {
						if mems, ok := moduleMembersFor(doc, modName); ok {
							for _, m := range mems {
								if m == member {
									contents = modName + "." + member
									break
								}
							}
						}
					}
					if contents == "" {
						for _, m := range doc.Index.MemberProps {
							if m != nil && contains(m.Range, request.Params.Position) {
								contents = "member " + m.Name
								break
							}
						}
					}
				}
			}
			if contents == "" {
				contents = "Code-Lang"
			}
			msg := lsp.HoverResponse{
				Response: lsp.Response{
					RPC: "2.0",
					ID:  &request.ID,
				},
				Result: lsp.Hover{Contents: contents},
			}
			writeResponse(writer, msg)
		case "textDocument/completion":
			var request lsp.CompletionRequest
			if err := json.Unmarshal(content, &request); err != nil {
				logger.Printf("Unable to parse the completion request with err: %s", err)
			}
			
			items := []lsp.CompletionItem{
				{Label: "let", Detail: "keyword"},
				{Label: "const", Detail: "keyword"},
				{Label: "fn", Detail: "keyword"},
				{Label: "if", Detail: "keyword"},
				{Label: "else", Detail: "keyword"},
				{Label: "elseif", Detail: "keyword"},
				{Label: "while", Detail: "keyword"},
				{Label: "for", Detail: "keyword"},
				{Label: "return", Detail: "keyword"},
				{Label: "break", Detail: "keyword"},
				{Label: "continue", Detail: "keyword"},
				{Label: "struct", Detail: "keyword"},
				{Label: "import", Detail: "keyword"},
				{Label: "true", Detail: "keyword"},
				{Label: "false", Detail: "keyword"},
			}
			if doc := state.GetDocument(request.Params.TextDocument.URI); doc != nil && doc.Index != nil {
				if modName, prefix, ok := memberCompletionContext(doc.Text, request.Params.Position); ok {
					if mems, ok := moduleMembersFor(doc, modName); ok {
						items = []lsp.CompletionItem{}
						for _, m := range mems {
							if prefix == "" || strings.HasPrefix(m, prefix) {
								items = append(items, lsp.CompletionItem{
									Label:  m,
									Detail: "module member",
								})
							}
						}
					}
				} else {
					seen := map[string]bool{}
					for _, def := range doc.CompletionAt(request.Params.Position) {
						if def == nil {
							continue
						}
						if seen[def.Name] {
							continue
						}
						seen[def.Name] = true
						items = append(items, lsp.CompletionItem{
							Label:  def.Name,
							Detail: def.Kind.String(),
						})
					}
				}
			}
			msg := lsp.CompletionResponse{
				Response: lsp.Response{
					RPC: "2.0",
					ID:  &request.ID,
				},
				Result: lsp.CompletionList{
					IsIncomplete: false,
					Items:        items,
				},
			}
			writeResponse(writer, msg)
		case "textDocument/definition":
			var request lsp.DefinitionRequest
			if err := json.Unmarshal(content, &request); err != nil {
				logger.Printf("Unable to parse the definition request with err: %s", err)
			}
			
			var locs []lsp.Location
			if doc := state.GetDocument(request.Params.TextDocument.URI); doc != nil {
				for _, def := range doc.DefinitionsFor(request.Params.Position) {
					locs = append(locs, lsp.Location{URI: def.URI, Range: def.Range})
				}
			}
			msg := lsp.DefinitionResponse{
				Response: lsp.Response{
					RPC: "2.0",
					ID:  &request.ID,
				},
				Result: locs,
			}
			writeResponse(writer, msg)
		case "textDocument/declaration":
			var request lsp.DeclarationRequest
			if err := json.Unmarshal(content, &request); err != nil {
				logger.Printf("Unable to parse the declaration request with err: %s", err)
			}
			
			var locs []lsp.Location
			if doc := state.GetDocument(request.Params.TextDocument.URI); doc != nil {
				for _, def := range doc.DefinitionsFor(request.Params.Position) {
					locs = append(locs, lsp.Location{URI: def.URI, Range: def.Range})
				}
			}
			msg := lsp.DeclarationResponse{
				Response: lsp.Response{
					RPC: "2.0",
					ID:  &request.ID,
				},
				Result: locs,
			}
			writeResponse(writer, msg)
		case "textDocument/implementation":
			var request lsp.ImplementationRequest
			if err := json.Unmarshal(content, &request); err != nil {
				logger.Printf("Unable to parse the implementation request with err: %s", err)
			}
			
			var locs []lsp.Location
			if doc := state.GetDocument(request.Params.TextDocument.URI); doc != nil {
				for _, def := range doc.DefinitionsFor(request.Params.Position) {
					locs = append(locs, lsp.Location{URI: def.URI, Range: def.Range})
				}
			}
			msg := lsp.ImplementationResponse{
				Response: lsp.Response{
					RPC: "2.0",
					ID:  &request.ID,
				},
				Result: locs,
			}
			writeResponse(writer, msg)
		case "textDocument/documentSymbol":
			var request lsp.DocumentSymbolRequest
			if err := json.Unmarshal(content, &request); err != nil {
				logger.Printf("Unable to parse the documentSymbol request with err: %s", err)
			}
			
			var symbols []lsp.DocumentSymbol
			if doc := state.GetDocument(request.Params.TextDocument.URI); doc != nil && doc.Index != nil {
				for _, def := range doc.Index.Definitions {
					if def == nil {
						continue
					}
					symbols = append(symbols, lsp.DocumentSymbol{
						Name:           def.Name,
						Kind:           lspSymbolKind(def.Kind),
						Range:          def.Range,
						SelectionRange: def.Range,
					})
				}
			}
			msg := lsp.DocumentSymbolResponse{
				Response: lsp.Response{
					RPC: "2.0",
					ID:  &request.ID,
				},
				Result: symbols,
			}
			writeResponse(writer, msg)
		case "textDocument/references":
			var request lsp.ReferenceRequest
			if err := json.Unmarshal(content, &request); err != nil {
				logger.Printf("Unable to parse the references request with err: %s", err)
			}
			
			var locs []lsp.Location
			if doc := state.GetDocument(request.Params.TextDocument.URI); doc != nil {
				refs := doc.ReferencesFor(request.Params.Position)
				for _, ref := range refs {
					locs = append(locs, lsp.Location{URI: ref.URI, Range: ref.Range})
				}
				if defs := doc.DefinitionsFor(request.Params.Position); len(defs) > 0 {
					for _, def := range defs {
						locs = append(locs, lsp.Location{URI: def.URI, Range: def.Range})
					}
				}
			}
			msg := lsp.ReferenceResponse{
				Response: lsp.Response{
					RPC: "2.0",
					ID:  &request.ID,
				},
				Result: locs,
			}
			writeResponse(writer, msg)
		case "textDocument/rename":
			var request lsp.RenameRequest
			if err := json.Unmarshal(content, &request); err != nil {
				logger.Printf("Unable to parse the rename request with err: %s", err)
			}
			
			changes := map[string][]lsp.TextEdit{}
			if doc := state.GetDocument(request.Params.TextDocument.URI); doc != nil {
				refs := doc.ReferencesFor(request.Params.Position)
				defs := doc.DefinitionsFor(request.Params.Position)
				for _, ref := range refs {
					changes[ref.URI] = append(changes[ref.URI], lsp.TextEdit{
						Range:   ref.Range,
						NewText: request.Params.NewName,
					})
				}
				for _, def := range defs {
					changes[def.URI] = append(changes[def.URI], lsp.TextEdit{
						Range:   def.Range,
						NewText: request.Params.NewName,
					})
				}
			}
			msg := lsp.RenameResponse{
				Response: lsp.Response{
					RPC: "2.0",
					ID:  &request.ID,
				},
				Result: lsp.WorkspaceEdit{Changes: changes},
			}
			writeResponse(writer, msg)
		case "textDocument/codeAction":
			var request lsp.CodeActionRequest
			if err := json.Unmarshal(content, &request); err != nil {
				logger.Printf("Unable to parse the codeAction request with err: %s", err)
			}
			
			msg := lsp.CodeActionResponse{
				Response: lsp.Response{
					RPC: "2.0",
					ID:  &request.ID,
				},
				Result: codeActionsFromDiagnostics(request.Params.TextDocument.URI, request.Params.Context.Diagnostics),
			}
			writeResponse(writer, msg)
		default:
			logger.Printf("new unknown method: %s", method)
	}
}

func lspSymbolKind(kind symbol.SymbolKind) int {
	switch kind {
	case symbol.FUNCTION:
		return 12
	case symbol.VARIABLE:
		return 13
	case symbol.CONSTANT:
		return 14
	case symbol.STRUCT:
		return 23
	case symbol.PARAMETER:
		return 26
	default:
		return 13
	}
}

func codeActionsFromDiagnostics(uri string, diags []lsp.Diagnostic) []lsp.CodeAction {
	var actions []lsp.CodeAction
	for _, d := range diags {
		name, ok := extractUndefinedName(d.Message)
		if ok && name != "" {
			edit := lsp.WorkspaceEdit{
				Changes: map[string][]lsp.TextEdit{
					uri: {
						{
							Range: lsp.Range{
								Start: lsp.Position{Line: 0, Character: 0},
								End:   lsp.Position{Line: 0, Character: 0},
							},
							NewText: "let " + name + " = null;\n",
						},
					},
				},
			}
			actions = append(actions, lsp.CodeAction{
				Title: "Create variable '" + name + "'",
				Kind:  "quickfix",
				Edit:  &edit,
			})
		}
	}
	if len(actions) == 0 {
		actions = append(actions, lsp.CodeAction{
			Title: "No fixes available",
			Kind:  "quickfix",
		})
	}
	return actions
}

func extractUndefinedName(msg string) (string, bool) {
	const needle = "undefined identifier: "
	idx := strings.Index(msg, needle)
	if idx == -1 {
		return "", false
	}
	name := strings.TrimSpace(msg[idx+len(needle):])
	if name == "" {
		return "", false
	}
	return name, true
}

func contains(r lsp.Range, pos lsp.Position) bool {
	if pos.Line < r.Start.Line || pos.Line > r.End.Line {
		return false
	}
	if pos.Line == r.Start.Line && pos.Character < r.Start.Character {
		return false
	}
	if pos.Line == r.End.Line && pos.Character >= r.End.Character {
		return false
	}
	return true
}

func memberCompletionContext(text string, pos lsp.Position) (string, string, bool) {
	line := getLine(text, pos.Line)
	if line == "" {
		return "", "", false
	}
	runes := []rune(line)
	if pos.Character > len(runes) {
		pos.Character = len(runes)
	}
	prefix := string(runes[:pos.Character])
	re := regexp.MustCompile(`([A-Za-z_][A-Za-z0-9_]*)\.([A-Za-z0-9_]*)$`)
	m := re.FindStringSubmatch(prefix)
	if len(m) != 3 {
		return "", "", false
	}
	return m[1], m[2], true
}

func getLine(text string, line int) string {
	if line < 0 {
		return ""
	}
	lines := strings.Split(text, "\n")
	if line >= len(lines) {
		return ""
	}
	return lines[line]
}

func moduleMembersFor(doc *analysis.Document, name string) ([]string, bool) {
	if doc != nil && doc.Index != nil {
		if !doc.Index.Imports[name] {
			if _, ok := stdModuleMembers()[name]; !ok {
				return nil, false
			}
		}
	}
	mems, ok := stdModuleMembers()[name]
	return mems, ok
}

var cachedStdMembers = buildStdModuleMembers()

func stdModuleMembers() map[string][]string {
	return cachedStdMembers
}

func buildStdModuleMembers() map[string][]string {
	m := map[string][]string{}
	add := func(name string, members map[string]object.Object) {
		var keys []string
		for k := range members {
			keys = append(keys, k)
		}
		m[name] = keys
	}

	add("fmt", general.Module().Members)
	add("arrays", arrays.Module().Members)
	add("fs", fs.Module().Members)
	add("hash", hash.Module().Members)
	add("json", JsonModule.JsonModule().Members)
	add("math", math.Module().Members)
	add("strings", stdstrings.Module().Members)
	add("time", time.Module().Members)
	add("os", osModule.Module().Members)
	add("http", net.HttpModule().Members)
	add("net", net.NetModule(nil).Members)

	return m
}
