#![warn(clippy::pedantic)]

use gildedrose::{GildedRose, Item};

#[test]
fn a_regular_item_loses_one_quality_before_its_sell_by_date() {
    let items = vec![Item::new("Elixir of the Mongoose", 10, 20)];
    let mut rose = GildedRose::new(items);

    rose.update_quality();

    assert_eq!((9, 19), (rose.items[0].sell_in, rose.items[0].quality));
}

#[test]
fn a_regular_item_loses_two_quality_after_its_sell_by_date() {
    let items = vec![Item::new("Elixir of the Mongoose", 0, 10)];
    let mut rose = GildedRose::new(items);

    rose.update_quality();

    assert_eq!((-1, 8), (rose.items[0].sell_in, rose.items[0].quality));
}

#[test]
fn a_regular_items_quality_never_falls_below_zero() {
    let items = vec![Item::new("Elixir of the Mongoose", 0, 0)];
    let mut rose = GildedRose::new(items);

    rose.update_quality();

    assert_eq!(0, rose.items[0].quality);
}

#[test]
fn aged_brie_increases_in_quality_before_its_sell_by_date() {
    let items = vec![Item::new("Aged Brie", 10, 20)];
    let mut rose = GildedRose::new(items);

    rose.update_quality();

    assert_eq!((9, 21), (rose.items[0].sell_in, rose.items[0].quality));
}

#[test]
fn aged_brie_increases_twice_in_quality_after_its_sell_by_date() {
    let items = vec![Item::new("Aged Brie", 0, 20)];
    let mut rose = GildedRose::new(items);

    rose.update_quality();

    assert_eq!((-1, 22), (rose.items[0].sell_in, rose.items[0].quality));
}

#[test]
fn aged_bries_quality_never_exceeds_fifty() {
    let items = vec![Item::new("Aged Brie", 0, 50)];
    let mut rose = GildedRose::new(items);

    rose.update_quality();

    assert_eq!(50, rose.items[0].quality);
}

#[test]
fn sulfuras_never_changes() {
    let items = vec![Item::new("Sulfuras, Hand of Ragnaros", 0, 80)];
    let mut rose = GildedRose::new(items);

    rose.update_quality();

    assert_eq!((0, 80), (rose.items[0].sell_in, rose.items[0].quality));
}

#[test]
fn backstage_pass_quality_increases_by_one_when_more_than_ten_days_remain() {
    let items = vec![Item::new(
        "Backstage passes to a TAFKAL80ETC concert",
        11,
        20,
    )];
    let mut rose = GildedRose::new(items);

    rose.update_quality();

    assert_eq!((10, 21), (rose.items[0].sell_in, rose.items[0].quality));
}

#[test]
fn backstage_pass_quality_increases_by_two_with_ten_days_remaining() {
    let items = vec![Item::new(
        "Backstage passes to a TAFKAL80ETC concert",
        10,
        20,
    )];
    let mut rose = GildedRose::new(items);

    rose.update_quality();

    assert_eq!((9, 22), (rose.items[0].sell_in, rose.items[0].quality));
}

#[test]
fn backstage_pass_quality_increases_by_three_with_five_days_remaining() {
    let items = vec![Item::new(
        "Backstage passes to a TAFKAL80ETC concert",
        5,
        20,
    )];
    let mut rose = GildedRose::new(items);

    rose.update_quality();

    assert_eq!((4, 23), (rose.items[0].sell_in, rose.items[0].quality));
}

#[test]
fn backstage_pass_quality_drops_to_zero_after_the_concert() {
    let items = vec![Item::new(
        "Backstage passes to a TAFKAL80ETC concert",
        0,
        20,
    )];
    let mut rose = GildedRose::new(items);

    rose.update_quality();

    assert_eq!((-1, 0), (rose.items[0].sell_in, rose.items[0].quality));
}

#[test]
fn backstage_pass_quality_never_exceeds_fifty() {
    let items = vec![Item::new(
        "Backstage passes to a TAFKAL80ETC concert",
        5,
        49,
    )];
    let mut rose = GildedRose::new(items);

    rose.update_quality();

    assert_eq!(50, rose.items[0].quality);
}
