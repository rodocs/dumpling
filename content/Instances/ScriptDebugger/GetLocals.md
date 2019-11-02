+++
Target = "ScriptDebugger.GetLocals"
Type = { (stackFrame?: number | undefined): Map<string, any>; (stackFrame?: number | undefined): Map<string, any>; }
+++

Returns a dictionary of all local variables in the specified stack, where the keys are the names of the variables, and the values are the actual values of the variables.