+++
Target = "LocalizationTable.SetEntryContext"
Type = (key: string, source: string, context: string, newContext: string) => void
+++

Sets the **Context** field of a LocalizationTable entry to _newContext_, using the specified _key_, _source_, and _context_ to narrow down the entry that will have this change applied.