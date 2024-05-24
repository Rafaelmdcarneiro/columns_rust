use crate::{
    block::{Block, BlockKind},
    column::Column,
    frame::{Drawable, Frame},
    point,
    timer::Timer,
    Point, NUM_COLS, NUM_ROWS, PIT_STARTING_X,
};
use std::time::Duration;
use std::{cmp::min, slice::Iter};

pub type Heap = [[Block; NUM_ROWS]; NUM_COLS];

#[derive(Debug)]
pub enum CardinalAxis {
    NxS,
    ExW,
    NExSW,
    NWxSE,
}

impl CardinalAxis {
    const SEEK_ORDER: [Self; 4] = [Self::NxS, Self::ExW, Self::NExSW, Self::NWxSE];

    pub fn iter<'a>() -> Iter<'a, CardinalAxis> {
        Self::SEEK_ORDER.iter()
    }
}

#[derive(Debug, PartialEq)]
enum PitStage {
    Stable,
    Matching,
    Collecting,
    Dropping,
}

pub struct PitState {
    stage: PitStage,
    move_timer: Timer,
    times: u8,
}

impl Default for PitState {
    fn default() -> Self {
        Self {
            stage: PitStage::Stable,
            move_timer: Timer::from_millis(Self::MOVE_MILLIS),
            times: 0,
        }
    }
}

impl PitState {
    const MOVE_MILLIS: u64 = 1000;
    pub const SCORE_MUL: usize = 10;

    pub fn update_dropping_at<const R: usize, const C: usize>(
        &self,
        heap: &mut [[Block; R]; C],
        origins: &mut [Point],
    ) -> bool {
        let mut something_dropped = false;
        // drop all active blocks one step if they have a slot for that
        for origin in origins.iter_mut() {
            if origin.y < R - 1 && heap[origin.x][origin.y + 1].empty() {
                // the slot below is empty, let's drop it we can drop one level!
                // and let's update things accordingly in the heap
                let new_item = heap[origin.x][origin.y].to_owned();
                heap[origin.x][origin.y].update(None);
                origin.y += 1;
                heap[origin.x][origin.y] = new_item;
                something_dropped = true;
            }
        }

        something_dropped
    }

    pub fn collect_dropping_at<const R: usize, const C: usize>(
        &self,
        heap: &[[Block; R]; C],
        origins: &[Point],
    ) -> Vec<Point> {
        let mut items = Vec::new();

        for origin in origins {
            for y in (0..origin.y).rev() {
                if heap[origin.x][y].empty() {
                    break;
                }
                items.push(point!(origin.x, y));
            }
        }

        // sort by highest 'y' points first, so we don't run into troubles when updating next...
        items.sort_unstable_by(|a, b| b.y.cmp(&a.y));

        items
    }

    pub fn collect_matching_at<const R: usize, const C: usize>(
        &self,
        heap: &[[Block; R]; C],
        origins: &[Point],
        partial_score: &mut usize,
    ) -> Vec<Point> {
        let mut items = Vec::new();
        let mut cache = [[false; R]; C];

        for origin in origins {
            let (matches, number_axes) = self.matching_at(heap, origin);

            for item in matches {
                if !cache[item.x][item.y] {
                    cache[item.x][item.y] = true;
                    items.push(item);
                    *partial_score += number_axes * Self::SCORE_MUL;
                }
            }
        }

        items
    }

