<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-sm">
    <div class="bg-[#131b2e] border border-slate-700 w-full max-w-lg rounded-2xl p-6 shadow-2xl relative">
      <div class="flex items-center justify-between pb-4 border-b border-slate-800">
        <div>
          <h3 class="text-lg font-bold text-white flex items-center gap-2">
            <span>Risk Manager & Quantitative Parameters</span>
          </h3>
          <p class="text-xs text-slate-400">Control capital preservation and staking sizing</p>
        </div>
        <button @click="$emit('close')" class="text-slate-400 hover:text-white p-1 rounded-lg">
          ✕
        </button>
      </div>

      <div class="space-y-4 py-4 text-xs">
        <!-- Kelly Fraction -->
        <div>
          <div class="flex justify-between text-slate-300 font-medium mb-1">
            <span>Kelly Criterion Multiplier (Fractional Kelly)</span>
            <span class="font-mono text-amber-400 font-bold">{{ (kellyFraction * 100).toFixed(0) }}% ({{ kellyLabel }})</span>
          </div>
          <input
            type="range"
            min="0.05"
            max="0.50"
            step="0.05"
            v-model.number="kellyFraction"
            class="w-full accent-amber-500 cursor-pointer"
          />
          <p class="text-[11px] text-slate-500 mt-0.5">
            Fractional Kelly scales the mathematical Kelly stake to mitigate bankroll variance.
          </p>
        </div>

        <!-- Minimum EV -->
        <div>
          <div class="flex justify-between text-slate-300 font-medium mb-1">
            <span>Minimum Value Expectation (+EV Cutoff)</span>
            <span class="font-mono text-emerald-400 font-bold">+{{ (minEv * 100).toFixed(1) }}%</span>
          </div>
          <input
            type="range"
            min="0.01"
            max="0.10"
            step="0.005"
            v-model.number="minEv"
            class="w-full accent-emerald-500 cursor-pointer"
          />
          <p class="text-[11px] text-slate-500 mt-0.5">
            Only bets where model probability exceeds bookmaker implied probability by this threshold are flagged.
          </p>
        </div>

        <!-- Max Bet Size -->
        <div>
          <div class="flex justify-between text-slate-300 font-medium mb-1">
            <span>Max Single Bet Limit</span>
            <span class="font-mono text-cyan-400 font-bold">{{ (maxBetPct * 100).toFixed(1) }}% of bankroll</span>
          </div>
          <input
            type="range"
            min="0.01"
            max="0.10"
            step="0.01"
            v-model.number="maxBetPct"
            class="w-full accent-cyan-500 cursor-pointer"
          />
        </div>

        <!-- Stop Loss -->
        <div>
          <div class="flex justify-between text-slate-300 font-medium mb-1">
            <span>Circuit Breaker Drawdown Stop-Loss</span>
            <span class="font-mono text-rose-400 font-bold">25.0%</span>
          </div>
          <div class="p-2.5 rounded-lg bg-rose-500/10 border border-rose-500/20 text-rose-300 text-[11px]">
            Automatic safety halt: stops automated execution if portfolio peak-to-trough decline breaches 25%.
          </div>
        </div>
      </div>

      <div class="pt-4 border-t border-slate-800 flex justify-end gap-3">
        <button
          @click="$emit('close')"
          class="px-4 py-2 rounded-lg bg-slate-800 text-slate-300 hover:bg-slate-700 text-xs font-semibold"
        >
          Cancel
        </button>
        <button
          @click="save"
          class="px-4 py-2 rounded-lg bg-amber-500 hover:bg-amber-400 text-slate-950 text-xs font-bold shadow-md"
        >
          Apply Parameters
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';

defineProps({
  isOpen: Boolean,
});

const emit = defineEmits(['close', 'saved']);

const kellyFraction = ref(0.25);
const minEv = ref(0.03);
const maxBetPct = ref(0.05);

const kellyLabel = computed(() => {
  if (kellyFraction.value <= 0.15) return 'Conservative';
  if (kellyFraction.value <= 0.30) return 'Quarter Kelly (Balanced)';
  return 'Aggressive';
});

const save = () => {
  emit('saved', {
    kellyFraction: kellyFraction.value,
    minEv: minEv.value,
    maxBetPct: maxBetPct.value,
  });
  emit('close');
};
</script>

