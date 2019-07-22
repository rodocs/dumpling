+++
Target = "Instance.GetPropertyChangedSignal"
Type = { <T extends { [Key in keyof this]-?: Key extends "ClassName" | "GetPropertyChangedSignal" ? never : this[Key] extends RBXScriptSignal<Function, false> ? never : () => any extends this[Key] ? never : Key; }[keyof this]>(propertyName: T): RBXScriptSignal<Function, false>; (propertyName: string): RBXScriptSignal<Function, false>; <T extends { [Key in keyof this]-?: Key extends "ClassName" | "GetPropertyChangedSignal" ? never : this[Key] extends RBXScriptSignal<Function, false> ? never : () => any extends this[Key] ? never : Key; }[keyof this]>(propertyName: T): RBXScriptSignal<Function, false>; (propertyName: string): RBXScriptSignal<Function, false>; }
+++