    fn matching_at<const R: usize, const C: usize>(
        &self,
        heap: &[[Block; R]; C],
        origin: &Point,
    ) -> (Vec<Point>, usize) {
        let mut items = Vec::new();
        let mut matched_axes = 0;
        let origin_item = heap[origin.x][origin.y];

        if !origin_item.empty() {
            for axis in CardinalAxis::iter() {
                let mut matches: Vec<Point> = Vec::new();

                match axis {
                    CardinalAxis::NxS => {
                        // north (N)
                        for y in (0..origin.y).rev() {
                            if heap[origin.x][y] != origin_item {
                                break;
                            }
                            matches.push(point!(origin.x, y));
                        }
                        // south (S)
                        for y in (origin.y + 1)..R {
                            if heap[origin.x][y] != origin_item {
                                break;
                            }
                            matches.push(point!(origin.x, y));
                        }
                    }
                    CardinalAxis::ExW => {
                        // west (W)
                        for x in (0..origin.x).rev() {
                            if heap[x][origin.y] != origin_item {
                                break;
                            }
                            matches.push(point!(x, origin.y));
                        }
                        // east (E)
                        #[allow(clippy::needless_range_loop)]
                        for x in (origin.x + 1)..C {
                            if heap[x][origin.y] != origin_item {
                                break;
                            }
                            matches.push(point!(x, origin.y));
                        }
                    }
                    CardinalAxis::NExSW => {
                        // northeast (NE)
                        for i in 1..min(C - origin.x, origin.y + 1) {
                            if heap[origin.x + i][origin.y - i] != origin_item {
                                break;
                            }
                            matches.push(point!(origin.x + i, origin.y - i));
                        }
                        // southwest (SW)
                        for i in 1..min(R - origin.y, origin.x + 1) {
                            if heap[origin.x - i][origin.y + i] != origin_item {
                                break;
                            }
                            matches.push(point!(origin.x - i, origin.y + i));
                        }
                    }
                    CardinalAxis::NWxSE => {
                        // northwest (NW)
                        for i in 1..=min(origin.x, origin.y) {
                            if heap[origin.x - i][origin.y - i] != origin_item {
                                break;
                            }
                            matches.push(point!(origin.x - i, origin.y - i));
                        }
                        // southeast (SE)
                        for i in 1..min(C - origin.x, R - origin.y) {
                            if heap[origin.x + i][origin.y + i] != origin_item {
                                break;
                            }
                            matches.push(point!(origin.x + i, origin.y + i));
                        }
                    }
                }
                if matches.len() >= 2 {
                    matched_axes += 1;
                    items.append(&mut matches);
                }
            }

            if !items.is_empty() {
                items.push(origin.to_owned());
            }
        }

        (items, matched_axes)
    }
}

pub struct Pit {
    pub heap: Heap,
    state: PitState,
    active_origins: Vec<Point>,
    score: usize,
    blocks_score: usize,
}

impl Default for Pit {
    fn default() -> Self {
        Self {
            heap: Self::new_heap(None),
            active_origins: Vec::new(),
            state: PitState::default(),
            score: 0,
            blocks_score: 0,
        }
    }
}

impl Pit {
    pub fn new_heap<const R: usize, const C: usize>(
        block_kind: Option<BlockKind>,
    ) -> [[Block; R]; C] {
        [[Block::new(block_kind); R]; C]
    }

    pub fn update(&mut self, column: &mut Column, delta: Duration) -> (usize, usize) {
        use PitStage::*;

        match &self.state.stage {
            Stable => {
                if let Some(origins) = column.detect_landing(&mut self.heap, delta) {
                    self.active_origins = origins;
                    self.state.stage = Matching;
                    self.state.move_timer.finish();
                }
            }
            Matching => {
                let mut partial_score = 0;
                let items = self.state.collect_matching_at(
                    &self.heap,
                    &self.active_origins,
                    &mut partial_score,
                );
                // scoring
                self.score += partial_score;
                self.blocks_score += items.len();

                self.active_origins = items;

                self.state.stage = if self.active_origins.is_empty() {
                    Stable
                } else {
                    Collecting
                };
            }
            Collecting => {
                if self.state.times == 3 {
                    self.state.times = 0;

                    for item in self.active_origins.iter() {
                        self.heap[item.x][item.y].exploding = false;
                        self.heap[item.x][item.y].update(None);
                    }

                    self.active_origins = self
                        .state
                        .collect_dropping_at(&self.heap, &self.active_origins);

                    self.state.stage = if self.active_origins.is_empty() {
                        Stable
                    } else {
                        Dropping
                    };
                } else if self.state.move_timer.update(delta).ready() {
                    self.state.move_timer.reset();
                    self.state.times += 1;

                    if !self.active_origins.is_empty() {
                        let exploding = self.state.times % 2 != 0;

                        for item in self.active_origins.iter() {
                            self.heap[item.x][item.y].exploding = exploding;
                        }
                    }
                }
            }
            Dropping => {
                if self.state.move_timer.update(delta).ready() {
                    self.state.move_timer.reset();

                    if !self
                        .state
                        .update_dropping_at(&mut self.heap, &mut self.active_origins)
                    {
                        self.state.stage = Matching;
                    }
                }
            }
        }

        (self.score, self.blocks_score)
    }

