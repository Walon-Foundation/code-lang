# Security Policy

## Supported Versions

Code-Lang is currently in active development. Only the latest version on the `main` branch receives security fixes.

| Version | Supported |
|---|---|
| latest (`main`) | Yes |
| older releases | No |

## Reporting a Vulnerability

**Do not open a public GitHub issue for security vulnerabilities.**

Report vulnerabilities privately via one of:

- **GitHub Private Vulnerability Reporting:** [github.com/Walon-Foundation/code-lang/security/advisories/new](https://github.com/Walon-Foundation/code-lang/security/advisories/new)
- **Email:** security issues can be raised through the GitHub Security tab above

### What to include

- A description of the vulnerability and its potential impact
- Steps to reproduce or a minimal proof-of-concept
- The version or commit hash you tested against
- Any suggested mitigations if you have them

### What to expect

- Acknowledgement within 72 hours
- A status update within 7 days
- A fix or mitigation plan communicated before any public disclosure

We appreciate responsible disclosure and will credit reporters in the release notes unless you prefer to remain anonymous.

## Scope

The following are in scope:

- The interpreter (lexer, parser, evaluator)
- The standard library modules
- The REPL

The following are out of scope:

- The `legacy/` directory (Go implementation, no longer maintained)
- Third-party dependencies (report those to the relevant upstream project)
