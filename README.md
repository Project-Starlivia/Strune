# Strune
Strune is a **simple, directional knowledge structure**.
It is primarily intended to function as a word book or a lightweight linked reference, but how you use it is entirely up to you.

[日本語](./README.md)
## Data Structure
```json
{
    "label": "string",
    "description": "string",
    "dependence": label[],
    "options": any
}
```
- label: The name of the knowledge node.
- description: A short explanation of the concept, typically within 200 characters.
- dependence: A list of parent concepts from the perspective of this node.This field contains an array of other labels (strings).
- options: A freely extensible field for user-defined metadata.

Note that dependence is not merely a relational link.
Interpretation is ultimately up to the user, but by default it represents elements that compose or contain this knowledge.

Examples:
- `Unity` would have `GameEngine` listed in its dependence.
- `Blender` and `FBX` would list each other in their dependence fields.

## Isn’t this ambiguous?
Yes. Knowledge rarely fits into clean parent–child relationships.
Strune intentionally embraces this ambiguity and aims to visualize loosely directional conceptual structures, even if they don’t map perfectly.

Its core strength lies in its simplicity and directionality, and the expressive power that emerges from them.
# Packages
This project is currently organized as a monorepo.
Rust is used as the primary implementation language simply because I wanted to try writing it.

> ## ⚠️ Beta Notice
> The following components are still in beta and may undergo breaking API changes.
The data structure itself will not change.

## Markdown Parser – MdParser

A library that converts extended Markdown into the Strune JSON format.

Writing raw JSON is tedious, and building a full editor felt excessive.
So Strune introduces a minimal Markdown-based syntax:

```markdown
# <label>
## description
<description>
## dependence
- <dependence01>
- <dependence02>
or
- [[<dependence01>]]
- [[<dependence02>]]
## options
<options>
```

Other than labels are optional.
For Obsidian compatibility, the `[[<dependence01>]]` notation is also supported.

## Viewer – PerseusView

A static-site viewer built with Perseus, including some animation work.