+++
Target = "DebuggerManager.AddDebugger"
Type = (script: Instance) => Instance | undefined
+++

Registers a script to be used in the Lua Debugger. Returns a `ScriptDebugger` for the script.