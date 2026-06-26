# Terminal Guide

## Create artifacts

```bash
lore feature new "Authentication"
lore req new "Support OAuth"
lore story new "Implement callback"
lore adr new "Use Keycloak"
lore test new "OAuth login"
```

## Link artifacts

```bash
lore link FEATURE-001 REQ-001
lore link FEATURE-001 STORY-001
lore link FEATURE-001 ADR-001
lore link FEATURE-001 TEST-001
```

## Browse

```bash
lore show FEATURE-001
lore search oauth
lore validate
lore validate --fix
lore trace
lore gaps
lore ui
```
