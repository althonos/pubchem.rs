#[macro_use]
mod macros;

use std::io::BufRead;

use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Error as XmlError;
use quick_xml::Reader;

use crate::error::Error;

/// A trait for types that can be parsed from an XML element.
pub trait FromXml: Sized {
    fn from_xml<B: BufRead>(
        event: &BytesStart,
        reader: &mut Reader<B>,
        buffer: &mut Vec<u8>,
    ) -> Result<Self, Error>;
}

/// A trait for types that can be returned by the REST API.
pub trait FromApiResponse: FromXml {
    fn from_api_response(response: ureq::Response) -> Result<Self, Error>;
}

impl<T: FromXml> FromApiResponse for T {
    fn from_api_response(response: ureq::Response) -> Result<Self, Error> {
        let reader = response.into_reader();
        let mut xml = Reader::from_reader(std::io::BufReader::new(reader));
        let mut buffer = Vec::new();

        let event = loop {
            buffer.clear();
            match xml.read_event(&mut buffer) {
                Err(e) => return Err(Error::from(e)),
                Ok(Event::Start(e)) => break e.to_owned(),
                Ok(Event::Eof) => {
                    let e = String::from("xml");
                    return Err(Error::from(XmlError::UnexpectedEof(e)));
                }
                _ => (),
            }
        };

        T::from_xml(&event, &mut xml, &mut buffer)
    }
}
