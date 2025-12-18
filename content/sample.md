# Strune
## description
Strune is a simple, directional knowledge structure.
## dependencies
- Rust
- Project Starlivia
- Structural Data Format
- Knowledge Graph
## options
### slug
index

# Project Starlivia
## dependencies
へのへのん
## description
> Oss libraries from @henohenon
https://github.com/Project-Starlivia
## options
### slug
project-starlivia

# へのへのん
## description
ぱんじー
https://henohenon.github.io/
## options
### slug
henohenon

# Rust
> A language empowering everyone to build reliable and efficient software.
https://rust-lang.org/
## dependencies
- Programming Language

# TypeScript
> TypeScript is JavaScript with syntax for types.
https://www.typescriptlang.org/
## dependencies
- Programming Language
- JavaScript


# Programming Language
## description
A formal language comprising a set of instructions that produce various kinds of output. Programming languages are used to create programs that implement specific algorithms and control the behavior of machines.
## options
### slug
programming-language

# Programming Language?
## description
Languages that may be considered programming languages but have some debate around their classification, such as markup languages or domain-specific languages.
## dependencies
Programming Language
## options
### slug
programming-languageQ

# core Web Language
## description
The fundamental languages that form the basis of web development: HTML for structure, CSS for presentation, and JavaScript for behavior. These three languages work together to create web pages and applications.
## dependencies
Programming Language
## options
### slug
core-web-language


# HTML
## description
> HTML (HyperText Markup Language) is the most basic building block of the Web. It defines the meaning and structure of web content. Other technologies besides HTML are generally used to describe a web page's appearance/presentation (CSS) or functionality/behavior (JavaScript).
https://developer.mozilla.org/en-US/docs/Web/HTML
## dependencies
- Programming Language?
- core Web Language
- Structural Data Format

# CSS
## dependencies
- Programming Language?
- core Web Language
## description
> Cascading Style Sheets (CSS) is a stylesheet language used to describe the presentation of a document written in HTML or XML (including XML dialects such as SVG, MathML or XHTML). CSS describes how elements should be rendered on screen, on paper, in speech, or on other media.
https://developer.mozilla.org/en-US/docs/Web/CSS

# JavaScript
## dependencies
- Programming Language
- core Web Language
## description
> Cascading Style Sheets (CSS) is a stylesheet language used to describe the presentation of a document written in HTML or XML (including XML dialects such as SVG, MathML or XHTML). CSS describes how elements should be rendered on screen, on paper, in speech, or on other media.
https://developer.mozilla.org/en-US/docs/Web/JavaScript

# crates/tera-render
## description
> A rendering engine that generates static HTML sites from Strune nodes using the Tera template engine.
https://github.com/Project-Starlivia/Strune?tab=readme-ov-file#tera-render
## dependencies
- Strune
- tera
- HTML
- CSS
## options
### slug
crates_sailfish-render


# tera
## description
> A powerful, easy to use template engine for Rust Inspired by Jinja2 and Django templates
https://keats.github.io/tera/
## dependencies
- Rust
- Template Engine
- HTML
- jinja2

# jinja2
## description
> Jinja2 is a template engine written in pure Python. It provides a Django inspired non-XML syntax but supports inline expressions and an optional sandboxed environment.
https://github.com/noirbizarre/jinja2
## dependencies
- Template Engine
- HTML

# Template Engine
## description
A software component that combines templates with data to produce output documents. Template engines are commonly used to generate HTML, configuration files, or other text-based formats by separating logic from presentation.
## options
### slug
template_engine

# rust-sailfish
## description
> Simple, small, and extremely fast template engine for Rust
https://github.com/rust-sailfish/sailfish
## dependencies
- Rust
- Template Engine


# cargo
## description
> Cargo is the Rust package manager. It downloads dependencies, compiles packages, makes distributable packages, and uploads them to crates.io.
https://doc.rust-lang.org/cargo/
## dependencies
- Rust


# serde
## description
> Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.
https://serde.rs/
## dependencies
- Rust


# SSG
## description
Static Site Generator - A tool that generates a complete static website based on raw data and templates. SSGs produce HTML files that can be deployed directly to a web server without requiring server-side processing.
## options
### slug
ssg


# hugo
## description
> The world's fastest framework for building websites. Hugo is a static site generator written in Go, known for its speed and flexibility.
https://gohugo.io/
## dependencies
- SSG


