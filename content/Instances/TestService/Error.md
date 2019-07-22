+++
Target = "TestService.Error"
Type = (description: string, source?: Instance | undefined, line?: number | undefined) => void
+++

Prints a red message to the output, prefixed by `TestService: `.