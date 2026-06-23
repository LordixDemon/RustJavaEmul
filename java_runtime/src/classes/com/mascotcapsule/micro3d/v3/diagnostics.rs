use alloc::{
    format,
    string::{String as RustString, ToString},
    vec::Vec,
};
use core::sync::atomic::{AtomicU64, Ordering};

use parking_lot::Mutex;

use crate::classes::java::lang::{System, Thread};

const V3_RENDER_DIAGNOSTIC_LIMIT: usize = 16;

static V3_RENDER_FRAME: AtomicU64 = AtomicU64::new(1);
static V3_RENDER_LAST_END_MS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_DIAG_MS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_FPS_WINDOW_START_MS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_FPS_WINDOW_FRAMES: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_FIGURE_CACHE_HITS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_FIGURE_CACHE_MISSES: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_TEXTURE_CACHE_HITS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_TEXTURE_CACHE_MISSES: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_SLEEP_CALLS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_SLEEP_TOTAL_MS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_SLEEP_ZERO_CALLS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_SLEEP_SHORT_CALLS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_SLEEP_FRAME_CALLS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_SLEEP_LONG_CALLS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_ACTIVE_SAMPLES: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_ACTIVE_TOTAL_MS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_ACTIVE_LE_4MS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_ACTIVE_5_10MS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_ACTIVE_11_20MS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_ACTIVE_21_40MS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_ACTIVE_OVER_40MS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_GC_CALLS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_GC_SKIPPED: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_GC_RAN: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_GC_COLLECTED: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_GAP_LE_20MS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_GAP_21_34MS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_GAP_35_50MS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_GAP_OVER_50MS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_GAP_LE_20MS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_GAP_21_34MS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_GAP_35_50MS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_LAST_GAP_OVER_50MS: AtomicU64 = AtomicU64::new(0);
static V3_RENDER_DIAGNOSTICS: Mutex<Vec<RustString>> = Mutex::new(Vec::new());

#[derive(Clone, Copy, Default)]
pub(super) struct V3FrameDrawStats {
    pub(super) setup_ms: u64,
    pub(super) raster_ms: u64,
    pub(super) upload_ms: u64,
    pub(super) store_ms: u64,
    pub(super) draw_calls: usize,
    pub(super) rasterized_triangles: usize,
    pub(super) submitted_pixels: usize,
    pub(super) batched: bool,
    pub(super) target_image: bool,
    pub(super) gpu_direct: bool,
    pub(super) gpu_reject: Option<&'static str>,
}

pub(super) struct V3FrameStats {
    pub(super) total_ms: u64,
    pub(super) queue_ms: u64,
    pub(super) merge_ms: u64,
    pub(super) sort_ms: u64,
    pub(super) draw_ms: u64,
    pub(super) prepare_ms: u64,
    pub(super) figure_prepare_ms: u64,
    pub(super) primitive_prepare_ms: u64,
    pub(super) queued: usize,
    pub(super) figure_count: usize,
    pub(super) primitive_count: usize,
    pub(super) textures: usize,
    pub(super) triangles: usize,
    pub(super) figure_cache_hits: u64,
    pub(super) figure_cache_misses: u64,
    pub(super) texture_cache_hits: u64,
    pub(super) texture_cache_misses: u64,
    pub(super) draw: V3FrameDrawStats,
}

pub fn latest_render_diagnostics() -> RustString {
    let diagnostics = V3_RENDER_DIAGNOSTICS.lock();
    if diagnostics.is_empty() {
        return "v3: waiting for render".to_string();
    }

    let mut output = RustString::new();
    for line in diagnostics.iter() {
        if !output.is_empty() {
            output.push('\n');
        }
        output.push_str(line);
    }
    output
}

