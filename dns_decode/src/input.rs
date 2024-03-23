// Largely copied from https://github.com/fflorent/nom_locate/blob/master/src/lib.rs

use std::{
    iter::{Copied, Enumerate},
    ops::{Range, RangeFrom, RangeFull, RangeTo},
    slice::Iter,
    str::FromStr,
};

use nom::{
    error::{Error, ErrorKind, ParseError},
    AsBytes, Compare, CompareResult, Err, ErrorConvert, ExtendInto, FindSubstring, FindToken,
    InputIter, InputLength, InputTake, InputTakeAtPosition, Offset, ParseTo, Slice,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DnsFrameInput<'a> {
    pub input: &'a [u8],
    pub frame: &'a [u8],
}

impl<'a> DnsFrameInput<'a> {
    pub fn new(frame: &'a [u8]) -> Self {
        Self {
            frame: frame,
            input: frame,
        }
    }
}

// impl<'a> ParseError<&[u8]> for DnsFrameInput<'a> {
//     fn from_error_kind(input: &[u8], kind: ErrorKind) -> Self {
//         DnsFrameInput {
//             frame: input,
//             input: input,
//         }
//     }

//     fn append(input: &[u8], kind: ErrorKind, other: Self) -> Self {
//         todo!()
//     }
// }

impl<'a> AsBytes for DnsFrameInput<'a> {
    fn as_bytes(&self) -> &[u8] {
        self.input.as_bytes()
    }
}

impl<'a, 'b> Compare<DnsFrameInput<'b>> for DnsFrameInput<'a> {
    #[inline(always)]
    fn compare(&self, t: DnsFrameInput<'b>) -> CompareResult {
        self.input.compare(t.input)
    }

    #[inline(always)]
    fn compare_no_case(&self, t: DnsFrameInput<'b>) -> CompareResult {
        self.input.compare_no_case(t.input)
    }
}

impl<'a, 'b> Compare<&'b [u8]> for DnsFrameInput<'a> {
    #[inline(always)]
    fn compare(&self, t: &'b [u8]) -> CompareResult {
        self.input.compare(t)
    }

    #[inline(always)]
    fn compare_no_case(&self, t: &'b [u8]) -> CompareResult {
        self.input.compare_no_case(t)
    }
}

impl<'a> Compare<&[u8; 1]> for DnsFrameInput<'a> {
    #[inline(always)]
    fn compare(&self, t: &[u8; 1]) -> CompareResult {
        self.input.compare(t)
    }

    #[inline(always)]
    fn compare_no_case(&self, t: &[u8; 1]) -> CompareResult {
        self.input.compare_no_case(t)
    }
}

impl<'a> ExtendInto for DnsFrameInput<'a> {
    type Item = &'a [u8];
    type Extender = Vec<u8>;

    #[inline]
    fn new_builder(&self) -> Self::Extender {
        self.input.new_builder()
    }

    #[inline]
    fn extend_into(&self, acc: &mut Self::Extender) {
        self.input.extend_into(acc)
    }
}

impl<'a, 'b> FindSubstring<&'b [u8]> for DnsFrameInput<'a> {
    #[inline]
    fn find_substring(&self, substr: &'b [u8]) -> Option<usize> {
        self.input.find_substring(substr)
    }
}

impl<'a, 'b> FindToken<u8> for DnsFrameInput<'a> {
    fn find_token(&self, token: u8) -> bool {
        self.input.find_token(token)
    }
}

impl<'a> InputIter for DnsFrameInput<'a> {
    type Item = u8;
    type Iter = Enumerate<Self::IterElem>;
    type IterElem = Copied<Iter<'a, u8>>;

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.input.iter_indices()
    }
    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.input.iter_elements()
    }
    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.input.position(predicate)
    }
    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        self.input.slice_index(count)
    }
}

impl<'a> InputLength for DnsFrameInput<'a> {
    fn input_len(&self) -> usize {
        self.input.input_len()
    }
}

impl<'a> InputTake for DnsFrameInput<'a>
where
    Self: Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
{
    fn take(&self, count: usize) -> Self {
        self.slice(..count)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        (self.slice(count..), self.slice(..count))
    }
}

// impl<'a> InputTakeAtPosition for DnsFrameInput<'a> {
//     type Item = u8;

