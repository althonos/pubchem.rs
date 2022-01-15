//! [![Star me](https://img.shields.io/github/stars/althonos/pubchem.rs.svg?style=social&label=Star&maxAge=3600)](https://github.com/althonos/pubchem.rs/stargazers)
//!
//! *Rust data structures and parser for the [UniprotKB database(s)].*
//!
extern crate quick_xml;
extern crate thiserror;
extern crate ureq;
extern crate form_urlencoded;

#[macro_use]
mod parser;
#[macro_use]
mod utils;

pub mod error;
pub mod model;

use std::borrow::Cow;

use self::error::Error;
use self::model::rest;
use self::parser::FromApiResponse;

named_enum! {
    #[derive(Debug, PartialEq, Eq)]
    /// A single property that can be retrieved from a compound.
    pub enum CompoundProperty {
        /// Molecular formula.
        MolecularFormula,
        /// The molecular weight is the sum of all atomic weights of the constituent atoms in a compound, measured in g/mol. In the absence of explicit isotope labelling, averaged natural abundance is assumed. If an atom bears an explicit isotope label, 100% isotopic purity is assumed at this location.
        MolecularWeight,
        /// Canonical SMILES (Simplified Molecular Input Line Entry System) string.  It is a unique SMILES string of a compound, generated by a “canonicalization” algorithm.
        CanonicalSMILES,
        /// Isomeric SMILES string.  It is a SMILES string with stereochemical and isotopic specifications.
        IsomericSMILES,
        /// Standard IUPAC International Chemical Identifier (InChI).  It does not allow for user selectable options in dealing with the stereochemistry and tautomer layers of the InChI string.
        InChI,
        /// Hashed version of the full standard InChI, consisting of 27 characters.
        InChIKey,
        /// Chemical name systematically determined according to the IUPAC nomenclatures.
        IUPACName,
        /// The title used for the compound summary page.
        Title,
        /// Computationally generated octanol-water partition coefficient or distribution coefficient. XLogP is used as a measure of hydrophilicity or hydrophobicity of a molecule.
        XLogP,
        /// The mass of the most likely isotopic composition for a single molecule, corresponding to the most intense ion/molecule peak in a mass spectrum.
        ExactMass,
        /// The mass of a molecule, calculated using the mass of the most abundant isotope of each element.
        MonoisotopicMass,
        /// Topological polar surface area, computed by the algorithm described in the paper by Ertl et al.
        TPSA,
        /// The molecular complexity rating of a compound, computed using the Bertz/Hendrickson/Ihlenfeldt formula.
        Complexity,
        /// The total (or net) charge of a molecule.
        Charge,
        /// Number of hydrogen-bond donors in the structure.
        HBondDonorCount,
        /// Number of hydrogen-bond acceptors in the structure.
        HBondAcceptorCount,
        /// Number of rotatable bonds.
        RotatableBondCount,
        /// Number of non-hydrogen atoms.
        HeavyAtomCount,
        /// Number of atoms with enriched isotope(s)
        IsotopeAtomCount,
        /// Total number of atoms with tetrahedral (sp3) stereo [e.g., (R)- or (S)-configuration]
        AtomStereoCount,
        /// Number of atoms with defined tetrahedral (sp3) stereo.
        DefinedAtomStereoCount,
        /// Number of atoms with undefined tetrahedral (sp3) stereo.
        UndefinedAtomStereoCount,
        /// Total number of bonds with planar (sp2) stereo [e.g., (E)- or (Z)-configuration].
        BondStereoCount,
        /// Number of atoms with defined planar (sp2) stereo.
        DefinedBondStereoCount,
        /// Number of atoms with undefined planar (sp2) stereo.
        UndefinedBondStereoCount,
        /// Number of covalently bound units.
        CovalentUnitCount,
        /// Analytic volume of the first diverse conformer (default conformer) for a compound.
        Volume3D,
        /// The x component of the quadrupole moment (Qx) of the first diverse conformer (default conformer) for a compound.
        XStericQuadrupole3D,
        /// The y component of the quadrupole moment (Qy) of the first diverse conformer (default conformer) for a compound.
        YStericQuadrupole3D,
        /// The z component of the quadrupole moment (Qz) of the first diverse conformer (default conformer) for a compound.
        ZStericQuadrupole3D,
        /// Total number of 3D features (the sum of FeatureAcceptorCount3D, FeatureDonorCount3D, FeatureAnionCount3D, FeatureCationCount3D, FeatureRingCount3D and FeatureHydrophobeCount3D)
        FeatureCount3D,
        /// Number of hydrogen-bond acceptors of a conformer.
        FeatureAcceptorCount3D,
        /// Number of hydrogen-bond donors of a conformer.
        FeatureDonorCount3D,
        /// Number of anionic centers (at pH 7) of a conformer.
        FeatureAnionCount3D,
        /// Number of cationic centers (at pH 7) of a conformer.
        FeatureCationCount3D,
        /// Number of rings of a conformer.
        FeatureRingCount3D,
        /// Number of hydrophobes of a conformer.
        FeatureHydrophobeCount3D,
        /// Conformer sampling RMSD in Å.
        ConformerModelRMSD3D,
        /// Total number of 3D features (the sum of FeatureAcceptorCount3D, FeatureDonorCount3D, FeatureAnionCount3D, FeatureCationCount3D, FeatureRingCount3D and FeatureHydrophobeCount3D)
        EffectiveRotorCount3D,
        /// The number of conformers in the conformer model for a compound.
        ConformerCount3D,
        /// Base64-encoded PubChem Substructure Fingerprint of a molecule.
        Fingerprint2D,
    }
}

