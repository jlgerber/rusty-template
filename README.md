# rusty-template

String templating a la Jinja with pipelined functions and optionals.

## Templates & Variables
The templates themselves should be familiar to anyone who has used Jinja. Embed variables in the template by enclosing them in brackets ({}).

## Pipelined functions
In addition to variables, templates may define one or more pipelined functions with the signature ```func (item: String) -> String```. Upon template evaluation, the variable is resolved and then handed off to any pipelined function or functions, before acreting in the result string. For example:
```
"/job/home/{ username | lower_case }"
```
Specifies a template with a variable named username, which would be resolved and then passed through a function called ```loser_case```.

## Optional Variables

Variables with question mark suffixes are conditional. If the parser is unable to resolve them, then a blank string is returned. If there are pipelined functions, in the event that an optional variable fails to resolve, no further progress is made; the function(s) do not get called.

# Default Filter Functions

The package consists of a set of default filter functions:
- upper: Converts input to upper case
- dot_to_slash: Converts any periods to slashes
- slash: Appends a slash to the end of the variable
- hash: Generates a seahash from the input.
