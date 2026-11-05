<template>
  <div class="bg-[#131b2e] border border-slate-800 rounded-xl p-5 shadow-lg">
    <div class="flex items-center justify-between mb-4">
      <div>
        <h3 class="text-base font-bold text-white flex items-center gap-2">
          <span class="w-2.5 h-2.5 rounded-full bg-amber-400 animate-pulse"></span>
          Bankroll Equity Curve
        </h3>
        <p class="text-xs text-slate-400">Cumulative performance & drawdown trajectory</p>
      </div>
      <div class="flex items-center gap-3 text-xs font-mono">
        <span class="text-slate-400">Peak: <strong class="text-emerald-400">€{{ peak.toFixed(2) }}</strong></span>
        <span class="text-slate-400">Current: <strong class="text-amber-400">€{{ current.toFixed(2) }}</strong></span>
      </div>
    </div>

    <!-- SVG Area Chart -->
    <div class="w-full h-56 relative">
      <svg class="w-full h-full overflow-visible" :viewBox="`0 0 ${width} ${height}`" preserveAspectRatio="none">
        <defs>
          <linearGradient id="equityGrad" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stop-color="#f59e0b" stop-opacity="0.35" />
            <stop offset="100%" stop-color="#f59e0b" stop-opacity="0.0" />
          </linearGradient>
        </defs>

        <!-- Baseline Initial Line -->
        <line
          :x1="0"
          :y1="baseY"
          :x2="width"
          :y2="baseY"
          stroke="#334155"
          stroke-dasharray="4 4"
          stroke-width="1"
        />

        <!-- Area Fill -->
        <polygon :points="areaPoints" fill="url(#equityGrad)" />

        <!-- Line Stroke -->
        <polyline
          :points="linePoints"
          fill="none"
          stroke="#f59e0b"
          stroke-width="2.5"
          stroke-linecap="round"
          stroke-linejoin="round"
        />

        <!-- Data points -->
        <circle
          v-for="(p, i) in points"
          :key="i"
          :cx="p.x"
          :cy="p.y"
          r="4"
          class="fill-amber-400 stroke-[#131b2e] stroke-2 hover:r-6 transition-all cursor-pointer"
        />
      </svg>
    </div>

    <div class="mt-3 flex items-center justify-between text-[11px] text-slate-500 font-mono">
      <span>Baseline: €1,000.00</span>
      <span>Live Simulation Feed</span>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  history: {
    type: Array,
    default: () => [
      { balance: 1000 },
      { balance: 1025 },
      { balance: 1010 },
      { balance: 1065 },
      { balance: 1045 },
      { balance: 1110 },
      { balance: 1148.5 },
    ],
  },
});

const width = 600;
const height = 200;
const padding = 20;

const balances = computed(() => {
  if (!props.history || props.history.length === 0) return [1000, 1000];
  return props.history.map((h) => h.balance || 1000);
});

const peak = computed(() => Math.max(...balances.value, 1000));
const min = computed(() => Math.min(...balances.value, 950));
const current = computed(() => balances.value[balances.value.length - 1] || 1000);

const scaleX = (idx, total) => {
  if (total <= 1) return padding;
  return padding + (idx / (total - 1)) * (width - 2 * padding);
};

const scaleY = (val) => {
  const range = peak.value - min.value || 1;
  const normalized = (val - min.value) / range;
  return height - padding - normalized * (height - 2 * padding);
};

const baseY = computed(() => scaleY(1000));

const points = computed(() => {
  return balances.value.map((val, idx) => ({
    x: scaleX(idx, balances.value.length),
    y: scaleY(val),
    val,
  }));
});

const linePoints = computed(() => {
  return points.value.map((p) => `${p.x},${p.y}`).join(' ');
});

const areaPoints = computed(() => {
  if (points.value.length === 0) return '';
  const first = points.value[0];
  const last = points.value[points.value.length - 1];
  return `${first.x},${height} ${linePoints.value} ${last.x},${height}`;
});
</script>

