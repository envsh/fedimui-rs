package nulogic

import "testing"

func BenchmarkX100(b *testing.B) {
	for i := 0; i < b.N; i++ {
		ffipxygocxrswrap(nil)
	}
}
