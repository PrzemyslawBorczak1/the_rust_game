use std::collections::VecDeque;
use std::fs;
use std::time::Duration;

use anyhow::{Result, anyhow};
use bevy::platform::thread;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use shared::resources::{NO_OWNER, Province, TERRAIN_FLAT, TERRAIN_WATER};

pub fn generate_province(
    amount: u32,
    seed: u64,
    adjaceny: Vec<Vec<usize>>,
    path: &str,
    countries: u32,
) -> Result<()> {
    println!("\n\n\nret{}\n\n", adjaceny.len());

    let n = amount as usize;

    if adjaceny.len() != n {
        return Err(anyhow!(
            "adjaceny length mismatch: expected {}, got {}",
            n,
            adjaceny.len()
        ));
    }

    for (i, neigh) in adjaceny.iter().enumerate() {
        for &v in neigh {
            if v >= n {
                return Err(anyhow!(
                    "adjaceny[{}] contains out-of-range neighbor {} (n={})",
                    i,
                    v,
                    n
                ));
            }
        }
    }

    let mut ret = Vec::with_capacity(n);
    let mut rng = StdRng::seed_from_u64(seed);

    for _ in 0..n {
        ret.push(Province {
            army: 0,
            owner_id: NO_OWNER,
            terrain_type: TERRAIN_FLAT,
            gold_per_second: rng.random_range(6..12),
            level: 1,
        });
    }

    paint_terrain_blobs(&mut ret, n, &adjaceny, &mut rng);

    let capitals = choose_capitals_farthest_graph(n, countries as usize, &adjaceny, &mut rng);

    for (cid, &cap_idx) in capitals.iter().enumerate() {
        ret[cap_idx].owner_id = cid as u32;
    }

    grow_countries_bfs_capped(
        &mut ret,
        &adjaceny,
        n,
        countries as usize,
        &capitals,
        &mut rng,
    );

    let json = serde_json::to_string_pretty(&ret)?;
    fs::write("assets\\".to_string() + path, json)?;
    Ok(())
}

fn paint_terrain_blobs(ret: &mut [Province], n: usize, adj: &[Vec<usize>], rng: &mut StdRng) {
    let div = 4usize;

    for _ in 0..(n / div).max(1) {
        let terrain = rng.random_range(1..4) as u32;
        let mut idx = rng.random_range(0..n);

        for _ in 0..div {
            ret[idx].terrain_type = terrain;
            if terrain == TERRAIN_WATER {
                ret[idx].gold_per_second = 0;
            } else {
                // avoid divide-by-zero; terrain is in [1..3]
                ret[idx].gold_per_second /= terrain;
            }

            if adj[idx].is_empty() {
                break;
            }
            let next = adj[idx][rng.random_range(0..adj[idx].len())];
            idx = next;
        }
    }
}

/// Pick capitals "far apart" using graph distance (BFS on adjacency).
/// Works for any topology (not just grids).
fn choose_capitals_farthest_graph(
    n: usize,
    countries: usize,
    adj: &[Vec<usize>],
    rng: &mut StdRng,
) -> Vec<usize> {
    let mut caps = Vec::with_capacity(countries);
    if countries == 0 || n == 0 {
        return caps;
    }

    // Start anywhere
    caps.push(rng.random_range(0..n));

    // Heuristic: sample a bunch of random candidates and keep the one that maximizes
    // the minimum distance to existing caps.
    let samples_per_cap = 64usize;

    while caps.len() < countries {
        let mut best = rng.random_range(0..n);
        let mut best_score = -1i32;

        for _ in 0..samples_per_cap {
            let cand = rng.random_range(0..n);

            // score = min distance from cand to any existing cap
            let mut min_dist = i32::MAX;
            for &c in &caps {
                let d = bfs_distance_limited(adj, cand, c, min_dist);
                if d < min_dist {
                    min_dist = d;
                }
                if min_dist == 0 {
                    break;
                }
            }

            if min_dist > best_score {
                best_score = min_dist;
                best = cand;
            }
        }

        if !caps.contains(&best) {
            caps.push(best);
        } else {
            // fallback: find any province not yet chosen
            if let Some(x) = (0..n).find(|i| !caps.contains(i)) {
                caps.push(x);
            } else {
                break;
            }
        }
    }

    caps
}

/// BFS shortest-path distance between `src` and `dst`.
/// `early_cutoff`: if we already have a min distance this small, we can stop exploring deeper.
fn bfs_distance_limited(adj: &[Vec<usize>], src: usize, dst: usize, early_cutoff: i32) -> i32 {
    if src == dst {
        return 0;
    }

    let n = adj.len();
    let mut dist = vec![-1i32; n];
    let mut q = VecDeque::new();

    dist[src] = 0;
    q.push_back(src);

    while let Some(u) = q.pop_front() {
        let du = dist[u];
        if du + 1 >= early_cutoff {
            // no need to search deeper if we can't beat current best
            continue;
        }
        for &v in &adj[u] {
            if dist[v] != -1 {
                continue;
            }
            dist[v] = du + 1;
            if v == dst {
                return dist[v];
            }
            q.push_back(v);
        }
    }

    // disconnected
    i32::MAX / 4
}

fn grow_countries_bfs_capped(
    ret: &mut [Province],
    adj: &[Vec<usize>],
    amount: usize,
    countries: usize,
    capitals: &[usize],
    rng: &mut StdRng,
) {
    if countries == 0 || amount == 0 {
        return;
    }

    let target = amount / (4 * countries);
    if target == 0 {
        return;
    }

    let mut owned = vec![0usize; countries];
    let mut q: VecDeque<(usize, u32)> = VecDeque::new();

    for (cid, &cap) in capitals.iter().enumerate() {
        q.push_back((cap, cid as u32));
        owned[cid] = owned[cid].saturating_add(1);
    }

    while let Some((idx, cid_u32)) = q.pop_front() {
        let cid = cid_u32 as usize;

        if owned[cid] >= target {
            continue;
        }
        if adj[idx].is_empty() {
            continue;
        }

        // Randomize neighbor order (no fixed 4 limit; uses Vec)
        let mut neigh = adj[idx].clone();
        for i in (1..neigh.len()).rev() {
            let j = rng.random_range(0..=i);
            neigh.swap(i, j);
        }

        for &nidx in &neigh {
            if owned[cid] >= target {
                break;
            }
            if ret[nidx].owner_id == NO_OWNER {
                ret[nidx].owner_id = cid_u32;
                owned[cid] += 1;
                q.push_back((nidx, cid_u32));
            }
        }
    }
}
