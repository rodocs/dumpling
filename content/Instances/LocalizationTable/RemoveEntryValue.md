+++
Target = "LocalizationTable.RemoveEntryValue"
Type = (key: string, source: string, context: string, localeId: string) => void
+++

Removes a single language translation from the LocalizationTable, using the provided _key_, _source_, _context_, and _localeId_ to narrow down the specific entry to be removed.