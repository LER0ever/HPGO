package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"
	"strings"

	"github.com/alecthomas/participle"
	"github.com/alecthomas/participle/lexer"
	"github.com/alecthomas/participle/lexer/ebnf"
	"github.com/sirupsen/logrus"
	"github.com/tidwall/pretty"
)

var HLOLexer = lexer.Must(ebnf.New(`
Comment = ("#" | "//") { "\u0000"…"\uffff"-"\n" } .

ConvDimLabel = char char char char "_" char char char char Rightarrow char char char char .
ConvPadSize = digit ("x" | "_") digit {("x" | "_") digit} .

Ident = (alpha | "_") { "." | "_" | "-" | alpha | digit } .
String = "\"" {"'" | Ident | Number | "/" | "," | "$" | "{" | "}" | ":" } "\"" .
VarName = "%" Ident .
Boolean = ("true" | "false") .

Number = { "-" } ("." | digit | "inf") {"." | digit} .
Whitespace = " " | "\t" | "\n" | "\r" .
Rightarrow = "->" .
Assign = "=" .
Punct = "!"…"/" | ":"…"@" | "["…"_" | "{"…"~" .
char = alpha | digit .
alpha = "a"…"z" | "A"…"Z" .
digit = "0"…"9" .
`))

//SubString = "\\\"" {Ident | "/" | "$" | "{" | "}" | ":" } "\\\"" .

var log = logrus.New()

type HLORoot struct {
	Functions []HLOFunction `@@*`
}

type HLOFunction struct {
	Name        string        `("ENTRY")? @VarName`
	Params      []Param       `"(" [ @@ { "," @@ } ] ")"`
	ReturnTypes []Type        `"->" ( "(" [ @@ { "," @@ } ] ")" | @@)`
	Body        []Instruction `"{" @@ {@@} "}"`
}

type Instruction struct {
	VarName string       `("ROOT")? @VarName "="`
	Fn      FunctionCall `@@`
	Meta    []Meta       `{ "," @@ }`
}

type FunctionCall struct {
	ReturnType Type       `@@`
	Name       string     `@Ident`
	Argument   []Argument `"(" ( @@ { "," @@ } )? ")"`
}

type Meta struct {
	Key   string `@Ident "="`
	Value *Value `@@`
}

type Value struct {
	Number  int32   `  @Number`
	String  *string `| (@Ident|@VarName|@String)`
	Numbers []int32 `| ("{" @Number {"," @Number } "}")`
	Dicts   []Dict  `| ("{" { @@ } "}")`
	Slices  []Slice `| ("{" @@ {"," @@ } "}")`
	Boolean *bool   `| ("{" (@"true" | "false") "}")`
	Misc    *string `| ( @ConvPadSize | @ConvDimLabel )`
}

type Dict struct {
	Key   string `@Ident "="`
	Value *Value `@@`
}

type Slice struct {
	Start int32 `"[" @Number ":"`
	End   int32 `@Number "]"`
}

type Param struct {
	Name string `@Ident ":"`
	Type Type   `@@`
}

type Argument struct {
	Type Type   `(@@)?`
	Name string `@VarName | @Number | @Ident`
}

type Type struct {
	DataType   string  `(   @Ident`
	Dimensions []int32 `    "[" [ @Number { "," @Number } ] "]"`
	Layout     []int32 `    ("{" [ @Number { "," @Number } ] "}")?`
	TupleType  []Type  `)|  "(" @@ { "," @@ } ")"`
}

func preprocess(s string) string {
	s = strings.Replace(s, "\\\"", "'", -1)
	return s
}

func parse(s string) *HLORoot {
	parser, err := participle.Build(&HLORoot{},
		participle.Lexer(HLOLexer),
		participle.Elide("Comment", "Whitespace"),
		//participle.UseLookahead(3),
	)

	if err != nil {
		panic(err)
	}
	hlo := &HLORoot{}

	l, _ := HLOLexer.Lex(strings.NewReader(s))
	tokens, err := lexer.ConsumeAll(l)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", tokens)

	err = parser.Parse(strings.NewReader(s), hlo)
	if err != nil {
		panic(err)
	}

	// repr.Println(hlo, repr.Indent("  "), repr.OmitEmpty(true))
	return hlo
}

func main() {
	// log.Level = logrus.DebugLevel

	// hlo_file := "hlo.txt"
	hlo_file := "hlo.txt"
	if len(os.Args) >= 2 {
		hlo_file = os.Args[1]
	}

	log.Println("Reading HLO Text File...")
	content, err := ioutil.ReadFile(hlo_file)
	if err != nil {
		_ = fmt.Errorf(err.Error())
	}
	text := string(content)
	text = preprocess(text)

	log.Println("Parsing HLO into AST...")
	ast := parse(text)
	log.Debugf("%+v\n", ast)
	// fmt.Printf("Enriched AST: %s\n", spew.Sdump(enrichAST(ast)))
	ast_string, _ := json.Marshal(ast)
	// TODO: no error handling at all
	ast_json_prettified := pretty.Pretty(ast_string)

	fmt.Printf("%s", ast_json_prettified)
}
