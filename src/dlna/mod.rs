use futures::{Future, Stream};
use hyper;
use percent_encoding;
use serde;
use serde_xml_rs;
use std;

use failure::Error;
// use error::ResultExt;

pub mod discovery;
pub mod server;
pub mod types;

const UDN: &str = "uuid:06289e13-a832-4d76-be0b-00151d439863";

#[derive(Debug)]
struct Request {
	req: hyper::Request,
	path_offset: usize,
}

impl Request {
	fn new(req: hyper::Request) -> Self {
		Request {
			path_offset: if req.path().starts_with('/') { 1 } else { 0 },
			req: req,
		}
	}

	fn path(&self) -> &str { &self.req.path()[self.path_offset..] }

	fn decoded_path(&self) -> Result<String, Error> {
		percent_encoding::percent_decode(self.path().as_bytes())
			.decode_utf8()
			.map_err(|e| format_err!("Error percent-decoding path to utf8: {:?}", e))
			.map(|s| s.to_string())
	}

	fn pop(&mut self) -> &str {
		let next_chunk_start = self.path_offset;
		let next_chunk_end = match self.path().find('/') {
			Some(i) => {
				self.path_offset += i + 1;
				next_chunk_start + i
			}
			None => {
				self.path_offset = self.req.path().len();
				self.path_offset
			}
		};

		let next_chunk = &self.req.path()[next_chunk_start..next_chunk_end];
		// eprintln!("Pop {:?} from {:?}", next_chunk, self.path());
		return next_chunk
	}

	fn body_vec(self) -> Box<Future<Item=Vec<u8>, Error=Error>> {
		Box::new(self.req.body()
			.then(|r| r.map_err(|e| format_err!("Parsing request body: {:?}", e)))
			.fold(Vec::new(), |mut v, chunk| {
				v.extend(chunk);
				Ok::<_,Error>(v)
			}))
	}

	fn body_str_lossy(self) -> Box<Future<Item=String, Error=Error>> {
		Box::new(self.req.body()
			.then(|r| r.map_err(|e| format_err!("Parsing request body: {:?}", e)))
			.fold(String::new(), |mut s, chunk| {
				s += &String::from_utf8_lossy(&chunk);
				Ok::<_, Error>(s)
			}))
	}

	fn to_xml<B: 'static + serde::Deserialize<'static> + std::fmt::Debug>(self)
		-> Box<Future<Item=types::Envelope<B>, Error=Error>>
	{
		Box::new(self.body_vec()
			.and_then(|v| {
				eprintln!("Parsing xml: {}", String::from_utf8_lossy(&v));
				serde_xml_rs::deserialize(&v[..])
					.map_err(|e| format_err!("Error parsing xml:\n{}", String::from_utf8_lossy(&v)))
			})
			.inspect(|xml| eprintln!("Request: {:#?}", xml)))
	}
}
