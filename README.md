# `pubchem.rs` [![Star me](https://img.shields.io/github/stars/althonos/pubchem.rs.svg?style=social&label=Star&maxAge=3600)](https://github.com/althonos/pubchem.rs/stargazers)

*Rust data structures and client for the [PubChem](https://pubchem.ncbi.nlm.nih.gov/) [REST API](https://pubchemdocs.ncbi.nlm.nih.gov/pug-rest).*

[![Actions](https://img.shields.io/github/workflow/status/althonos/pubchem.rs/Test?style=flat-square&maxAge=600)](https://github.com/althonos/pubchem.rs/actions)
[![Codecov](https://img.shields.io/codecov/c/gh/althonos/pubchem.rs/master.svg?style=flat-square&maxAge=600)](https://codecov.io/gh/althonos/pubchem.rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/pubchem.rs)
[![Crate](https://img.shields.io/crates/v/pubchem.svg?maxAge=600&style=flat-square)](https://crates.io/crates/pubchem)
[![Documentation](https://img.shields.io/badge/docs.rs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/pubchem)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/pubchem.rs/blob/master/CHANGELOG.md)
[![GitHub issues](https://img.shields.io/github/issues/althonos/pubchem.rs.svg?style=flat-square&maxAge=600)](https://github.com/althonos/pubchem.rs/issues)


## 🔌 Usage

### 💊 Compound

Create a `Compound` to query the PubChem API for a single compound. It can
be constructed from a compound ID, from a compound name, from an InChI or
InChIKey, or from a SMILES string:

```rust
extern crate pubchem;

let alanine = pubchem::Compound::new(5950);
let aspirin = pubchem::Compound::with_name("aspirin");
let acetone = pubchem::Compound::with_inchi("InChI=1S/C3H6O/c1-3(2)4/h1-2H3");
let lysine  = pubchem::Compound::with_inchikey("KDXKERNSBIXSRK-YFKPBYRVSA-N");
let benzene = pubchem::Compound::with_smiles("C1=CC=CC=C1");
```

Use the methods to query the REST API with [`ureq`](https://crates.io/crates/ureq).
Dedicated methods exist for common single properties:

```rust
let alanine = pubchem::Compound::new(5950);

alanine.title().unwrap(); // "Alanine"
alanine.molecular_formula().unwrap(); // "C3H7NO2"
alanine.canonical_smiles().unwrap(); // "CC(C(=O)O)N"
alanine.isomeric_smiles().unwrap();  // "C[C@@H](C(=O)O)N"
```

Each method will perform a single query to the PubChem API, which is inefficient
if you wish to retrieve several properties at once. In that case, use the
`properties` method and select which properties you want to retrieve
in a single query:

```rust
use pubchem::CompoundProperty::*;

let properties = pubchem::Compound::new(5950)
    .properties(&[Title, MolecularFormula, CanonicalSMILES])
    .unwrap();

properties.molecular_formula; // Some("C3H7NO2")
properties.canonical_smiles; // Some("CC(C(=O)O)N")
properties.isomeric_smiles; // Some("C[C@@H](C(=O)O)N")
```

To retrieve metadata from multiple compounds at once, use the `Compounds`
struct and use the `properties` method to pack everything into a single
query:

```rust
use pubchem::CompoundProperty::*;

// retrieve metadata from the three aromatic L-amino acids at once
for prop in pubchem::Compounds::new([6140, 145742, 6305])
    .properties(&[Title, IUPACName, ExactMass])
    .unwrap()
{
    println!(
        "[{cid}] {title} {iupac} {mass}g/mol",
        cid = prop.cid,
        title = prop.title.unwrap(),
        iupac = prop.iupac_name.unwrap(),
        mass = prop.exact_mass.unwrap(),
    );
}
```

## 💭 Feedback

### ⚠️ Issue Tracker

Found a bug ? Have an enhancement request ? Head over to the [GitHub issue
tracker](https://github.com/althonos/pubchem.rs/issues) if you need to report
or ask something. If you are filing in on a bug, please include as much
information as you can about the issue, and try to recreate the same bug
in a simple, easily reproducible situation.

<!-- ### 🏗️ Contributing

Contributions are more than welcome! See
[`CONTRIBUTING.md`](https://github.com/althonos/pubchem.rs/blob/main/CONTRIBUTING.md)
for more details. -->

## 📋 Changelog

This project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html)
and provides a [changelog](https://github.com/althonos/pubchem.rs/blob/master/CHANGELOG.md)
in the [Keep a Changelog](http://keepachangelog.com/en/1.0.0/) format.

## 🔍 See Also

If you're a bioinformatician and a Rustacean, you may be interested in these
other libraries:

- [`uniprot.rs`](https://github.com/althonos/uniprot.rs): Rust data structures
  for the UniProtKB databases.
- [`obofoundry.rs`](https://github.com/althonos/obofoundry.rs): Rust data
  structures for the OBO Foundry.
- [`fastobo`](https://github.com/fastobo/fastobo): Rust parser and abstract
  syntax tree for Open Biomedical Ontologies.
- [`proteinogenic`](https://github.com/althonos/proteinogenic): Chemical 
  structure generation for protein sequences as 
  [SMILES](https://en.wikipedia.org/wiki/Simplified_molecular-input_line-entry_system) strings.

## 📜 License

This library is provided under the open-source
[MIT license](https://choosealicense.com/licenses/mit/).

*This project is in no way not affiliated, sponsored, or otherwise endorsed
by the PubChem developers. It was developed
by [Martin Larralde](https://github.com/althonos/) during his PhD project
at the [European Molecular Biology Laboratory](https://www.embl.de/) in
the [Zeller team](https://github.com/zellerlab). PubChem® is a registered 
trademark of the US National Library of Medicine.*

