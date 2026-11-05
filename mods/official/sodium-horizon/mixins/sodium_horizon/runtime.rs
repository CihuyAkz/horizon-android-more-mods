// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Sodium Horizon experimental contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
// Independently authored Rust implementation; contains no Sodium source.
// Rows are indexed y * 16 + z, with bit x denoting an open cell.
pub const ALL_VISIBLE: u64 = (1u64 << 36) - 1;

#[inline]
fn span(seed: u16, open: u16) -> u16 {
    let (mut left, mut right) = (seed, seed);
    let (mut lmask, mut rmask) = (open, open);
    for shift in [1, 2, 4, 8] {
        left |= lmask & (left << shift);
        right |= rmask & (right >> shift);
        lmask &= lmask << shift;
        rmask &= rmask >> shift;
    }
    left | right
}

// Only boundary-connected regions affect the face-connectivity matrix.
// Consume each discovered run once; use the mesher worker's existing stack.
pub fn resolve(open: &[u16; 256], stack: &mut Vec<usize>) -> u64 {
    stack.clear();
    if open.iter().all(|&row| row == u16::MAX) {
        return ALL_VISIBLE;
    }
    let mut remaining = *open;
    let mut result = 0u64;
    for start in 0..256 {
        let boundary = if start < 16 || start >= 240 || start & 15 == 0 || start & 15 == 15 {
            u16::MAX
        } else {
            0x8001
        };
        while remaining[start] & boundary != 0 {
            let seeds = remaining[start] & boundary;
            stack.push((start << 16) | (seeds & seeds.wrapping_neg()) as usize);
            let mut faces = 0u8;
            while let Some(entry) = stack.pop() {
                let row = entry >> 16;
                let seed = entry as u16 & remaining[row];
                if seed == 0 {
                    continue;
                }
                let run = span(seed, remaining[row]);
                remaining[row] &= !run;
                let (y, z) = (row >> 4, row & 15);
                faces |= u8::from(y == 0)
                    | (u8::from(y == 15) << 1)
                    | (u8::from(z == 0) << 2)
                    | (u8::from(z == 15) << 3)
                    | (u8::from(run & 1 != 0) << 4)
                    | (u8::from(run & 0x8000 != 0) << 5);
                if faces == 63 {
                    stack.clear();
                    return ALL_VISIBLE;
                }
                for next in [
                    row.checked_sub(16),
                    (row < 240).then_some(row + 16),
                    if z > 0 { Some(row - 1) } else { None },
                    if z < 15 { Some(row + 1) } else { None },
                ]
                .into_iter()
                .flatten()
                {
                    let seed = run & remaining[next];
                    if seed != 0 {
                        stack.push((next << 16) | seed as usize);
                    }
                }
            }
            for a in 0..6 {
                if faces & (1 << a) != 0 {
                    result |= (faces as u64) << (a * 6);
                }
            }
            if result == ALL_VISIBLE {
                return result;
            }
        }
    }
    result
}
