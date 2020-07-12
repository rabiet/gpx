//! extensions handles parsing of GPX-spec extensions.

// TODO: extensions are not implemented

use std::io::Read;

use error_chain::ensure;
use xml::reader::XmlEvent;

use crate::errors::*;
use crate::parser::{Context, string};
use crate::Extension;

/// consume consumes a single string as tag content.
pub fn consume<R: Read>(context: &mut Context<R>) -> Result<(Extension)> {
    let mut extension: Extension = Extension { speed: None, accuracy: None, bearing: None };
    loop {
        let next_event = {
            if let Some(next) = context.reader.peek() {
                match next {
                    Ok(n) => n,
                    Err(_) => panic!("error while parsing waypoint event"),
                }
            } else {
                break;
            }
        };

        match next_event {
            XmlEvent::StartElement { ref name, .. } => {
                match name.local_name.as_ref() {
                    "speed" => {
                        extension.speed = Some(
                            string::consume(context, "speed", false)?
                                .parse()
                                .chain_err(|| "error while casting speed to f64")?,
                        );
                    },
                    "extensions" => {
                        context.reader.next();
                    },
                    "accuracy" => {
                        extension.accuracy = Some(
                            string::consume(context, "accuracy", false)?
                                .parse()
                                .chain_err(|| "error while casting speed to f64")?,
                        );
                    },
                    "bearing" => {
                        extension.bearing = Some(
                            string::consume(context, "bearing", false)?
                                .parse()
                                .chain_err(|| "error while casting speed to f64")?,
                        );
                    }
                    x => {
                        println!("Unknown tag in extensions!");
                    }
                }
            }
            XmlEvent::EndElement { ref name } => {
                context.reader.next(); //consume the end tag
                return Ok(extension);
            }
            _ => {
                println!("What is this tag?");
                context.reader.next(); //consume and ignore this event
            }
        }
    }
    panic!("Couldn't parse extension");
}

#[cfg(test)]
mod tests {
    use super::consume;
    use crate::GpxVersion;

    #[test]
    fn consume_arbitrary_extensions() {
        let result = consume!(
            "<extensions>
                hello world
                <a><b cond=\"no\"><c>derp</c></b></a>
                <tag>yadda yadda we dont care</tag>
            </extensions>",
            GpxVersion::Gpx11
        );

        assert!(result.is_ok());
    }
}