    pub fn topped_up(&self) -> bool {
        self.stable() && self.heap.iter().any(|c| !c[0].empty())
    }

    pub fn stable(&self) -> bool {
        self.state.stage == PitStage::Stable
    }
}

impl Drawable for Pit {
    fn draw(&self, frame: &mut Frame) {
        for (x, cols) in self.heap.iter().enumerate() {
            for (y, block) in cols.iter().enumerate() {
                frame[x + PIT_STARTING_X][y] = block.to_pixel();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::block::BlockKind;

    type Heap = [[Block; 3]; 3];

    mod test_stage_transition {
        use super::*;

        #[test]
        fn test_update_stable_stage() {
            let mut pit = Pit::default();

            assert!(pit.stable());

            let mut col = Column::from([
                Block::default(),
                Block::new(Some(BlockKind::Cyan)),
                Block::new(Some(BlockKind::Cyan)),
            ]);
            for _ in 1..NUM_ROWS {
                col.move_down(&pit.heap);
            }

            pit.update(&mut col, Duration::from_millis(0));

            assert!(pit.stable());
        }

        #[test]
        fn test_update_matching_stage() {
            let mut pit = Pit::default();

            assert!(pit.stable());

            let mut col = Column::from([
                Block::new(Some(BlockKind::Cyan)),
                Block::new(Some(BlockKind::Cyan)),
                Block::new(Some(BlockKind::Cyan)),
            ]);
            for _ in 1..NUM_ROWS {
                col.move_down(&pit.heap);
            }
            pit.update(&mut col, Duration::from_millis(Column::MOVE_MILLIS));

            assert!(!pit.stable());
        }
    }

    mod test_collect_matching {
        use super::*;

        #[test]
        fn test_collect_matching_empty() {
            //
            // ┌─┬─┬─┐
            // │▒│ │ │
            // ├─┼─┤─┤
            // │▒│ │ │
            // ├─┼─┼─┤
            // │ │ │ │
            // └─┴─┴─┘
            //
            let pit_state = PitState::default();
            let mut heap: [[Block; 3]; 3] = Pit::new_heap(None);
            let origins = [point!(0, 0), point!(0, 1)];
            for origin in origins.iter() {
                heap[origin.x][origin.y] = Block::new(Some(BlockKind::Cyan));
            }
            let items = pit_state.collect_matching_at(&heap, &origins, &mut 0);

            assert!(items.is_empty());
        }

        #[test]
        fn test_collect_matching_north_south() {
            // ┌─┬─┬─┐
            // │▒│ │ │
            // ├─┼─┤─┤
            // │▒│ │ │
            // ├─┼─┼─┤
            // │▒│ │ │
            // └─┴─┴─┘
            let assert_items = |pit_state: &PitState, heap: &Heap, origins: &[Point]| {
                let items = pit_state.collect_matching_at(heap, origins, &mut 0);

                assert_eq!(items.len(), 3);

                for origin in origins {
                    assert!(items.contains(origin));
                    assert!(items.contains(&point!(origin.x, (origin.y + 1) % 3)));
                    assert!(items.contains(&point!(origin.x, (origin.y + 2) % 3)));
                }
            };
            let pit_state = PitState::default();
            let mut heap = Pit::new_heap(None);
            let origins = [point!(0, 0), point!(0, 1), point!(0, 2)];
            for origin in origins.iter() {
                heap[origin.x][origin.y] = Block::new(Some(BlockKind::Cyan));
            }

            assert_items(&pit_state, &heap, &origins);
        }

        #[test]
        fn test_collect_matching_east_west() {
            // ┌─┬─┬─┐
            // │ │ │ │
            // ├─┼─┤─┤
            // │▒│▒│▒│
            // ├─┼─┼─┤
            // │ │ │ │
            // └─┴─┴─┘
            let assert_items = |pit_state: &PitState, heap: &Heap, origins: &[Point]| {
                let items = pit_state.collect_matching_at(heap, origins, &mut 0);

                assert_eq!(items.len(), 3);

                for origin in origins {
                    assert!(items.contains(origin));
                    assert!(items.contains(&point!((origin.x + 1) % 3, origin.y)));
                    assert!(items.contains(&point!((origin.x + 2) % 3, origin.y)));
                }
            };
            let pit_state = PitState::default();
            let mut heap = Pit::new_heap(None);
            let origins = [point!(0, 1), point!(1, 1), point!(2, 1)];
            for origin in origins.iter() {
                heap[origin.x][origin.y] = Block::new(Some(BlockKind::Cyan));
            }

            assert_items(&pit_state, &heap, &origins);
        }

        #[test]
        fn test_collect_matching_north_east() {
            // ┌─┬─┬─┐
            // │▒│ │ │
            // ├─┼─┤─┤
            // │ │▒│ │
            // ├─┼─┼─┤
            // │ │ │▒│
            // └─┴─┴─┘
            let assert_items = |pit_state: &PitState, heap: &Heap, origins: &[Point]| {
                let items = pit_state.collect_matching_at(heap, origins, &mut 0);

                assert_eq!(items.len(), 3);

                for origin in origins {
                    assert!(items.contains(origin));
                    assert!(items.contains(&point!((origin.x + 1) % 3, (origin.y + 1) % 3)));
                    assert!(items.contains(&point!((origin.x + 2) % 3, (origin.y + 2) % 3)));
                }
            };
            let pit_state = PitState::default();
            let mut heap = Pit::new_heap(None);
            let origins = [point!(0, 0), point!(1, 1), point!(2, 2)];
            for origin in origins.iter() {
                heap[origin.x][origin.y] = Block::new(Some(BlockKind::Cyan));
            }

            assert_items(&pit_state, &heap, &origins);
        }

        #[test]
        fn test_collect_matching_south_west() {
            //
            // ┌─┬─┬─┐
            // │ │ │▒│
            // ├─┼─┤─┤
            // │ │▒│ │
            // ├─┼─┼─┤
            // │▒│ │ │
            // └─┴─┴─┘
            //
            let assert_items = |pit_state: &PitState, heap: &Heap, origins: &[Point]| {
                let items = pit_state.collect_matching_at(heap, origins, &mut 0);

                assert_eq!(items.len(), 3);

                for origin in origins {
                    assert!(items.contains(origin));
                    assert!(items.contains(&point!((origin.x + 1) % 3, (origin.y + 2) % 3)));
                    assert!(items.contains(&point!((origin.x + 2) % 3, (origin.y + 1) % 3)));
                }
            };
            let pit_state = PitState::default();
            let mut heap = Pit::new_heap(None);
            let origins = [point!(2, 0), point!(1, 1), point!(0, 2)];
            for origin in origins.iter() {
                heap[origin.x][origin.y] = Block::new(Some(BlockKind::Orange));
            }

            assert_items(&pit_state, &heap, &origins);
        }

        #[test]
        fn test_collect_matching_all_directions() {
            #[rustfmt::skip]
            let assert_items_for_matches = |
                pit_state: &PitState,
                heap: &Heap,
                origins: &[Point],
                matches: &[Point; 6]
            | -> Vec<Point> {
                let mut partial_score = 0;
                let items = pit_state.collect_matching_at(heap, origins, &mut partial_score);

                let num_axes = 3;
                assert_eq!(items.len() * PitState::SCORE_MUL * num_axes, partial_score);

                for origin in origins {
                    assert!(items.contains(origin));
                }
                for item in matches {
                    assert!(items.contains(item));
                }

                items
            };

            let pit_state = PitState::default();
            let heap = Pit::new_heap(Some(BlockKind::Cyan));
            let origins = [point!(0, 0), point!(2, 2)];
            let matches = [
                // ┌─┬─┬─┐
                // │▓│▒│▒│
                // ├─┼─┤─┤
                // │▒│▒│░│
                // ├─┼─┼─┤
                // │▒│░│▒│
                // └─┴─┴─┘
                [
                    point!(1, 0),
                    point!(2, 0),
                    point!(0, 1),
                    point!(0, 2),
                    point!(1, 1),
                    point!(2, 2),
                ],
                // ┌─┬─┬─┐
                // │▒│░│▒│
                // ├─┼─┤─┤
                // │░│▒│▒│
                // ├─┼─┼─┤
                // │▒│▒│▓│
                // └─┴─┴─┘
                [
                    point!(0, 0),
                    point!(2, 0),
                    point!(1, 1),
                    point!(2, 2),
                    point!(0, 2),
                    point!(1, 2),
                ],
            ];

            let items = assert_items_for_matches(&pit_state, &heap, &origins[..1], &matches[0]);
            assert_eq!(items.len(), 1 + &matches[0].len());

            let items = assert_items_for_matches(&pit_state, &heap, &origins[1..], &matches[1]);
            assert_eq!(items.len(), 1 + &matches[0].len());

            let items = pit_state.collect_matching_at(&heap, &origins, &mut 0);
            assert_eq!(items.len(), 9);
        }
    }

    mod test_collect_dropping {
        use super::*;

        fn create_and_populate_heap_for_dropping() -> (Heap, [Point; 5], [Point; 2]) {
            let mut heap: [[Block; 3]; 3] = Pit::new_heap(None);
            let origins = [
                point!(0, 0),
                point!(0, 1),
                point!(1, 1),
                point!(2, 1),
                point!(2, 2),
            ];
            let cyan = [point!(1, 0), point!(2, 0)];
            let orange = [point!(0, 2), point!(1, 2)];
            let dropping = [point!(1, 0), point!(2, 0)];

            for origin in cyan {
                heap[origin.x][origin.y].update(Some(BlockKind::Cyan));
            }
            for origin in orange {
                heap[origin.x][origin.y].update(Some(BlockKind::Orange));
            }
            (heap, origins, dropping)
        }

        #[test]
        fn test_collect_dropping_at() {
            // ┌─┬─┬─┐
            // │*│░│░│  ░ = Cyan
            // ├─┼─┤─┤  ▒ = Orange
            // │*│*│*│  * = Empty (previously exploding)
            // ├─┼─┼─┤
            // │▒│▒│*│
            // └─┴─┴─┘
            let (heap, origins, dropping) = create_and_populate_heap_for_dropping();
            let items = PitState::default().collect_dropping_at(&heap, &origins);

            for item in dropping {
                assert!(items.contains(&item));
            }
        }

        #[test]
        fn test_update_dropping_at() {
            // ┌─┬─┬─┐
            // │*│░│░│  ░ = Cyan
            // ├─┼─┤─┤  ▒ = Orange
            // │*│*│*│  * = Empty (previously exploding)
            // ├─┼─┼─┤
            // │▒│▒│*│
            // └─┴─┴─┘
            let (mut heap, _, mut dropping) = create_and_populate_heap_for_dropping();
            let pit_state = PitState::default();
            let mut drop_times = 0;

            loop {
                if !pit_state.update_dropping_at(&mut heap, &mut dropping) {
                    break;
                }
                drop_times += 1;
            }

            assert_eq!(drop_times, 2);
            // ┌─┬─┬─┐
            // │ │ │ │  ░ = Cyan
            // ├─┼─┤─┤  ▒ = Orange
            // │ │░│ │
            // ├─┼─┼─┤
            // │▒│▒│░│
            // └─┴─┴─┘
            // empty blocks
            assert!(&heap[0][0].empty());
            assert!(&heap[1][0].empty());
            assert!(&heap[2][0].empty());
            assert!(&heap[0][1].empty());
            assert!(&heap[2][1].empty());
            // non-empty blocks
            assert!(!&heap[1][1].empty());
            assert!(!&heap[0][2].empty());
            assert!(!&heap[1][2].empty());
            assert!(!&heap[2][2].empty());
        }
    }
}
