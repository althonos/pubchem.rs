//! Raw data types returned by the Power User Gateway REST API.

use std::io::BufRead;

use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::error::Error;
use crate::parser::FromXml;

#[derive(Default, Debug, PartialEq)]
pub struct Fault {
    pub code: String,
    pub message: String,
    pub details: Vec<String>,
}

impl FromXml for Fault {
    fn from_xml<B: BufRead>(
        event: &BytesStart,
        reader: &mut Reader<B>,
        buffer: &mut Vec<u8>,
    ) -> Result<Self, Error> {
        debug_assert_eq!(event.local_name(), b"Fault");

        let mut fault = Fault::default();
        parse_inner! {event, reader, buffer,
            b"Code" => {
                fault.code = reader.read_text(b"Code", buffer)?;
            },
            b"Message" => {
                fault.message = reader.read_text(b"Message", buffer)?;
            },
            b"Details" => {
                fault.details.push(reader.read_text(b"Details", buffer)?);
            }
        }
        Ok(fault)
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Waiting {
    pub list_key: String,
    pub message: Option<String>,
}

#[derive(Default, Debug, PartialEq)]
pub struct PropertyTable {
    pub properties: Vec<Properties>,
}

impl FromXml for PropertyTable {
    fn from_xml<B: BufRead>(
        event: &BytesStart,
        reader: &mut Reader<B>,
        buffer: &mut Vec<u8>,
    ) -> Result<Self, Error> {
        debug_assert_eq!(event.local_name(), b"PropertyTable");

        let mut table = PropertyTable::default();
        parse_inner! {event, reader, buffer,
            e @ b"Properties" => {
                table.properties.push(Properties::from_xml(&e, reader, buffer)?);
            }
        };

        Ok(table)
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Properties {
    pub cid: i32,
    pub molecular_formula: Option<String>,
    pub molecular_weight: Option<String>,
    pub canonical_smiles: Option<String>,
    pub isomeric_smiles: Option<String>,
    pub inchi: Option<String>,
    pub inchi_key: Option<String>,
    pub iupac_name: Option<String>,
    pub xlogp: Option<f64>,
    pub exact_mass: Option<String>,
    pub monoisotopic_mass: Option<String>,
    pub tpsa: Option<f64>,
    pub complexity: Option<i32>,
    pub charge: Option<i32>,
    pub hbond_donor_count: Option<i32>,
    pub hbond_acceptor_count: Option<i32>,
    pub rotatable_bond_count: Option<i32>,
    pub heavy_atom_count: Option<i32>,
    pub isotope_atom_count: Option<i32>,
    pub atom_stereo_count: Option<i32>,
    pub defined_atom_stereo_count: Option<i32>,
    pub undefined_atom_stereo_count: Option<i32>,
    pub bond_stereo_count: Option<i32>,
    pub defined_bond_stereo_count: Option<i32>,
    pub undefined_bond_stereo_count: Option<i32>,
    pub covalent_unit_count: Option<i32>,
    pub volume_3d: Option<f64>,
    pub x_steric_quadrupole_3d: Option<f64>,
    pub y_steric_quadrupole_3d: Option<f64>,
    pub z_steric_quadrupole_3d: Option<f64>,
    pub feature_count_3d: Option<i32>,
    pub feature_acceptor_count_3d: Option<i32>,
    pub feature_donor_count_3d: Option<i32>,
    pub feature_anion_count_3d: Option<i32>,
    pub feature_cation_count_3d: Option<i32>,
    pub feature_ring_count_3d: Option<i32>,
    pub feature_hydrophobe_count_3d: Option<i32>,
    pub conformer_model_rmsd_3d: Option<f64>,
    pub effective_rotor_count_3d: Option<f64>,
    pub conformer_count_3d: Option<i32>,
    pub fingerprint_2d: Option<String>,
    pub title: Option<String>,
}

impl FromXml for Properties {
    fn from_xml<B: BufRead>(
        event: &BytesStart,
        reader: &mut Reader<B>,
        buffer: &mut Vec<u8>,
    ) -> Result<Self, Error> {
        debug_assert_eq!(event.local_name(), b"Properties");

        macro_rules! to_field {
            ($reader:ident, $buffer:ident, $e:ident, $p:ident . $field:ident) => {{
                $p.$field = Some($reader.read_text($e.name(), $buffer)?);
            }};
            ($reader:ident, $buffer:ident, $e:ident, $p:ident . $field:ident ?) => {{
                $p.$field = Some($reader.read_text($e.name(), $buffer)?.parse()?);
            }};
        }

        let mut p = Properties::default();
        parse_inner! {event, reader, buffer,
            b"CID" => { p.cid = reader.read_text(b"CID", buffer)?.parse()?; },
            e @ b"MolecularFormula" => to_field!(reader, buffer, e, p.molecular_formula),
            e @ b"MolecularWeight" => to_field!(reader, buffer, e, p.molecular_weight),
            e @ b"CanonicalSMILES" => to_field!(reader, buffer, e, p.canonical_smiles),
            e @ b"IsomericSMILES" => to_field!(reader, buffer, e, p.isomeric_smiles),
            e @ b"InChI" => to_field!(reader, buffer, e, p.inchi),
            e @ b"InChIKey" => to_field!(reader, buffer, e, p.inchi_key),
            e @ b"IUPACName" => to_field!(reader, buffer, e, p.iupac_name),
            e @ b"XLogP" => to_field!(reader, buffer, e, p.xlogp ?),
            e @ b"ExactMass" => to_field!(reader, buffer, e, p.exact_mass),
            e @ b"MonoisotopicMass" => to_field!(reader, buffer, e, p.monoisotopic_mass),
            e @ b"TPSA" => to_field!(reader, buffer, e, p.tpsa ?),
            e @ b"Complexity" => to_field!(reader, buffer, e, p.complexity ?),
            e @ b"Charge" => to_field!(reader, buffer, e, p.charge ?),
            e @ b"HBondDonorCount" => to_field!(reader, buffer, e, p.hbond_donor_count ?),
            e @ b"HBondAcceptorCount" => to_field!(reader, buffer, e, p.hbond_acceptor_count ?),
            e @ b"RotatableBondCount" => to_field!(reader, buffer, e, p.rotatable_bond_count ?),
            e @ b"HeavyAtomCount" => to_field!(reader, buffer, e, p.heavy_atom_count ?),
            e @ b"IsotopeAtomCount" => to_field!(reader, buffer, e, p.isotope_atom_count ?),
            e @ b"AtomStereoCount" => to_field!(reader, buffer, e, p.atom_stereo_count ?),
            e @ b"DefinedAtomStereoCount" => to_field!(reader, buffer, e, p.defined_atom_stereo_count ?),
            e @ b"UndefinedAtomStereoCount" => to_field!(reader, buffer, e, p.undefined_atom_stereo_count ?),
            e @ b"BondStereoCount" => to_field!(reader, buffer, e, p.bond_stereo_count ?),
            e @ b"DefinedBondStereoCount" => to_field!(reader, buffer, e, p.defined_bond_stereo_count ?),
            e @ b"UndefinedBondStereoCount" => to_field!(reader, buffer, e, p.undefined_bond_stereo_count ?),
            e @ b"CovalentUnitCount" => to_field!(reader, buffer, e, p.covalent_unit_count ?),
            e @ b"Volume3D" => to_field!(reader, buffer, e, p.volume_3d ?),
            e @ b"XStericQuadrupole3D" => to_field!(reader, buffer, e, p.x_steric_quadrupole_3d ?),
            e @ b"YStericQuadrupole3D" => to_field!(reader, buffer, e, p.y_steric_quadrupole_3d ?),
            e @ b"ZStericQuadrupole3D" => to_field!(reader, buffer, e, p.z_steric_quadrupole_3d ?),
            e @ b"FeatureCount3D" => to_field!(reader, buffer, e, p.feature_count_3d ?),
            e @ b"FeatureAcceptorCount3D" => to_field!(reader, buffer, e, p.feature_acceptor_count_3d ?),
            e @ b"FeatureDonorCount3D" => to_field!(reader, buffer, e, p.feature_donor_count_3d ?),
            e @ b"FeatureAnionCount3D" => to_field!(reader, buffer, e, p.feature_anion_count_3d ?),
            e @ b"FeatureCationCount3D" => to_field!(reader, buffer, e, p.feature_cation_count_3d ?),
            e @ b"FeatureRingCount3D" => to_field!(reader, buffer, e, p.feature_ring_count_3d ?),
            e @ b"FeatureHydrophobeCount3D" => to_field!(reader, buffer, e, p.feature_hydrophobe_count_3d ?),
            e @ b"ConformerModelRMSD3D" => to_field!(reader, buffer, e, p.conformer_model_rmsd_3d ?),
            e @ b"EffectiveRotorCount3D" => to_field!(reader, buffer, e, p.effective_rotor_count_3d ?),
            e @ b"ConformerCount3D" => to_field!(reader, buffer, e, p.conformer_count_3d ?),
            e @ b"Fingerprint2D" => to_field!(reader, buffer, e, p.fingerprint_2d),
            e @ b"Title" => to_field!(reader, buffer, e, p.title),
        }

        Ok(p)
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct InformationList {
    pub informations: Vec<Information>,
    pub source_names: Vec<String>,
    pub annotations: Vec<Annotation>,
}

impl FromXml for InformationList {
    fn from_xml<B: BufRead>(
        event: &BytesStart,
        reader: &mut Reader<B>,
        buffer: &mut Vec<u8>,
    ) -> Result<Self, Error> {
        debug_assert_eq!(event.local_name(), b"InformationList");

        let mut list = InformationList::default();
        parse_inner! {event, reader, buffer,
            e @ b"SourceName" => {
                list.source_names.push(reader.read_text(e.name(), buffer)?);
            },
            e @ b"Information" => {
                list.informations.push(Information::from_xml(&e, reader, buffer)?);
            },
            e @ b"Annotation" => {
                list.annotations.push(Annotation::from_xml(&e, reader, buffer)?);
            }
        };

        Ok(list)
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Information {
    pub id: i32,
    pub synonyms: Vec<String>,
    pub cids: Vec<i32>,
    pub sids: Vec<i32>,
    pub aids: Vec<i32>,
    pub gis: Vec<i32>,
    pub gene_ids: Vec<i32>,
    pub deposition_date: Option<DateTime>,
    pub modification_date: Option<DateTime>,
    pub creation_date: Option<DateTime>,
    pub hold_date: Option<DateTime>,
    pub registry_ids: Vec<String>,
    pub rns: Vec<String>,
    pub pubmed_ids: Vec<i32>,
    pub mmdb_ids: Vec<i32>,
    pub db_urls: Vec<String>,
    pub sb_urls: Vec<String>,
    pub protein_gis: Vec<i32>,
    pub nucleotide_gis: Vec<i32>,
    pub taxonomy_ids: Vec<i32>,
    pub mim_ids: Vec<i32>,
    pub probe_ids: Vec<i32>,
    pub patent_ids: Vec<i32>,
    pub protein_names: Vec<String>,
    pub gene_symbols: Vec<String>,
    pub source_names: Vec<String>,
    pub source_categories: Vec<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub description_source_name: Option<String>,
    pub description_url: Option<String>,
    pub conformer_ids: Vec<String>,
    pub protein_accessions: Vec<String>,
}

impl FromXml for Information {
    fn from_xml<B: BufRead>(
        event: &BytesStart,
        reader: &mut Reader<B>,
        buffer: &mut Vec<u8>,
    ) -> Result<Self, Error> {
        debug_assert_eq!(event.local_name(), b"Information");

        macro_rules! push_field {
            ($reader:ident, $buffer:ident, $e:ident, $p:ident . $field:ident) => {{
                $p.$field.push($reader.read_text($e.name(), $buffer)?);
            }};
            ($reader:ident, $buffer:ident, $e:ident, $p:ident . $field:ident ?) => {{
                $p.$field
                    .push($reader.read_text($e.name(), $buffer)?.parse()?);
            }};
        }

        macro_rules! set_option {
            ($reader:ident, $buffer:ident, $e:ident, $p:ident . $field:ident) => {{
                $p.$field = Some(reader.read_text($e.name(), $buffer)?);
            }};
            ($reader:ident, $buffer:ident, $e:ident, $p:ident . $field:ident XML) => {{
                $p.$field = Some(FromXml::from_xml(&$e, $reader, $buffer)?);
            }};
        }

        let mut i = Information::default();
        parse_inner! {event, reader, buffer,
            e @ b"ID" => {
                i.id = reader.read_text(e.name(), buffer)?.parse()?;
            },
            e @ b"Synonym" => push_field!(reader, buffer, e, i.synonyms),
            e @ b"CID" => push_field!(reader, buffer, e, i.cids ?),
            e @ b"SID" => push_field!(reader, buffer, e, i.sids ?),
            e @ b"AID" => push_field!(reader, buffer, e, i.aids ?),
            e @ b"GI" => push_field!(reader, buffer, e, i.gis ?),
            e @ b"GeneID" => push_field!(reader, buffer, e, i.gene_ids ?),
            e @ b"DepositionDate" => set_option!(reader, buffer, e, i.deposition_date XML),
            e @ b"ModificationDate" => set_option!(reader, buffer, e, i.modification_date XML),
            e @ b"CreationDate" => set_option!(reader, buffer, e, i.creation_date XML),
            e @ b"HoldDate" => set_option!(reader, buffer, e, i.hold_date XML),
            e @ b"RegistryID" => push_field!(reader, buffer, e, i.registry_ids),
            e @ b"RN" => push_field!(reader, buffer, e, i.rns),
            e @ b"PubMedId" => push_field!(reader, buffer, e, i.pubmed_ids ?),
            e @ b"MMDBID" => push_field!(reader, buffer, e, i.mmdb_ids ?),
            e @ b"DBURL" => push_field!(reader, buffer, e, i.db_urls),
            e @ b"SBURL" => push_field!(reader, buffer, e, i.sb_urls),
            e @ b"ProteinGI" => push_field!(reader, buffer, e, i.protein_gis ?),
            e @ b"NucleotideGI" => push_field!(reader, buffer, e, i.nucleotide_gis ?),
            e @ b"TaxonomyID" => push_field!(reader, buffer, e, i.taxonomy_ids ?),
            e @ b"MIMID" => push_field!(reader, buffer, e, i.mim_ids ?),
            e @ b"ProbeID" => push_field!(reader, buffer, e, i.probe_ids ?),
            e @ b"PatentID" => push_field!(reader, buffer, e, i.patent_ids ?),
            e @ b"ProteinName" => push_field!(reader, buffer, e, i.protein_names),
            e @ b"GeneSymbol" => push_field!(reader, buffer, e, i.gene_symbols),
            e @ b"SourceName" => push_field!(reader, buffer, e, i.source_names),
            e @ b"SourceCategory" => push_field!(reader, buffer, e, i.source_categories),
            e @ b"Title" => set_option!(reader, buffer, e, i.title),
            e @ b"Description" => set_option!(reader, buffer, e, i.description),
            e @ b"DescriptionSourceName" => set_option!(reader, buffer, e, i.description_source_name),
            e @ b"DescriptionURL" => set_option!(reader, buffer, e, i.description_url),
            e @ b"ConformerID" => push_field!(reader, buffer, e, i.conformer_ids),
            e @ b"ProteinAccession" => push_field!(reader, buffer, e, i.protein_accessions),
        }

        Ok(i)
    }
}

#[derive(Debug, PartialEq)]
pub struct Annotation {
    heading: String,
    ty: String,
}

impl FromXml for Annotation {
    fn from_xml<B: BufRead>(
        event: &BytesStart,
        reader: &mut Reader<B>,
        buffer: &mut Vec<u8>,
    ) -> Result<Self, Error> {
        debug_assert_eq!(event.local_name(), b"Annotation");

        unimplemented!()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct DateTime {
    year: Option<i32>,
    month: Option<i32>,
    day: Option<i32>,
    hour: Option<i32>,
    minute: Option<i32>,
    second: Option<i32>,
}

impl FromXml for DateTime {
    fn from_xml<B: BufRead>(
        event: &BytesStart,
        reader: &mut Reader<B>,
        buffer: &mut Vec<u8>,
    ) -> Result<Self, Error> {
        debug_assert_eq!(event.local_name(), b"DateTime");

        macro_rules! to_field {
            ($reader:ident, $buffer:ident, $e:ident, $dt:ident . $field:ident) => {{
                $dt.$field = Some($reader.read_text($e.name(), $buffer)?.parse()?);
            }};
        }

        let mut dt = DateTime::default();
        parse_inner! {event, reader, buffer,
            e @ b"Year" => to_field!(reader, buffer, e, dt.year),
            e @ b"Month" => to_field!(reader, buffer, e, dt.month),
            e @ b"Day" => to_field!(reader, buffer, e, dt.day),
            e @ b"Hour" => to_field!(reader, buffer, e, dt.hour),
            e @ b"Minute" => to_field!(reader, buffer, e, dt.minute),
            e @ b"Second" => to_field!(reader, buffer, e, dt.second),
        };

        Ok(dt)
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct IdentifierList {
    pub cids: Vec<i32>,
    pub sids: Vec<i32>,
    pub aids: Vec<i32>,
    pub list_key: Option<String>,
    pub size: Option<i32>,
    pub entrez_db: Option<String>,
    pub entrez_web_env: Option<String>,
    pub entrez_query_key: Option<i32>,
    pub entrez_url: Option<String>,
    pub cache_key: Option<String>,
}

impl FromXml for IdentifierList {
    fn from_xml<B: BufRead>(
        event: &BytesStart,
        reader: &mut Reader<B>,
        buffer: &mut Vec<u8>,
    ) -> Result<Self, Error> {
        debug_assert_eq!(event.local_name(), b"IdentifierList");

        let mut list = IdentifierList::default();
        parse_inner! {event, reader, buffer,
            b"CID" => {
                list.cids.push(reader.read_text(b"CID", buffer)?.parse()?);
            },
            b"SID" => {
                list.sids.push(reader.read_text(b"SID", buffer)?.parse()?);
            },
            b"AID" => {
                list.sids.push(reader.read_text(b"AID", buffer)?.parse()?);
            },
            b"ListKey" => {
                list.list_key = Some(reader.read_text(b"ListKey", buffer)?);
            },
            b"Size" => {
                list.size = Some(reader.read_text(b"Size", buffer)?.parse()?);
            },
            b"EntrezDB" => {
                list.entrez_db = Some(reader.read_text(b"EntrezDB", buffer)?);
            },
            b"EntrezWebEnv" => {
                list.entrez_web_env = Some(reader.read_text(b"EntrezWebEnv", buffer)?);
            },
            b"EntrezQueryKey" => {
                list.entrez_query_key = Some(reader.read_text(b"EntrezQueryKey", buffer)?.parse()?);
            },
            b"EntrezURL" => {
                list.entrez_url = Some(reader.read_text(b"EntrezURL", buffer)?);
            },
            b"CacheKey" => {
                list.cache_key = Some(reader.read_text(b"CacheKey", buffer)?);
            },
        };

        Ok(list)
    }
}

// #[derive(Debug, PartialEq, Eq)]
// pub struct Table {
//     columns: Vec<String>,
//     rows: Vec<String>
// }
