+++
Target = "ScriptDebugger.SetBreakpoint"
Type = (line: number) => Instance | undefined
+++

Sets the specified line of the script as a breakpoint. Returns a `DebuggerBreakpoint` that you can use to manage the breakpoint.