#[derive(Debug)]
/// A client to retrieve information about a single PubChem compound.
pub struct Compound {
    namespace: Cow<'static, str>,
    identifier: Cow<'static, str>,
}

impl Compound {
    pub fn new(id: u32) -> Self {
        Self {
            namespace: Cow::Borrowed("cid"),
            identifier: Cow::Owned(id.to_string()),
        }
    }

    pub fn with_name(name: &str) -> Self {
        Self {
            namespace: Cow::Borrowed("name"),
            identifier: Cow::Owned(name.to_string()),
        }
    }

    pub fn with_smiles(smiles: &str) -> Self {
        Self {
            namespace: Cow::Borrowed("smiles"),
            identifier: Cow::Owned(smiles.to_string()),
        }
    }

    pub fn with_inchi(inchi: &str) -> Self {
        Self {
            namespace: Cow::Borrowed("inchi"),
            identifier: Cow::Owned(inchi.to_string())
        }
    }

    pub fn with_inchikey(inchikey: &str) -> Self {
        Self {
            namespace: Cow::Borrowed("inchikey"),
            identifier: Cow::Owned(inchikey.to_string())
        }
    }

    // pub fn with_sdf(sdf: &str) -> Self {
    //     Self {
    //         namespace: Cow::Borrowed("sdf"),
    //         identifiers: Cow::Owned(sdf.to_string())
    //     }
    // }

    /// Request the REST API for the given operation.
    ///
    /// The response is checked to see if the HTTP client or the API errored,
    /// otherwise the raw response is returned so that it can be parsed by
    /// the appropriate method.
    ///
    fn request(&self, operation: &str) -> Result<ureq::Response, Error> {
        let url = format!(
            "https://pubchem.ncbi.nlm.nih.gov/rest/pug/{dom}/{ns}/{op}/XML",
            dom = "compound",
            ns = &self.namespace,
            op = operation
        );
        let form_data = form_urlencoded::Serializer::new(String::new())
            .append_pair(&self.namespace, &self.identifier)
            .finish();
        match ureq::post(&url)
            .set("Accept", "application/xml")
            .set("Content-Type", "application/x-www-form-urlencoded")
            .send_string(&form_data)
        {
            Err(ureq::Error::Status(400 | 404 | 405 | 500 | 501 | 503 | 504, response)) => {
                let fault = rest::Fault::from_api_response(response)?;
                Err(Error::Api(fault.into()))
            }
            Err(e) => Err(Error::Request(e)),
            Ok(response) => Ok(response),
        }
    }

    /// Retrieve several properties at once for the compound.
    pub fn properties<'p, P>(&self, properties: P) -> Result<rest::Properties, Error>
    where
        P: IntoIterator<Item = &'p CompoundProperty>,
    {
        let path = {
            let mut path = String::from("/property/");
            let mut prop_iter = properties.into_iter().peekable();
            while let Some(property) = prop_iter.next() {
                path.push_str(property.name());
                if prop_iter.peek().is_some() {
                    path.push(',');
                }
            }
            path
        };
        self.request(&path)
            .and_then(|response| rest::PropertyTable::from_api_response(response))
            .map(|mut table| table.properties.pop().unwrap())
    }

