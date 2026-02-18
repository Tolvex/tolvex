---
date: 2026-02-18
authors:
  - tolvex_team
categories:
  - Announcements
  - Release
tags:
  - rebrand
  - tolvex
  - v0.1.6
---

# Introducing Tolvex: The Evolution of Healthcare Programming

**TL;DR**: Medi has been rebranded to **Tolvex**. New repository: [github.com/Tolvex/tolvex](https://github.com/Tolvex/tolvex)

---

We're excited to announce a significant milestone in our project's journey: **Medi is now Tolvex**.

<!-- more -->

## Why Tolvex?

When we started this project as "Medi," our vision was to create a programming language purpose-built for healthcare analytics. As the project evolved, we realized our scope and ambitions had grown beyond what the original name represented.

**Tolvex** better captures our mission: to empower healthcare with secure, fast, and clinician-friendly analytics. The name reflects:

- **Innovation**: A fresh identity for a modern healthcare programming language
- **Clarity**: Distinct branding that stands out in the healthcare tech ecosystem
- **Global Appeal**: A name that resonates across international healthcare communities
- **Trademark Protection**: Stronger legal protection for our growing ecosystem

## What's Changed?

We've completed a comprehensive rebranding across the entire project:

### Tooling & CLI

| Component | Old | New |
|-----------|-----|-----|
| **Compiler** | `medic` | `tlvxc` |
| **Package Manager** | `medipack` | `tvx` |
| **File Extension** | `.medi` | `.tlvx` |
| **Manifest File** | `medi.toml` | `formula.toml` |
| **Package Registry** (planned) | `medipacks.io` | `formulary.tolvex.dev` |

### Example: Before & After

**Before (Medi):**
```bash
# Install a package
medipack add fhir-utils

# Compile code
medic build app.medi
```

**After (Tolvex):**
```bash
# Install a formula
tvx add fhir-utils

# Compile code
tlvxc build app.tlvx
```

### Ecosystem Updates

- **Compiler crates**: `medic_*` → `tlvxc_*`
- **Standard library**: `medi_*` → `tolvex_*`
- **Python bindings**: `pymedi` → `pytolvex`
- **Namespaces**: `medi::*` → `tolvex::*`
- **GitHub**: [MediLang/medi](https://github.com/MediLang/medi) → [Tolvex/tolvex](https://github.com/Tolvex/tolvex)
- **Website**: medi-lang.org → [tolvex.dev](https://tolvex.dev)
- **Social**: @MediLangHQ → [@TolvexLang](https://twitter.com/TolvexLang)

## Release v0.1.6: The Rebranding Release

Today we're releasing **v0.1.6**, which marks the official transition to Tolvex. This release includes:

✅ Complete rebranding of all tooling and documentation  
✅ Updated CLI tools (`tlvxc` and `tvx`)  
✅ Renamed package ecosystem ("formulas" on `formulary.tolvex.dev`)  
✅ Migrated repository with full commit history preserved  
✅ Updated Python bindings (`pytolvex`)  

**All historical releases (v0.0.1 - v0.1.15) have been preserved** in the new repository.

[View the full changelog →](https://github.com/Tolvex/tolvex/blob/main/CHANGELOG.md#v016---2026-02-18)

## Migration Guide

Migrating your existing Medi code to Tolvex is straightforward:

### 1. Update Repository

```bash
# Clone the new repository
git clone https://github.com/Tolvex/tolvex.git
cd tolvex

# Build the tools
cargo build --workspace --release
```

### 2. Rename Files

```bash
# Rename .medi files to .tlvx
find . -name "*.medi" -exec rename 's/\.medi$/.tlvx/' {} \;

# Rename medi.toml to formula.toml
mv medi.toml formula.toml
```

### 3. Update Code

```tlvx
// Old imports
use medi::core::*;
use medi_stats::mean;

// New imports
use tolvex::core::*;
use tolvex_stats::mean;
```

### 4. Update Commands

```bash
# Old
medic build
medipack add fhir-utils

# New
tlvxc build
tvx add fhir-utils
```

For detailed migration instructions, see our [Migration Guide](https://github.com/Tolvex/tolvex/blob/main/MIGRATION.md).

## What Stays the Same?

While the name has changed, our core mission remains unchanged:

✅ **Healthcare-first design**: Built for FHIR, HL7, DICOM, and clinical workflows  
✅ **Privacy by default**: Compile-time privacy tracking and HIPAA compliance  
✅ **Performance**: Fast execution for real-time healthcare analytics  
✅ **Clinician-friendly**: Designed for healthcare professionals, not just developers  
✅ **Open source**: MIT/Apache-2.0 licensed  

All the features you rely on—privacy tracking, healthcare standards support, statistical analysis, and more—continue to evolve under the Tolvex name.

## Looking Forward

This rebrand marks the beginning of an exciting new chapter. Here's what's coming:

### Short-term (Q1 2026)
- 🚀 Launch of `formulary.tolvex.dev` package registry
- 📦 First community-contributed formulas
- 📚 Expanded documentation and tutorials
- 🔧 Enhanced IDE support and tooling

### Medium-term (Q2-Q3 2026)
- 🌐 WebAssembly compilation support
- 🔐 Advanced privacy features (differential privacy, federated learning)
- 📊 Integrated data visualization
- 🏥 Healthcare institution pilot programs

### Long-term (2026+)
- 🎯 Self-hosting compiler in Tolvex
- 🌍 Multi-language interoperability
- 🤖 AI-assisted healthcare code generation
- 🏆 Production deployments in clinical settings

## Join the Tolvex Community

We're building more than a programming language—we're building a community of healthcare innovators, data scientists, and clinicians who believe in better tools for better healthcare.

**Get involved:**

- 🌟 Star us on GitHub: [github.com/Tolvex/tolvex](https://github.com/Tolvex/tolvex)
- 💬 Join discussions: [github.com/Tolvex/tolvex/discussions](https://github.com/Tolvex/tolvex/discussions)
- 🐦 Follow us: [@TolvexLang](https://twitter.com/TolvexLang)
- 📖 Read the docs: [tolvex.dev/docs](https://tolvex.dev/docs)
- 🤝 Contribute: [Contributing Guide](https://github.com/Tolvex/tolvex/blob/main/CONTRIBUTING.md)

## Thank You

To everyone who has supported the Medi project—contributors, early adopters, and community members—**thank you**. Your feedback, code contributions, and enthusiasm have shaped this project into what it is today.

We're excited to continue this journey as **Tolvex** and look forward to building the future of healthcare programming together.

---

**Questions about the migration?** Check out our [Migration FAQ](https://github.com/Tolvex/tolvex/discussions) or open an [issue](https://github.com/Tolvex/tolvex/issues).

**— The Tolvex Team**

*Empowering Healthcare with Secure, Fast, and Clinician-Friendly Analytics*
