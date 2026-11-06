//! `NtfsFs::gather_records` from outside the crate: a caller building its own
//! view of a file (names, sizes, streams) needs the attributes in extension
//! records too, not only those in the base record.
//!
//! The fixture is `partition.dd` inside the committed `SampleTinyNtfsVolume.zip`
//! (see `vfs_ntfs.rs`); `istat -f ntfs partition.dd 37` shows file1.txt with no
//! `$ATTRIBUTE_LIST`, so its records are the base record alone.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::io::{Cursor, Read};

use ntfs_core::NtfsFs;

const SAMPLE_ZIP: &[u8] = include_bytes!("../../tests/data/SampleTinyNtfsVolume.zip");

fn open_real_volume() -> NtfsFs<Cursor<Vec<u8>>> {
    let mut archive = zip::ZipArchive::new(Cursor::new(SAMPLE_ZIP)).expect("open sample zip");
    let mut dd = Vec::new();
    archive
        .by_name("SampleTinyNtfsVolume/partition.dd")
        .expect("partition.dd present")
        .read_to_end(&mut dd)
        .expect("read partition.dd");
    NtfsFs::open(Cursor::new(dd)).expect("open NTFS volume")
}

#[test]
fn gather_records_is_the_base_record_alone_without_an_attribute_list() {
    let fs = open_real_volume();
    let base = fs.read_record(37).expect("record 37");
    let records = fs.gather_records(37, &base).expect("gather");
    assert_eq!(records, vec![base]);
}
