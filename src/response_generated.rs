// automatically generated by the FlatBuffers compiler, do not modify


#![allow(unused_imports, dead_code)]

use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(unused_imports, dead_code)]
pub mod eisenbeton {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;
#[allow(unused_imports, dead_code)]
pub mod wire {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;
#[allow(unused_imports, dead_code)]
pub mod response {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

pub enum EisenResponseOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct EisenResponse<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for EisenResponse<'a> {
    type Inner = EisenResponse<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> EisenResponse<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        EisenResponse {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args EisenResponseArgs<'args>) -> flatbuffers::WIPOffset<EisenResponse<'bldr>> {
      let mut builder = EisenResponseBuilder::new(_fbb);
      if let Some(x) = args.content { builder.add_content(x); }
      if let Some(x) = args.headers { builder.add_headers(x); }
      builder.add_status(args.status);
      builder.finish()
    }

    pub const VT_STATUS: flatbuffers::VOffsetT = 4;
    pub const VT_HEADERS: flatbuffers::VOffsetT = 6;
    pub const VT_CONTENT: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn status(&self) -> i32 {
    self._tab.get::<i32>(EisenResponse::VT_STATUS, Some(0)).unwrap()
  }
  #[inline]
  pub fn headers(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Header<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Header>>>>(EisenResponse::VT_HEADERS, None)
  }
  #[inline]
  pub fn content(&self) -> Option<&'a [u8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(EisenResponse::VT_CONTENT, None).map(|v| v.safe_slice())
  }
}

impl flatbuffers::Verifiable for EisenResponse<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i32>(&"status", Self::VT_STATUS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Header>>>>(&"headers", Self::VT_HEADERS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(&"content", Self::VT_CONTENT, false)?
     .finish();
    Ok(())
  }
}
pub struct EisenResponseArgs<'a> {
    pub status: i32,
    pub headers: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Header<'a>>>>>,
    pub content: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
}
impl<'a> Default for EisenResponseArgs<'a> {
    #[inline]
    fn default() -> Self {
        EisenResponseArgs {
            status: 0,
            headers: None,
            content: None,
        }
    }
}
pub struct EisenResponseBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> EisenResponseBuilder<'a, 'b> {
  #[inline]
  pub fn add_status(&mut self, status: i32) {
    self.fbb_.push_slot::<i32>(EisenResponse::VT_STATUS, status, 0);
  }
  #[inline]
  pub fn add_headers(&mut self, headers: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Header<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(EisenResponse::VT_HEADERS, headers);
  }
  #[inline]
  pub fn add_content(&mut self, content: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(EisenResponse::VT_CONTENT, content);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> EisenResponseBuilder<'a, 'b> {
    let start = _fbb.start_table();
    EisenResponseBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<EisenResponse<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for EisenResponse<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("EisenResponse");
      ds.field("status", &self.status());
      ds.field("headers", &self.headers());
      ds.field("content", &self.content());
      ds.finish()
  }
}
pub enum HeaderOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Header<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Header<'a> {
    type Inner = Header<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> Header<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Header {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args HeaderArgs<'args>) -> flatbuffers::WIPOffset<Header<'bldr>> {
      let mut builder = HeaderBuilder::new(_fbb);
      if let Some(x) = args.value { builder.add_value(x); }
      if let Some(x) = args.key { builder.add_key(x); }
      builder.finish()
    }

    pub const VT_KEY: flatbuffers::VOffsetT = 4;
    pub const VT_VALUE: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn key(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Header::VT_KEY, None)
  }
  #[inline]
  pub fn value(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Header::VT_VALUE, None)
  }
}

impl flatbuffers::Verifiable for Header<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>(&"key", Self::VT_KEY, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>(&"value", Self::VT_VALUE, false)?
     .finish();
    Ok(())
  }
}
pub struct HeaderArgs<'a> {
    pub key: Option<flatbuffers::WIPOffset<&'a str>>,
    pub value: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for HeaderArgs<'a> {
    #[inline]
    fn default() -> Self {
        HeaderArgs {
            key: None,
            value: None,
        }
    }
}
pub struct HeaderBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> HeaderBuilder<'a, 'b> {
  #[inline]
  pub fn add_key(&mut self, key: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Header::VT_KEY, key);
  }
  #[inline]
  pub fn add_value(&mut self, value: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Header::VT_VALUE, value);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> HeaderBuilder<'a, 'b> {
    let start = _fbb.start_table();
    HeaderBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Header<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Header<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Header");
      ds.field("key", &self.key());
      ds.field("value", &self.value());
      ds.finish()
  }
}
#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_eisen_response<'a>(buf: &'a [u8]) -> EisenResponse<'a> {
  unsafe { flatbuffers::root_unchecked::<EisenResponse<'a>>(buf) }
}

#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_eisen_response<'a>(buf: &'a [u8]) -> EisenResponse<'a> {
  unsafe { flatbuffers::size_prefixed_root_unchecked::<EisenResponse<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `EisenResponse`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_eisen_response_unchecked`.
pub fn root_as_eisen_response(buf: &[u8]) -> Result<EisenResponse, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<EisenResponse>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `EisenResponse` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_eisen_response_unchecked`.
pub fn size_prefixed_root_as_eisen_response(buf: &[u8]) -> Result<EisenResponse, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<EisenResponse>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `EisenResponse` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_eisen_response_unchecked`.
pub fn root_as_eisen_response_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<EisenResponse<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<EisenResponse<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `EisenResponse` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_eisen_response_unchecked`.
pub fn size_prefixed_root_as_eisen_response_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<EisenResponse<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<EisenResponse<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a EisenResponse and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `EisenResponse`.
pub unsafe fn root_as_eisen_response_unchecked(buf: &[u8]) -> EisenResponse {
  flatbuffers::root_unchecked::<EisenResponse>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed EisenResponse and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `EisenResponse`.
pub unsafe fn size_prefixed_root_as_eisen_response_unchecked(buf: &[u8]) -> EisenResponse {
  flatbuffers::size_prefixed_root_unchecked::<EisenResponse>(buf)
}
#[inline]
pub fn finish_eisen_response_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<EisenResponse<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_eisen_response_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<EisenResponse<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod response
}  // pub mod wire
}  // pub mod eisenbeton
