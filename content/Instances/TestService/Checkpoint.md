+++
Target = "TestService.Checkpoint"
Type = (text: string, source?: Instance | undefined, line?: number | undefined) => void
+++

Prints "Test checkpoint: ", followed by text, to the output, in blue text.