/*
   OxiRush
   Copyright 2026 Valentin D'Emmanuele

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

//! The traffic flow templates of TS 24.008 clause 10.5.6.12 from its octet 3: the Bearer
//! TFT (§8.19, IE type 84) and the Traffic Aggregate Description (§8.20, IE type 85).

use alloc::vec::Vec;

use crate::Error;
use crate::check::{Deviation, DeviationKind, Path, Rules};
use crate::ie::Codec;
use oxirush_utils::fields::Octets;

/// A packet filter of a template: its identifier octet (its direction in bits 6 and 5,
/// its identifier in bits 4 to 1), and, but in a deletion of packet filters (operation
/// 5), its evaluation precedence and its contents after their length.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PacketFilter {
    /// The octet of the identifier and the direction, its spare bits 8 and 7 included.
    pub identifier: u8,
    /// The evaluation precedence and the contents: none in a deletion of packet filters.
    pub filter: Option<(u8, Octets)>,
}

/// A parameter of a template, after the E bit: its identifier and its contents.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TftParameter {
    /// The parameter identifier: 1 authorization token, 2 flow identifier, 3 packet
    /// filter identifier.
    pub identifier: u8,
    /// Its contents, after their length.
    pub contents: Octets,
}

macro_rules! template {
    ($($(#[$meta:meta])* $ty:ident $name:literal $clause:literal;)*) => {$(
        $(#[$meta])*
        #[derive(Debug, Clone, Default, PartialEq, Eq)]
        pub struct $ty {
            /// The TFT operation code: 1 create new TFT, 2 delete existing TFT, 3 add packet
            /// filters, 4 replace packet filters, 5 delete packet filters, 6 no TFT
            /// operation.
            pub operation: u8,
            /// The packet filters, at most 15.
            pub filters: Vec<PacketFilter>,
            /// The parameters list, which the E bit announces.
            pub parameters: Option<Vec<TftParameter>>,
            /// The octets after the packet filters of a template without parameters.
            pub extra: Octets,
        }

        impl Codec for $ty {
            fn decode(octets: &[u8]) -> Option<Self> {
                let (template, extra) = decode(octets)?;
                Some(Self { operation: template.0, filters: template.1, parameters: template.2, extra })
            }
            fn encode(&self, out: &mut Vec<u8>) -> Result<(), Error> {
                encode($name, self.operation, &self.filters, self.parameters.as_deref(), &self.extra, out)
            }
            fn encoded_len(&self) -> usize {
                let filters: usize = self.filters.iter().map(|f| 1 + f.filter.as_ref().map_or(0, |(_, c)| 2 + c.len())).sum();
                let parameters: usize = self.parameters.iter().flatten().map(|p| 2 + p.contents.len()).sum();
                1 + filters + parameters + self.extra.len()
            }
        }

        impl Rules for $ty {
            fn check(&self, path: &Path<'_>, found: &mut Vec<Deviation>) {
                check(self.operation, &self.filters, $clause, path, found);
            }
        }

        impl $ty {
            /// The names of its fields, as its tree writes them.
            pub(crate) const FIELDS: &'static [&'static str] = FIELDS;
            /// The members of its tree.
            pub(crate) const MEMBERS: &'static [&'static str] = FIELDS;
            /// What its tree takes.
            #[cfg(feature = "tree")]
            pub(crate) const TAKES: &'static oxirush_utils::tree::Takes = &oxirush_utils::tree::Takes::Members(None, &[]);
            /// Its layout, which the test spec does not compare yet.
            #[doc(hidden)]
            #[cfg(feature = "spec")]
            pub const LAYOUT: Option<&'static oxirush_utils::fields::Layout> = None;
        }

        #[cfg(feature = "tree")]
        impl oxirush_utils::tree::Tree for $ty {
            const NAME: &'static str = $name;
            fn to_tree(&self) -> oxirush_utils::tree::Value {
                to_tree(self.operation, &self.filters, self.parameters.as_deref(), &self.extra)
            }
            fn from_tree(value: &oxirush_utils::tree::Value, reading: &mut oxirush_utils::tree::Reading) -> Result<Self, oxirush_utils::tree::Refused> {
                let (operation, filters, parameters, extra) = from_tree(value, reading)?;
                Ok(Self { operation, filters, parameters, extra })
            }
        }
    )*};
}

template! {
    /// EPS Bearer Level Traffic Flow Template (Bearer TFT) (§8.19).
    EpsBearerLevelTrafficFlowTemplate "EPS Bearer Level Traffic Flow Template (Bearer TFT)" "8.19";
    /// Traffic Aggregation Description (TAD) (§8.20).
    Tad "Traffic Aggregation Description (TAD)" "8.20";
}

const FIELDS: &[&str] = &[
    "TFT operation code",
    "Packet filters",
    "Parameters",
    "unknown octets",
];

/// The operation that lists the identifiers of packet filters alone.
const DELETE_FILTERS: u8 = 5;

type Template = (u8, Vec<PacketFilter>, Option<Vec<TftParameter>>);

fn decode(octets: &[u8]) -> Option<(Template, Octets)> {
    let (&first, mut rest) = octets.split_first()?;
    let (operation, e, count) = (first >> 5, first & 0x10 != 0, first & 0x0f);
    let mut filters = Vec::new();
    for _ in 0..count {
        let (&identifier, after) = rest.split_first()?;
        rest = after;
        let filter = match operation {
            DELETE_FILTERS => None,
            _ => {
                let [precedence, length, after @ ..] = rest else {
                    return None;
                };
                let (contents, after) = after.split_at_checked(usize::from(*length))?;
                rest = after;
                Some((*precedence, Octets::copied(contents)))
            }
        };
        filters.push(PacketFilter { identifier, filter });
    }
    let parameters = match e {
        true => {
            let mut parameters = Vec::new();
            while let [identifier, length, after @ ..] = rest {
                let (contents, after) = after.split_at_checked(usize::from(*length))?;
                parameters.push(TftParameter {
                    identifier: *identifier,
                    contents: Octets::copied(contents),
                });
                rest = after;
            }
            if !rest.is_empty() {
                return None;
            }
            Some(parameters)
        }
        false => None,
    };
    Some(((operation, filters, parameters), Octets::copied(rest)))
}

fn encode(
    name: &'static str,
    operation: u8,
    filters: &[PacketFilter],
    parameters: Option<&[TftParameter]>,
    extra: &Octets,
    out: &mut Vec<u8>,
) -> Result<(), Error> {
    let count = u8::try_from(filters.len()).ok().filter(|count| *count < 16);
    let count = count.ok_or(Error::FieldLength {
        ie: name,
        field: "Number of packet filters",
    })?;
    if operation > 7 {
        return Err(Error::FieldLength {
            ie: name,
            field: "TFT operation code",
        });
    }
    let inconsistent = |field| Error::Inconsistent {
        ie: name,
        field,
        clause: "TS 24.008 10.5.6.12",
    };
    if parameters.is_some() && !extra.is_empty() {
        return Err(inconsistent("the octets after a parameters list"));
    }
    out.push(operation << 5 | u8::from(parameters.is_some()) << 4 | count);
    for filter in filters {
        out.push(filter.identifier);
        match (&filter.filter, operation) {
            (None, DELETE_FILTERS) => {}
            (Some((precedence, contents)), operation) if operation != DELETE_FILTERS => {
                let length = u8::try_from(contents.len()).map_err(|_| Error::FieldLength {
                    ie: name,
                    field: "Packet filter contents",
                })?;
                out.extend_from_slice(&[*precedence, length]);
                out.extend_from_slice(contents);
            }
            _ => return Err(inconsistent("a packet filter of another operation")),
        }
    }
    for parameter in parameters.into_iter().flatten() {
        let length = u8::try_from(parameter.contents.len()).map_err(|_| Error::FieldLength {
            ie: name,
            field: "Parameter contents",
        })?;
        out.extend_from_slice(&[parameter.identifier, length]);
        out.extend_from_slice(&parameter.contents);
    }
    out.extend_from_slice(extra);
    Ok(())
}

fn check(
    operation: u8,
    filters: &[PacketFilter],
    clause: &'static str,
    path: &Path<'_>,
    found: &mut Vec<Deviation>,
) {
    let rule = |what| Deviation::new(path, clause, DeviationKind::Rule(what));
    if !(1..=6).contains(&operation) {
        found.push(Deviation::new(
            path,
            clause,
            DeviationKind::ReservedValue {
                value: operation.into(),
            },
        ));
    }
    // "delete existing TFT" and "no TFT operation" have no packet filters; the others one
    // at least (TS 24.008 10.5.6.12)
    match (operation, filters.is_empty()) {
        (2 | 6, false) => found.push(rule("packet filters of an operation that has none")),
        (1 | 3 | 4 | 5, true) => found.push(rule("no packet filter of an operation that has some")),
        _ => {}
    }
    let spare = filters.iter().any(|filter| match operation {
        DELETE_FILTERS => filter.identifier > 0x0f,
        _ => filter.identifier > 0x3f,
    });
    if spare {
        found.push(Deviation::new(path, clause, DeviationKind::SpareBits));
    }
}

#[cfg(feature = "tree")]
fn to_tree(
    operation: u8,
    filters: &[PacketFilter],
    parameters: Option<&[TftParameter]>,
    extra: &Octets,
) -> oxirush_utils::tree::Value {
    use oxirush_utils::tree::{self, Members, Value};
    let mut m = Members::new();
    m.put("TFT operation code", tree::unsigned(operation.into()));
    if !filters.is_empty() {
        let entries = filters.iter().map(|filter| {
            let mut e = Members::new();
            e.put(
                "Packet filter identifier",
                tree::unsigned((filter.identifier & 0x0f).into()),
            );
            if operation != DELETE_FILTERS {
                e.put(
                    "Packet filter direction",
                    tree::unsigned((filter.identifier >> 4 & 0x03).into()),
                );
            }
            let spare = if operation == DELETE_FILTERS {
                filter.identifier >> 4
            } else {
                filter.identifier >> 6
            };
            if spare != 0 {
                e.put("Spare", tree::unsigned(spare.into()));
            }
            if let Some((precedence, contents)) = &filter.filter {
                e.put(
                    "Packet filter evaluation precedence",
                    tree::unsigned((*precedence).into()),
                );
                e.put("Packet filter contents", tree::octets(contents));
            }
            e.finish()
        });
        m.put("Packet filters", Value::Array(entries.collect()));
    }
    if let Some(parameters) = parameters {
        let entries = parameters.iter().map(|parameter| {
            let mut e = Members::new();
            e.put(
                "Parameter identifier",
                tree::unsigned(parameter.identifier.into()),
            );
            e.put("Parameter contents", tree::octets(&parameter.contents));
            e.finish()
        });
        m.put("Parameters", Value::Array(entries.collect()));
    }
    if !extra.is_empty() {
        m.put("unknown octets", tree::octets(extra));
    }
    m.finish()
}

#[cfg(feature = "tree")]
#[allow(clippy::type_complexity)]
fn from_tree(
    value: &oxirush_utils::tree::Value,
    reading: &mut oxirush_utils::tree::Reading,
) -> Result<(u8, Vec<PacketFilter>, Option<Vec<TftParameter>>, Octets), oxirush_utils::tree::Refused>
{
    let read = reading.members(value, FIELDS)?;
    let operation = reading.member(&read, "TFT operation code", |v, r| r.unsigned(v, 0, 7))? as u8;
    const FILTER: &[&str] = &[
        "Packet filter identifier",
        "Packet filter direction",
        "Spare",
        "Packet filter evaluation precedence",
        "Packet filter contents",
    ];
    let filters = reading
        .optional(&read, "Packet filters", |v, r| {
            r.list(v, |e, r| {
                let f = r.members(e, FILTER)?;
                let id =
                    r.member(&f, "Packet filter identifier", |v, r| r.unsigned(v, 0, 15))? as u8;
                let direction = r
                    .optional(&f, "Packet filter direction", |v, r| r.unsigned(v, 0, 3))?
                    .unwrap_or(0) as u8;
                let spare = r
                    .optional(&f, "Spare", |v, r| r.unsigned(v, 0, 15))?
                    .unwrap_or(0) as u8;
                let precedence =
                    r.optional(&f, "Packet filter evaluation precedence", |v, r| {
                        r.unsigned(v, 0, 255)
                    })?;
                let contents = r.optional(&f, "Packet filter contents", |v, r| r.octets(v))?;
                let identifier = match operation {
                    DELETE_FILTERS => spare << 4 | id,
                    _ => spare << 6 | direction << 4 | id,
                };
                let filter = match (precedence, contents) {
                    (Some(precedence), contents) => {
                        Some((precedence as u8, contents.unwrap_or_default().into()))
                    }
                    (None, None) => None,
                    (None, Some(_)) => {
                        return Err(r.refuse(
                            oxirush_utils::tree::ErrorKind::Value,
                            "contents without a precedence",
                        ));
                    }
                };
                Ok(PacketFilter { identifier, filter })
            })
        })?
        .unwrap_or_default();
    let parameters = reading.optional(&read, "Parameters", |v, r| {
        r.list(v, |e, r| {
            let p = r.members(e, &["Parameter identifier", "Parameter contents"])?;
            Ok(TftParameter {
                identifier: r.member(&p, "Parameter identifier", |v, r| r.unsigned(v, 0, 255))?
                    as u8,
                contents: r
                    .member(&p, "Parameter contents", |v, r| r.octets(v))?
                    .into(),
            })
        })
    })?;
    let extra = reading
        .optional(&read, "unknown octets", |v, r| r.octets(v))?
        .unwrap_or_default()
        .into();
    Ok((operation, filters, parameters, extra))
}
