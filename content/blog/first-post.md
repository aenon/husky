---
title: My First Blog Post
template: default.html
date: 2025-04-10
---

# First Blog Post

This is my first blog post using this static site generator.

## Code Example

```rust
fn hello() {
    println!("Hello from my blog!");
}
```

The content demonstrates:
1. Required YAML frontmatter (as seen in the `parse_markdown` function)
2. Basic markdown features
3. Template specification (matching the generator's template handling)

Note that the generator will:
1. Find these files using `discover_content`
2. Parse them with `parse_markdown`
3. Process them with TypeScript (if configured)
4. Apply the specified template

Make sure you also have your template file set up, but since the template implementation isn't shown in the provided code, you'll need to check your full codebase for the expected template format.