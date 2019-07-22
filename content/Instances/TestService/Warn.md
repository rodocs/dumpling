+++
Target = "TestService.Warn"
Type = (condition: boolean, description: string, source?: Instance | undefined, line?: number | undefined) => void
+++

If condition is true, prints Warning passed: , followed by description, to the output, in blue text. Otherwise, prints Warning: , followed by description, to the output, in yellow text.