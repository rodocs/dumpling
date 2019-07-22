+++
Target = "LocalizationTable.SetEntryValue"
Type = (key: string, source: string, context: string, localeId: string, text: string) => void
+++

Sets the text of the specified localeId in a LocalizationTable entry, using the specified _key_, _source_, and _context_ to narrow down the entry that will have this change applied.