//     fn split_at_position<P, E: nom::error::ParseError<Self>>(
//         &self,
//         predicate: P,
//     ) -> nom::IResult<Self, Self, E>
//     where
//         P: Fn(Self::Item) -> bool,
//     {
//         let r: Result<(DnsFrameInput<'_>, DnsFrameInput<'_>), Err<_>> =
//             match self.input.split_at_position::<P, Err<&'a [u8]>(predicate) {
//                 Ok((prefix, suffix)) => Ok((
//                     DnsFrameInput {
//                         input: prefix,
//                         frame: self.frame,
//                     },
//                     DnsFrameInput {
//                         input: suffix,
//                         frame: self.frame,
//                     },
//                 )),
//                 Err(e) => Err(e),
//             };
//     }

//     fn split_at_position1<P, E: nom::error::ParseError<Self>>(
//         &self,
//         predicate: P,
//         e: nom::error::ErrorKind,
//     ) -> nom::IResult<Self, Self, E>
//     where
//         P: Fn(Self::Item) -> bool,
//     {
//         match self.input.split_at_position1(predicate, e) {
//             Ok((prefix, suffix)) => Ok((
//                 DnsFrameInput {
//                     input: prefix,
//                     frame: self.frame,
//                 },
//                 DnsFrameInput {
//                     input: suffix,
//                     frame: self.frame,
//                 },
//             )),
//             Err(e) => Err(Err::Incomplete(nom::Needed::Unknown)),
//         }
//     }

//     fn split_at_position_complete<P, E: nom::error::ParseError<Self>>(
//         &self,
//         predicate: P,
//     ) -> nom::IResult<Self, Self, E>
//     where
//         P: Fn(Self::Item) -> bool,
//     {
//         match self.input.split_at_position_complete(predicate) {
//             Ok((prefix, suffix)) => Ok((
//                 DnsFrameInput {
//                     input: prefix,
//                     frame: self.frame,
//                 },
//                 DnsFrameInput {
//                     input: suffix,
//                     frame: self.frame,
//                 },
//             )),
//             Err(e) => Err(Err::Incomplete(nom::Needed::Unknown)),
//         }
//     }

//     fn split_at_position1_complete<P, E: nom::error::ParseError<Self>>(
//         &self,
//         predicate: P,
//         e: nom::error::ErrorKind,
//     ) -> nom::IResult<Self, Self, E>
//     where
//         P: Fn(Self::Item) -> bool,
//     {
//         match self.input.split_at_position1_complete(predicate, e) {
//             Ok((prefix, suffix)) => Ok((
//                 DnsFrameInput {
//                     input: prefix,
//                     frame: self.frame,
//                 },
//                 DnsFrameInput {
//                     input: suffix,
//                     frame: self.frame,
//                 },
//             )),
//             Err(e) => Err(Err::Incomplete(nom::Needed::Unknown)),
//         }
//     }
// }

impl<'a> Offset for DnsFrameInput<'a> {
    fn offset(&self, second: &Self) -> usize {
        self.input.offset(second.input)
    }
}

impl<'a, R: FromStr> ParseTo<R> for DnsFrameInput<'a> {
    #[inline]
    fn parse_to(&self) -> Option<R> {
        self.input.parse_to()
    }
}

impl<'a> Slice<Range<usize>> for DnsFrameInput<'a> {
    fn slice(&self, range: Range<usize>) -> Self {
        let input = self.input.slice(range);
        DnsFrameInput {
            frame: self.frame,
            input: input,
        }
    }
}

impl<'a> Slice<RangeTo<usize>> for DnsFrameInput<'a> {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        let input = self.input.slice(range);
        DnsFrameInput {
            frame: self.frame,
            input: input,
        }
    }
}

impl<'a> Slice<RangeFrom<usize>> for DnsFrameInput<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        let input = self.input.slice(range);
        DnsFrameInput {
            frame: self.frame,
            input: input,
        }
    }
}

impl<'a> Slice<RangeFull> for DnsFrameInput<'a> {
    fn slice(&self, range: RangeFull) -> Self {
        let input = self.input.slice(range);
        DnsFrameInput {
            frame: self.frame,
            input: input,
        }
    }
}