    /// Retrieve the main PubChem designation for the compound.
    pub fn title(&self) -> Result<Option<String>, Error> {
        let properties = self.properties(&[CompoundProperty::Title])?;
        Ok(properties.title)
    }

    /// Retrieve the molecular formula of the compound.
    ///
    /// # Example
    /// ```
    /// let compound = pubchem::Compound::with_name("aspirin");
    /// assert_eq!( compound.molecular_formula().unwrap(), Some(String::from("C9H8O4")) );
    /// ```
    pub fn molecular_formula(&self) -> Result<Option<String>, Error> {
        let properties = self.properties(&[CompoundProperty::MolecularFormula])?;
        Ok(properties.molecular_formula)
    }

    // /// Retrieve the entire PubChem record for the compound.
    // pub fn record(&self) {
    //     unimplemented!()
    // }
    //
    // pub fn synonyms(&self) {
    //     unimplemented!()
    // }

    /// Retrieve the Compound IDs designating the compound.
    pub fn cids(&self) -> Result<Vec<i32>, Error> {
        self.request("cids")
            .and_then(|response| rest::IdentifierList::from_api_response(response))
            .map(|list| list.cids)
    }

    /// Retrieve the Substance IDs associated with the compound.
    pub fn sids(&self) -> Result<Vec<i32>, Error> {
        self.request("sids")
            .and_then(|response| rest::InformationList::from_api_response(response))
            .map(|mut list| list.informations.pop().unwrap().sids)
    }

    /// Retrieve the Assay IDs associated with the compound.
    pub fn aids(&self) -> Result<Vec<i32>, Error> {
        self.request("aids")
            .and_then(|response| rest::InformationList::from_api_response(response))
            .map(|mut list| list.informations.pop().unwrap().aids)
    }

    // pub fn assay_summary(&self) {
    //
    // }
    //
    // pub fn classification(&self) {
    //
    // }
    //
    // pub fn xref(&self) {
    //
    // }
    //
    // pub fn description(&self) {
    //
    // }
    //
    // pub fn conformers(&self) {
    //
    // }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::error::ApiError;

    #[test]
    fn compound_new() {
        let compound = Compound::new(2244);
        let properties = compound
            .properties(&[CompoundProperty::Title])
            .expect("compound property retrieval should not fail");
        assert_eq!(properties.title.as_ref().unwrap(), "Aspirin");
    }

    #[test]
    fn compound_with_name() {
        let compound = Compound::with_name("lyciumin A");
        let properties = compound
            .properties(&[CompoundProperty::Title])
            .expect("compound property retrieval should not fail");
        assert_eq!(properties.cid, 14430290);
    }

    #[test]
    fn compound_with_smiles() {
        let compound = Compound::with_smiles("CC(=O)OC1=CC=CC=C1C(=O)O");
        let properties = compound
            .properties(&[CompoundProperty::Title])
            .expect("compound property retrieval should not fail");
        assert_eq!(properties.cid, 2244);
    }

    #[test]
    fn compound_with_inchikey() {
        let compound = Compound::with_inchikey("AUJXLBOHYWTPFV-UHFFFAOYSA-N");
        assert_eq!(compound.title().unwrap(), Some(String::from("Echinomycin")));
    }

    #[test]
    fn compound_with_inchi() {
        let compound = Compound::with_inchi("InChI=1S/C3H6O/c1-3(2)4/h1-2H3");
        assert_eq!(compound.title().unwrap(), Some(String::from("Acetone")));
    }

    #[test]
    fn compound_cids() {
        let compound = Compound::new(2244);
        assert_eq!(compound.cids().unwrap(), vec![2244])
    }

    #[test]
    #[rustfmt::skip]
    fn compound_sids() {
        let compound = Compound::new(10444160);
        assert_eq!(
            compound.sids().unwrap(),
            vec![
                 15464793,  40571804,  50320706, 103261147,
                319343201, 383830942, 386266192, 459034771
            ]
        )
    }

    #[test]
    fn compound_name_not_found() {
        let compound = Compound::with_name("none");
        match compound.cids() {
            Err(Error::Api(ApiError::NotFound(_))) => (),
            Err(e) => panic!("unexpected error {}", e),
            Ok(_) => panic!("unexpected success"),
        }
    }
}
