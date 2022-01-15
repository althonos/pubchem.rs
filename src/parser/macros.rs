#[allow(unused_macros)]
macro_rules! parse_inner {
    ($event:expr, $reader:expr, $buffer:expr, $($rest:tt)*) => ({
        #[allow(unused_imports)]
        loop {
            use $crate::quick_xml::events::BytesEnd;
            use $crate::quick_xml::events::BytesStart;
            use $crate::quick_xml::events::Event;
            use $crate::quick_xml::Error as XmlError;

            $buffer.clear();
            match $reader.read_event($buffer) {
                Ok(Event::Start(ref x)) => {
                    parse_inner_impl!(x, x.local_name(), $($rest)*);
                    $reader.read_to_end(x.local_name(), &mut Vec::new())?;
                    unimplemented!(
                        "`{}` in `{}`",
                        String::from_utf8_lossy(x.local_name()),
                        String::from_utf8_lossy($event.local_name())
                    );
                }
                Err(e) => {
                    return Err(Error::from(e));
                }
                Ok(Event::Eof) => {
                    let e = String::from_utf8_lossy($event.local_name()).to_string();
                    return Err(Error::from(XmlError::UnexpectedEof(e)));
                }
                Ok(Event::End(ref e)) if e.local_name() == $event.local_name() => {
                    break;
                }
                Ok(Event::End(ref e)) => {
                    let expected = $event.unescaped()
                        .map(|s| String::from_utf8_lossy(s.as_ref()).to_string())?;
                    let found = String::from_utf8_lossy(e.name()).to_string();
                    let e = XmlError::EndEventMismatch { expected, found };
                    return Err(Error::from(e));
                }
                _ => continue,
            }
        }
    })
}

#[allow(unused_macros)]
macro_rules! parse_inner_impl {
    ( $x:ident, $name:expr ) => ();
    ( $x:ident, $name:expr, ) => ();
    ( $x:ident, $name:expr, $e:ident @ $l:expr => $r:expr ) => (
        if $name == $l {
            let $e = $x.clone().into_owned();
            $r;
            continue;
        }
    );
    ( $x:ident, $name:expr, $l:expr => $r:expr ) => (
        if $name == $l {
            $r;
            continue;
        }
    );
    ( $x:ident, $name:expr, $e:ident @ $l:expr => $r:expr, $($rest:tt)*) => (
        parse_inner_impl!( $x, $name, $e @ $l => $r );
        parse_inner_impl!( $x, $name, $($rest)* );
    );
    ( $x:ident, $name:expr, $l:expr => $r:expr, $($rest:tt)*) => (
        parse_inner_impl!( $x, $name, $l => $r );
        parse_inner_impl!( $x, $name, $($rest)* );
    )
}

#[allow(unused_macros)]
macro_rules! parse_comment {
    ( $event:ident, $reader:ident, $buffer:ident, $comment:ident ) => {
        parse_comment!{$event, $reader, $buffer, $comment, }
    };
    ( $event:ident, $reader:ident, $buffer:ident, $comment:ident, $($rest:tt)* ) => {
        parse_inner!{$event, $reader, $buffer,
            b"text" => {
                $comment.text.push($reader.read_text(b"text", $buffer)?);
            },
            m @ b"molecule" => {
                $comment.molecule = Molecule::from_xml(&m, $reader, $buffer)
                    .map(Some)?;
            },
            $($rest)*
        }
    }
}
