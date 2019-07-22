+++
Target = "TestService.Check"
Type = (condition: boolean, description: string, source?: Instance | undefined, line?: number | undefined) => void
+++

If condition is true, prints "Check passed: ", followed by description to the output, in blue text. Otherwise, prints "Check failed: ", again, followed by description, but in red text.