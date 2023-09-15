use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use ratatui_textarea::{CursorMove, Input, Key, TextArea};
use ratatui_textarea_bench::{dummy_terminal, TerminalExt, LOREM, SEED};

#[inline]
fn append_lorem(repeat: usize) -> usize {
    let mut textarea = TextArea::default();
    let mut term = dummy_terminal();
    for _ in 0..repeat {
        for line in LOREM {
            for c in line.chars() {
                textarea.input(Input {
                    key: Key::Char(c),
                    ctrl: false,
                    alt: false,
                });
                term.draw_textarea(&textarea);
            }
        }
        textarea.input(Input {
            key: Key::Enter,
            ctrl: false,
            alt: false,
        });
        term.draw_textarea(&textarea);
    }
    textarea.lines().len()
}

#[inline]
fn random_lorem(repeat: usize) -> usize {
    let mut rng = SmallRng::from_seed(SEED);
    let mut textarea = TextArea::default();
    let mut term = dummy_terminal();

    for _ in 0..repeat {
        for line in LOREM {
            let row = rng.gen_range(0..textarea.lines().len() as u16);
            textarea.move_cursor(CursorMove::Jump(row, 0));
            textarea.move_cursor(CursorMove::End);

            textarea.input(Input {
                key: Key::Enter,
                ctrl: false,
                alt: false,
            });
            term.draw_textarea(&textarea);

            for c in line.chars() {
                textarea.input(Input {
                    key: Key::Char(c),
                    ctrl: false,
                    alt: false,
                });
                term.draw_textarea(&textarea);
            }
        }
    }

    textarea.lines().len()
}

fn append(c: &mut Criterion) {
    c.bench_function("insert::append::1_lorem", |b| {
        b.iter(|| black_box(append_lorem(1)))
    });
    c.bench_function("insert::append::10_lorem", |b| {
        b.iter(|| black_box(append_lorem(10)))
    });
    c.bench_function("insert::append::50_lorem", |b| {
        b.iter(|| black_box(append_lorem(50)))
    });
}

fn random(c: &mut Criterion) {
    c.bench_function("insert::random::1_lorem", |b| {
        b.iter(|| black_box(random_lorem(1)))
    });
    c.bench_function("insert::random::10_lorem", |b| {
        b.iter(|| black_box(random_lorem(10)))
    });
    c.bench_function("insert::random::50_lorem", |b| {
        b.iter(|| black_box(random_lorem(50)))
    });
}

criterion_group!(insert, append, random);
criterion_main!(insert);
