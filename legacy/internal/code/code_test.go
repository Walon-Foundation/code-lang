package code

import "testing"

func TestMake(t *testing.T) {
	tests := []struct {
		op       Opcode
		operand  []int
		expected []byte
	}{
		{OpConstant, []int{65534}, []byte{byte(OpConstant), 255, 254}},
		{OpAdd, []int{}, []byte{byte(OpAdd)}},
	}

	for _, tt := range tests {
		instruction := Make(tt.op, tt.operand...)

		if len(instruction) != len(tt.expected) {
			t.Errorf("instruction has wrong length. want=%d, got=%d", len(tt.expected), len(instruction))
		}

		for i, b := range tt.expected {
			if instruction[i] != tt.expected[i] {
				t.Errorf("wrong byte at pos %d, want=%d, got=%d", i, b, instruction[i])
			}
		}
	}
}

func TestInstructionString(t *testing.T) {
	instruction := []Instructions{
		Make(OpAdd),
		Make(OpConstant, 2),
		Make(OpConstant, 65535),
		Make(OpAdd),
	}

	expected := `0000 OpAdd
0001 OpConstant 2
0004 OpConstant 65535
0007 OpAdd
`

	concatted := Instructions{}

	for _, ins := range instruction {
		concatted = append(concatted, ins...)
	}

	if concatted.String() != expected {
		t.Errorf("instruction wrongly formatted.\nwant=%q\ngot=%q", expected, concatted.String())
	}
}

func TestReadOperands(t *testing.T) {
	tests := []struct {
		op        Opcode
		operand   []int
		bytesRead int
	}{
		{OpConstant, []int{65535}, 2},
	}

	for _, tt := range tests {
		instructions := Make(tt.op, tt.operand...)

		def, err := Lookup(byte(tt.op))
		if err != nil {
			t.Fatalf("definition not found: %q\n", err)
		}

		operandsRead, n := ReadOperands(def, instructions[1:])
		if n != tt.bytesRead {
			t.Fatalf("n wrong. want=%d, got=%d", tt.bytesRead, n)
		}

		for i, want := range tt.operand {
			if operandsRead[i] != want {
				t.Errorf("operand wrong. want=%d, got=%d", want, operandsRead[i])
			}
		}
	}
}