# astro
## description
> Astro is a modern static site builder that allows you to use your favorite JavaScript framework (React, Vue, Svelte, etc.) and delivers zero JavaScript by default for faster sites.
https://astro.build/
## dependencies
- SSG
- JavaScript
- TypeScript


# jekyll
## description
> Jekyll is a simple, blog-aware static site generator written in Ruby. It's the engine behind GitHub Pages.
https://jekyllrb.com/
## dependencies
- SSG


# Git
## description
> Git is a distributed version control system for tracking changes in source code during software development. It's designed for coordinating work among programmers.
https://git-scm.com/


# GitHub
## description
> GitHub is a web-based platform for version control and collaboration using Git. It provides hosting for software development and offers distributed version control plus access control, bug tracking, and feature requests.
https://github.com/
## dependencies
- Git


# runes
## description
Ancient symbols used by Germanic peoples long ago. The Elder Futhark has 24 runes.

# ᛝ
## description
>>> ᛝ Ing ƿæs ærest mid Eástdenum
geseƿen secgum, oð he síððan e[á]st
ofer ƿæg geƿát. ƿæn æfter ran.
þus Heardingas þone hæle nemdon.
https://ja.wikipedia.org/wiki/%E1%9B%9C
## dependencies
- runes
## options
### slug
ingwaz


# crates/loader
## description
> A library for loading Strune nodes from various file formats into the core Node structure.
https://github.com/Project-Starlivia/Strune?tab=readme-ov-file#loader
## dependencies
- Strune
## options
### slug
crates_loader


# crates/operation
## description
> Provides operations for analyzing and manipulating Strune node graphs. This package introduces trait-based extensibility for adding computed fields to nodes.
https://github.com/Project-Starlivia/Strune?tab=readme-ov-file#operation
## dependencies
- Strune
## options
### slug
crates_operation

# cli
## description
> Command-line interface for Strune. This is the main executable that ties all packages together.
https://github.com/Project-Starlivia/Strune?tab=readme-ov-file#cli
## dependencies
- Strune
## options
### slug
strune_cli


# tera-render/ingwaz
## description
One of the tera-render styles. It strongly expresses the connections between nodes.
## dependencies
- crates/tera-render
- ᛝ
## options
### slug
tera-render_ingwaz


# JSON
## description
> JSON (JavaScript Object Notation) is a lightweight data interchange format that is easy for humans to read and write and easy for machines to parse and generate.
https://www.json.org/
## dependencies
- Structural Data Format


# Markdown
## description
> Markdown is a lightweight markup language with plain text formatting syntax. It's designed to be converted to HTML and many other formats, widely used for documentation and content creation.
https://daringfireball.net/projects/markdown/
## dependencies
- Structural Data Format

# strune-core
## description
> The core data structure library for Strune. This package defines the fundamental Node<T> type that all other packages depend on.
https://github.com/Project-Starlivia/Strune?tab=readme-ov-file#strune_core
## dependencies
- Strune
## options
### slug
strune-core


# regex
## description
> A Rust library providing regular expression support with a focus on performance and correctness. Used for pattern matching and text processing.
https://docs.rs/regex/
## dependencies
- Rust


# once_cell
## description
> A Rust library for single-assignment cells and lazy statics without macros, providing safe one-time initialization.
https://docs.rs/once_cell/
## dependencies
- Rust


# thiserror
## description
> A Rust library that provides a convenient derive macro for the standard library's std::error::Error trait, making error handling more ergonomic.
https://docs.rs/thiserror/
## dependencies
- Rust


# obsidian
## description
> Obsidian is a knowledge base application that works on local Markdown files. It features a powerful linking system and graph view for visualizing connections between notes.
https://obsidian.md/
## dependencies
- Markdown
- Knowledge Graph


# Zola
## description
> A fast static site generator written in Rust. Zola uses the Tera template engine and compiles to a single binary with no dependencies.
https://www.getzola.org/
## dependencies
- Rust
- SSG
- tera


# mdBook
## description
> A utility to create modern online books from Markdown files, written in Rust. It's used extensively in the Rust community for documentation.
https://rust-lang.github.io/mdBook/
## dependencies
- Rust
- SSG
- Markdown


# Knowledge Graph
## description
> A knowledge graph is a network of entities, their semantic types, properties, and relationships. It's used to represent knowledge in a structured, machine-readable format that shows how concepts are connected.
## options
### slug
knowledge-graph

# Structural Data Format
## description
A data description format for structurally expressing data and resolving it between systems
## options
### slug
structural-data-format
