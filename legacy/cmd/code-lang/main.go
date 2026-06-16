package main

import (
	"fmt"
	"os"
	"os/user"
	"path/filepath"

	"github.com/walonCode/code-lang/internal/repl"
)

var (
	Version = "dev"
	Commit = "none"
)

func main() {
	
	if len(os.Args) > 1 {
		switch os.Args[1]{
			case "-v", "--version":
				fmt.Printf("code-lang %s %s\n", Version, Commit)
			default:
				runFile(os.Args[1])
		}
	}else {
		runRepl()
	}	
}

func runFile(path string){
	if filepath.Ext(path) != ".cl" {
		fmt.Fprintf(os.Stderr, "Error: File %s must have a .cl extension\n", path)
		os.Exit(1)
	}
	
	file, err := os.ReadFile(path)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: could not open file %s\n",path)
		os.Exit(1)
	}
	
	repl.Execute(string(file), os.Stdout)
}

func runRepl(){
	usr, err := user.Current()
	if err != nil {
		panic(err)
	}

	fmt.Printf("Hello %s! This is the Code-Lang Programming Language\n", usr.Username)
	fmt.Printf("Feel free to start type in the commands\n")

	repl.Start(os.Stdout)
}