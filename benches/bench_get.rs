#![feature(test)]
extern crate test;

use git_info2;
use test::Bencher;

#[bench]
fn get(bencher: &mut Bencher) {
    bencher.iter(|| {
        let info = git_info2::get();

        assert!(info.current_branch.is_some());
    });
}
