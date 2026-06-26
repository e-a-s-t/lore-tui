---
layout: default
title: Getting Started
---

# Getting Started

## Install Lore

```shell
brew tap e-a-s-t/tap
brew install e-a-s-t/tap/lore
```

## Initialize a repository

```bash
lore init
```

This creates the `.lore/` directory structure.

## Create your first feature

```bash
lore feature new "User Authentication"
```

Then add supporting artifacts:

```bash
lore req new "Support OAuth login"
lore story new "Implement OAuth callback"
lore adr new "Use Keycloak as identity provider"
lore test new "OAuth login succeeds"
```
