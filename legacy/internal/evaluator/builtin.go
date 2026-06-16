package evaluator

import (
	"github.com/walonCode/code-lang/internal/std/arrays"
	"github.com/walonCode/code-lang/internal/std/fs"
	"github.com/walonCode/code-lang/internal/std/general"
	"github.com/walonCode/code-lang/internal/std/hash"
	"github.com/walonCode/code-lang/internal/std/json"
	"github.com/walonCode/code-lang/internal/std/math"
	"github.com/walonCode/code-lang/internal/std/net"
	"github.com/walonCode/code-lang/internal/std/os"
	"github.com/walonCode/code-lang/internal/std/strings"
	"github.com/walonCode/code-lang/internal/std/time"
)

func init() {
	// preload standard library modules
	e := Evaluator{}
	moduleCache["arrays"] = arrays.Module()
	moduleCache["fmt"] = general.Module()
	moduleCache["http"] = net.HttpModule()
	moduleCache["json"] = json.JsonModule()
	moduleCache["net"] = net.NetModule(e.applyFunction)
	moduleCache["fs"] = fs.Module()
	moduleCache["math"] = math.Module()
	moduleCache["strings"] = strings.Module()
	moduleCache["time"] = time.Module()
	moduleCache["hash"] = hash.Module()
	moduleCache["os"] = os.Module()
}
