# Recipe Format

- A recipe must be a markdown file (file extension `.md`)
- It must contain at least one ingredient
- Each ingredient is denoted with a dash, constituting a list
- Each ingredient _may_ have a [quantity](/src/qty.rs) attached to it, which comes right after the name of the ingredient (seprated by a comma)
- A quantity can be either a weight (200 g), a volume (40 cl), a plain number (6) or a custom quantity (1 handful)

Examples of listed ingredients may look like this

```markdown
- carrots, 300 g
- potatoes, 2
- honey, 5 cl
- olive oil, 2 dl
- rosemary, 3 tsp
- salt
```

All lines that are not preceeded by a dash are ignored. See [example.md](example.md) for a full example.
