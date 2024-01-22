// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::mem;
pub enum EventSegmentOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct EventSegment<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for EventSegment<'a> {
    type Inner = EventSegment<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> EventSegment<'a> {
    pub const VT_FIRST_BLOCK_NUMBER: flatbuffers::VOffsetT = 4;
    pub const VT_BLOCKS: flatbuffers::VOffsetT = 6;

    pub const fn get_fully_qualified_name() -> &'static str {
        "EventSegment"
    }

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        EventSegment { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args EventSegmentArgs<'args>,
    ) -> flatbuffers::WIPOffset<EventSegment<'bldr>> {
        let mut builder = EventSegmentBuilder::new(_fbb);
        builder.add_first_block_number(args.first_block_number);
        if let Some(x) = args.blocks {
            builder.add_blocks(x);
        }
        builder.finish()
    }

    #[inline]
    pub fn first_block_number(&self) -> u64 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u64>(EventSegment::VT_FIRST_BLOCK_NUMBER, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn blocks(
        &self,
    ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<BlockEvents<'a>>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<BlockEvents>>,
            >>(EventSegment::VT_BLOCKS, None)
        }
    }
}

impl flatbuffers::Verifiable for EventSegment<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<u64>("first_block_number", Self::VT_FIRST_BLOCK_NUMBER, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<BlockEvents>>,
            >>("blocks", Self::VT_BLOCKS, false)?
            .finish();
        Ok(())
    }
}
pub struct EventSegmentArgs<'a> {
    pub first_block_number: u64,
    pub blocks: Option<
        flatbuffers::WIPOffset<
            flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<BlockEvents<'a>>>,
        >,
    >,
}
impl<'a> Default for EventSegmentArgs<'a> {
    #[inline]
    fn default() -> Self {
        EventSegmentArgs {
            first_block_number: 0,
            blocks: None,
        }
    }
}

pub struct EventSegmentBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> EventSegmentBuilder<'a, 'b> {
    #[inline]
    pub fn add_first_block_number(&mut self, first_block_number: u64) {
        self.fbb_
            .push_slot::<u64>(EventSegment::VT_FIRST_BLOCK_NUMBER, first_block_number, 0);
    }
    #[inline]
    pub fn add_blocks(
        &mut self,
        blocks: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<BlockEvents<'b>>>,
        >,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(EventSegment::VT_BLOCKS, blocks);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> EventSegmentBuilder<'a, 'b> {
        let start = _fbb.start_table();
        EventSegmentBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<EventSegment<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for EventSegment<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("EventSegment");
        ds.field("first_block_number", &self.first_block_number());
        ds.field("blocks", &self.blocks());
        ds.finish()
    }
}
