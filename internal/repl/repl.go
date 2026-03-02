package repl

import (
	"fmt"
	"io"
	"os"
	"path/filepath"

	"github.com/chzyer/readline"
	"github.com/walonCode/code-lang/internal/compiler"
	"github.com/walonCode/code-lang/internal/lexer"
	"github.com/walonCode/code-lang/internal/object"
	"github.com/walonCode/code-lang/internal/parser"
	"github.com/walonCode/code-lang/internal/std/general"
	"github.com/walonCode/code-lang/internal/symbol"
	"github.com/walonCode/code-lang/internal/vm"
)

const PROMPT = ">> "

func Start(out io.Writer) {
	env := object.NewEnvironment()

	// Inject basic builtins for the REPL
	genMod := general.Module()
	for name, obj := range genMod.Members {
		env.Set(name, obj)
	}

	builder := symbol.NewBuilder()
	// Pre-populate symbol table with builtins
	for name := range genMod.Members {
		builder.Define(name, symbol.FUNCTION)
	}

	home, _ := os.UserHomeDir()
	historyPath := filepath.Join(home, ".code_lang_history")

	r1, err := readline.NewEx(&readline.Config{
		Prompt:          PROMPT,
		HistoryFile:     historyPath,
		InterruptPrompt: "^C",
	})

	if err != nil {
		panic(err)
	}

	defer r1.Close()

	for {
		line, err := r1.Readline()

		if err == readline.ErrInterrupt {
			fmt.Println("Exiting...")
			break
		}

		if line == "exit()" {
			fmt.Println("Exiting...")
			os.Exit(0)
		}

		l := lexer.New(line)
		p := parser.New(l)

		programe := p.ParsePrograme()
		if len(p.Errors()) != 0 {
			printParserError(out, p.Errors())
			continue
		}

		// Run static analysis
		builder.Visit(programe)
		if len(builder.Errors) != 0 {
			printSymbolError(out, builder.Errors)
			builder.Errors = nil // Clear errors for next line
			continue
		}
		
		comp := compiler.New()
		err = comp.Compile(programe)
		if err != nil {
			fmt.Fprintf(out, "whoops! compilation failed:\n %s\n",err)
			continue
		}
		
		machine := vm.New(comp.ByteCode())
		err = machine.Run()
		if err != nil {
			fmt.Fprintf(out, "Woops! Executing bytecode failed:\n %s\n", err)
			continue
		}
		
		stackTop := machine.StackTop()
		io.WriteString(out, stackTop.Inspect())
		io.WriteString(out, "\n")
	}
}

func printSymbolError(out io.Writer, errors []string) {
	io.WriteString(out, " static analysis errors:\n")
	for _, msg := range errors {
		io.WriteString(out, "\t"+msg+"\n")
	}
}

const CODE_LANG = `
______          __                 __                         
/ ____/  ____   / /  ___           / /   ____ _   ____    ____ _
/ /      / __ \ / /  / _ \  ______ / /   / __ /  / __ \  / __ /
/ /___   / /_/ // /  /  __/ /_____// /___/ /_/ /  / / / / / /_/ / 
\____/   \____//_/   \___/        /_____/\__,_/  /_/ /_/  \__, /  
                                                      /____/   

`

func printParserError(out io.Writer, errors []string) {
	io.WriteString(out, CODE_LANG)
	io.WriteString(out, "Whoops! We can in to some Code_lang business!\n")
	io.WriteString(out, " parser errors:\n")
	for _, msg := range errors {
		io.WriteString(out, "\t"+msg+"\n")
	}
}

func Execute(source string, out io.Writer) {
	l := lexer.New(source)
	p := parser.New(l)
	program := p.ParsePrograme()

	if len(p.Errors()) != 0 {
		printParserError(out, p.Errors())
		return
	}

	env := object.NewEnvironment()
	genMod := general.Module()
	for name, obj := range genMod.Members {
		env.Set(name, obj)
	}

	builder := symbol.NewBuilder()
	for name := range genMod.Members {
		builder.Define(name, symbol.FUNCTION)
	}

	builder.Visit(program)
	if len(builder.Errors) != 0 {
		printSymbolError(out, builder.Errors)
		return
	}

	comp := compiler.New()
	err := comp.Compile(program)
	if err != nil {
		fmt.Fprintf(out, "whoops! compilation failed:\n %s\n",err)
	}
	
	machine := vm.New(comp.ByteCode())
	err = machine.Run()
	if err != nil {
		fmt.Fprintf(out, "Woops! Executing bytecode failed:\n %s\n", err)
	}
	
	stackTop := machine.StackTop()
	io.WriteString(out, stackTop.Inspect())
	io.WriteString(out, "\n")
}