pub(super) fn publish_frame_diagnostic(now_ms: u64, stats: V3FrameStats) {
    let frame = V3_RENDER_FRAME.fetch_add(1, Ordering::Relaxed);
    let previous_end_ms = V3_RENDER_LAST_END_MS.swap(now_ms, Ordering::Relaxed);
    let gap_ms = if previous_end_ms == 0 {
        0
    } else {
        now_ms.saturating_sub(previous_end_ms)
    };
    record_gap_bucket(gap_ms);
    let fps = update_fps(now_ms);
    let slow = stats.total_ms >= 17 || gap_ms >= 34;
    let previous_diag_ms = V3_RENDER_LAST_DIAG_MS.load(Ordering::Relaxed);
    if !slow && now_ms.saturating_sub(previous_diag_ms) < 250 {
        return;
    }
    V3_RENDER_LAST_DIAG_MS.store(now_ms, Ordering::Relaxed);

    let (hot_name, hot_ms) = hottest_stage(&stats);
    let figure_cache_hits = delta_counter(&V3_RENDER_LAST_FIGURE_CACHE_HITS, stats.figure_cache_hits);
    let figure_cache_misses = delta_counter(&V3_RENDER_LAST_FIGURE_CACHE_MISSES, stats.figure_cache_misses);
    let texture_cache_hits = delta_counter(&V3_RENDER_LAST_TEXTURE_CACHE_HITS, stats.texture_cache_hits);
    let texture_cache_misses = delta_counter(&V3_RENDER_LAST_TEXTURE_CACHE_MISSES, stats.texture_cache_misses);
    let sleep_stats = Thread::sleep_stats();
    let sleep_calls = delta_counter(&V3_RENDER_LAST_SLEEP_CALLS, sleep_stats.calls);
    let sleep_total_ms = delta_counter(&V3_RENDER_LAST_SLEEP_TOTAL_MS, sleep_stats.total_ms);
    let sleep_zero_calls = delta_counter(&V3_RENDER_LAST_SLEEP_ZERO_CALLS, sleep_stats.zero_calls);
    let sleep_short_calls = delta_counter(&V3_RENDER_LAST_SLEEP_SHORT_CALLS, sleep_stats.short_calls);
    let sleep_frame_calls = delta_counter(&V3_RENDER_LAST_SLEEP_FRAME_CALLS, sleep_stats.frame_calls);
    let sleep_long_calls = delta_counter(&V3_RENDER_LAST_SLEEP_LONG_CALLS, sleep_stats.long_calls);
    let active_samples = delta_counter(&V3_RENDER_LAST_ACTIVE_SAMPLES, sleep_stats.active_samples);
    let active_total_ms = delta_counter(&V3_RENDER_LAST_ACTIVE_TOTAL_MS, sleep_stats.active_total_ms);
    let active_avg_ms = if active_samples == 0 {
        0.0
    } else {
        active_total_ms as f64 / active_samples as f64
    };
    let active_le_4 = delta_counter(&V3_RENDER_LAST_ACTIVE_LE_4MS, sleep_stats.active_le_4ms);
    let active_5_10 = delta_counter(&V3_RENDER_LAST_ACTIVE_5_10MS, sleep_stats.active_5_10ms);
    let active_11_20 = delta_counter(&V3_RENDER_LAST_ACTIVE_11_20MS, sleep_stats.active_11_20ms);
    let active_21_40 = delta_counter(&V3_RENDER_LAST_ACTIVE_21_40MS, sleep_stats.active_21_40ms);
    let active_over_40 = delta_counter(&V3_RENDER_LAST_ACTIVE_OVER_40MS, sleep_stats.active_over_40ms);
    let active_trace = Thread::active_last_trace();
    let gc_stats = System::explicit_gc_stats();
    let gc_calls = delta_counter(&V3_RENDER_LAST_GC_CALLS, gc_stats.calls);
    let gc_skipped = delta_counter(&V3_RENDER_LAST_GC_SKIPPED, gc_stats.skipped);
    let gc_ran = delta_counter(&V3_RENDER_LAST_GC_RAN, gc_stats.ran);
    let gc_collected = delta_counter(&V3_RENDER_LAST_GC_COLLECTED, gc_stats.collected);
    let gap_le_20 = delta_counter(&V3_RENDER_LAST_GAP_LE_20MS, V3_RENDER_GAP_LE_20MS.load(Ordering::Relaxed));
    let gap_21_34 = delta_counter(&V3_RENDER_LAST_GAP_21_34MS, V3_RENDER_GAP_21_34MS.load(Ordering::Relaxed));
    let gap_35_50 = delta_counter(&V3_RENDER_LAST_GAP_35_50MS, V3_RENDER_GAP_35_50MS.load(Ordering::Relaxed));
    let gap_over_50 = delta_counter(&V3_RENDER_LAST_GAP_OVER_50MS, V3_RENDER_GAP_OVER_50MS.load(Ordering::Relaxed));
    let line = format!(
        "v3 f={frame} fps={fps:.1} gap={gap_ms}ms gapBuckets <=20/21-34/35-50/>50={gap_le_20}/{gap_21_34}/{gap_35_50}/{gap_over_50} total={}ms hot={hot_name}:{}ms | prep={}ms(fig={} prim={}) queue={} merge={} sort={} draw={}ms(setup={} rast={} upload={} store={}) | q={} fig={} prim={} tex={} tri={} rastTri={} px={} calls={} batch={} target={} gpu={} gpuWhy={} | cache fig={}/{} tex={}/{} | gc calls={} skipped={} ran={} collected={} | sleep calls={} total={}ms last={}ms max={}ms buckets 0/1-5/6-20/>20={}/{}/{}/{} | active samples={} avg={active_avg_ms:.1}ms last={}ms max={}ms buckets <=4/5-10/11-20/21-40/>40={active_le_4}/{active_5_10}/{active_11_20}/{active_21_40}/{active_over_40} activeTrace={active_trace}",
        stats.total_ms,
        hot_ms,
        stats.prepare_ms,
        stats.figure_prepare_ms,
        stats.primitive_prepare_ms,
        stats.queue_ms,
        stats.merge_ms,
        stats.sort_ms,
        stats.draw_ms,
        stats.draw.setup_ms,
        stats.draw.raster_ms,
        stats.draw.upload_ms,
        stats.draw.store_ms,
        stats.queued,
        stats.figure_count,
        stats.primitive_count,
        stats.textures,
        stats.triangles,
        stats.draw.rasterized_triangles,
        stats.draw.submitted_pixels,
        stats.draw.draw_calls,
        stats.draw.batched,
        stats.draw.target_image,
        stats.draw.gpu_direct,
        stats.draw.gpu_reject.unwrap_or("ok"),
        figure_cache_hits,
        figure_cache_misses,
        texture_cache_hits,
        texture_cache_misses,
        gc_calls,
        gc_skipped,
        gc_ran,
        gc_collected,
        sleep_calls,
        sleep_total_ms,
        sleep_stats.last_ms,
        sleep_stats.max_ms,
        sleep_zero_calls,
        sleep_short_calls,
        sleep_frame_calls,
        sleep_long_calls,
        active_samples,
        sleep_stats.active_last_ms,
        sleep_stats.active_max_ms
    );

    {
        let mut diagnostics = V3_RENDER_DIAGNOSTICS.lock();
        if diagnostics.len() >= V3_RENDER_DIAGNOSTIC_LIMIT {
            diagnostics.remove(0);
        }
        diagnostics.push(line.clone());
    }

    if slow {
        tracing::warn!(target: "rustjava_v3", "{line}");
    } else {
        tracing::info!(target: "rustjava_v3", "{line}");
    }
}

