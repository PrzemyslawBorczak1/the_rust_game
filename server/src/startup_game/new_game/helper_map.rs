use anyhow::Result;
use image::{Rgb, RgbImage};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::VecDeque;

const UNASSIGNED: u32 = u32::MAX;

pub fn generate_province_map_png(
    width: u32,
    height: u32,
    provinces: u32,
    seed: u64,
    path: &str,
) -> Result<(Vec<Vec<usize>>, Vec<Vec<usize>>)> {
    let w = width as usize;
    let h = height as usize;

    let mut rng = StdRng::seed_from_u64(seed);

    let red = generate_red_palette(provinces, &mut rng);
    let mut grid = vec![UNASSIGNED; w * h];

    let mut q = VecDeque::new();
    seed_provinces(&mut grid, w, h, provinces, &mut rng, &mut q);
    flood_fill(&mut grid, w, h, &mut rng, &mut q);

    let adjacency = build_adjacency(&grid, w, h, provinces);

    let img = render_image(width, height, w, h, &grid, &red);
    img.save("assets\\".to_string() + path)?;

    let grid_ids = grid_to_2d_usize(&grid, w, h);

    Ok((grid_ids, adjacency))
}

fn generate_red_palette(provinces: u32, rng: &mut StdRng) -> Vec<u8> {
    let mut red = vec![0u8; provinces as usize];
    for r in &mut red {
        *r = rng.random();
    }
    red
}

fn seed_provinces(
    grid: &mut [u32],
    w: usize,
    h: usize,
    provinces: u32,
    rng: &mut StdRng,
    q: &mut VecDeque<(usize, usize)>,
) {
    for id in 0..provinces {
        loop {
            let x = rng.random_range(0..w);
            let y = rng.random_range(0..h);
            let idx = y * w + x;
            if grid[idx] == UNASSIGNED {
                grid[idx] = id;
                q.push_back((x, y));
                break;
            }
        }
    }
}

fn flood_fill(
    grid: &mut [u32],
    w: usize,
    h: usize,
    rng: &mut StdRng,
    q: &mut VecDeque<(usize, usize)>,
) {
    while let Some((x, y)) = q.pop_front() {
        let id = grid[y * w + x];

        let mut dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        shuffle_dirs(&mut dirs, rng);

        for (dx, dy) in dirs {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 || nx >= w as i32 || ny >= h as i32 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            let nidx = ny * w + nx;

            if grid[nidx] == UNASSIGNED {
                grid[nidx] = id;
                q.push_back((nx, ny));
            }
        }
    }
}

fn shuffle_dirs(dirs: &mut [(i32, i32)], rng: &mut StdRng) {
    for i in (1..dirs.len()).rev() {
        let j = rng.random_range(0..=i);
        dirs.swap(i, j);
    }
}

fn render_image(width: u32, height: u32, w: usize, h: usize, grid: &[u32], red: &[u8]) -> RgbImage {
    let mut img = RgbImage::new(width, height);
    for y in 0..h {
        for x in 0..w {
            let id = grid[y * w + x];
            if id != 0 {
                //println!("{id}");
            }

            let g = ((id >> 8) & 0xFF) as u8;
            let b = ((id) & 0xFF) as u8;
            let r = red[id as usize];
            img.put_pixel(x as u32, y as u32, Rgb([r, g, b]));
        }
    }
    img
}

fn grid_to_2d_usize(grid: &[u32], w: usize, h: usize) -> Vec<Vec<usize>> {
    let mut out = vec![vec![0usize; w]; h];
    for y in 0..h {
        for x in 0..w {
            out[y][x] = grid[y * w + x] as usize;
        }
    }
    out
}

fn build_adjacency(grid: &[u32], w: usize, h: usize, provinces: u32) -> Vec<Vec<usize>> {
    let p = provinces as usize;
    let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); p];

    let mut seen: Vec<u32> = vec![0; p];
    let mut stamp: u32 = 1;

    for y in 0..h {
        for x in 0..w {
            let a = grid[y * w + x] as usize;

            if x + 1 < w {
                let b = grid[y * w + (x + 1)] as usize;
                if a != b {
                    add_edge(&mut adjacency, &mut seen, &mut stamp, a, b);
                    add_edge(&mut adjacency, &mut seen, &mut stamp, b, a);
                }
            }
            if y + 1 < h {
                let b = grid[(y + 1) * w + x] as usize;
                if a != b {
                    add_edge(&mut adjacency, &mut seen, &mut stamp, a, b);
                    add_edge(&mut adjacency, &mut seen, &mut stamp, b, a);
                }
            }
        }
    }

    adjacency
}

fn add_edge(adjacency: &mut [Vec<usize>], seen: &mut [u32], stamp: &mut u32, a: usize, b: usize) {
    if !adjacency[a].iter().any(|&x| x == b) {
        adjacency[a].push(b);
    }

    let _ = (seen, stamp);
}
