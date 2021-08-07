package y2015

import (
	"fmt"
	"io/ioutil"
	"os"
	"testing"

	"github.com/stretchr/testify/require"
)

type answer []string

var answers = []answer{
	answer{"232", "1783"},
	answer{"1588178", "3783758"},
	answer{"2565", "2639"},
	answer{"282749", "9962624"},
	answer{"238", "69"},
	answer{"400410", "15343601"},
	answer{"956", "40149"},
	answer{"1342", "2074"},
	answer{"207", "804"},
	answer{"360154", "5103798"},
	answer{"vzbxxyzz", "vzcaabcc"},
	answer{"111754", "65402"},
	answer{"664", "640"},
	answer{"2640", "1102"},
	answer{"13882464", "11171160"},
	answer{"213", "323"},
	answer{"1304", "18"},
	answer{"814", "924"},
	answer{"576", "207"},
	answer{"786240", "831600"},
	answer{"78", "148"},
	answer{"1269", "1309"},
	answer{"184", "231"},
	answer{"11266889531", "77387711"},
	answer{"8997277", "0"},
}

func readFile(path string) (string, error) {
	file, err := os.Open(path)
	if err != nil {
		return "", err
	}

	b, err := ioutil.ReadAll(file)
	if err = file.Close(); err != nil {
		return "", err
	}

	if b[len(b)-1] == byte(10) {
		b = b[0 : len(b)-1]
	}

	return string(b), err
}

func TestAllDays(t *testing.T) {
	for i, a := range answers {
		t.Run(fmt.Sprintf("Day-%d", i+1), func(t *testing.T) {
			t.Parallel() // test parallelization!
			input, err := readFile(fmt.Sprintf("./testdata/%d.txt", i+1))
			require.NoError(t, err, "should not error parsing test input")

			out, err := Call(fmt.Sprint(i+1), input)
			require.NoError(t, err, "should not error processing test input")

			require.Equal(t, a[0], fmt.Sprint(out[0]), "answer 1 should match expected")
			require.Equal(t, a[1], fmt.Sprint(out[1]), "answer 2 should match expected")
		})
	}
}