fn delta_counter(last: &AtomicU64, current: u64) -> u64 {
    let previous = last.swap(current, Ordering::Relaxed);
    current.saturating_sub(previous)
}

fn record_gap_bucket(gap_ms: u64) {
    let bucket = if gap_ms <= 20 {
        &V3_RENDER_GAP_LE_20MS
    } else if gap_ms <= 34 {
        &V3_RENDER_GAP_21_34MS
    } else if gap_ms <= 50 {
        &V3_RENDER_GAP_35_50MS
    } else {
        &V3_RENDER_GAP_OVER_50MS
    };
    bucket.fetch_add(1, Ordering::Relaxed);
}

fn update_fps(now_ms: u64) -> f64 {
    let mut window_start = V3_RENDER_FPS_WINDOW_START_MS.load(Ordering::Relaxed);
    if window_start == 0 {
        V3_RENDER_FPS_WINDOW_START_MS.store(now_ms, Ordering::Relaxed);
        window_start = now_ms;
    }

    let frames = V3_RENDER_FPS_WINDOW_FRAMES.fetch_add(1, Ordering::Relaxed) + 1;
    let elapsed_ms = now_ms.saturating_sub(window_start);
    if elapsed_ms == 0 {
        return 0.0;
    }

    let fps = frames as f64 * 1000.0 / elapsed_ms as f64;
    if elapsed_ms >= 1000 {
        V3_RENDER_FPS_WINDOW_START_MS.store(now_ms, Ordering::Relaxed);
        V3_RENDER_FPS_WINDOW_FRAMES.store(0, Ordering::Relaxed);
    }
    fps
}

fn hottest_stage(stats: &V3FrameStats) -> (&'static str, u64) {
    [
        ("prepare", stats.prepare_ms),
        ("queue", stats.queue_ms),
        ("merge", stats.merge_ms),
        ("sort", stats.sort_ms),
        ("draw", stats.draw_ms),
        ("raster", stats.draw.raster_ms),
        ("upload", stats.draw.upload_ms),
        ("store", stats.draw.store_ms),
    ]
    .into_iter()
    .max_by_key(|(_, ms)| *ms)
    .unwrap_or(("none", 0))
}
