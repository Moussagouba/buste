<template>
  <div class="bg-[#131b2e] border border-slate-800 rounded-xl p-5 shadow-lg">
    <div class="flex flex-col sm:flex-row sm:items-center justify-between pb-4 border-b border-slate-800 gap-3">
      <div>
        <h3 class="text-base font-bold text-white flex items-center gap-2">
          <span class="px-2 py-0.5 rounded text-xs bg-emerald-500/20 text-emerald-400 font-mono font-bold">+EV</span>
          Detected Value Betting Opportunities
        </h3>
        <p class="text-xs text-slate-400">
          Filtered by Mathematical Expectation > 3% and Kelly Criterion
        </p>
      </div>
      <div class="flex items-center gap-2">
        <span class="text-xs text-slate-400">Sort by:</span>
        <button
          @click="sortBy = 'ev'"
          :class="sortBy === 'ev' ? 'bg-amber-500/20 text-amber-400 border-amber-500/40' : 'text-slate-400 border-slate-700'"
          class="px-2.5 py-1 text-xs rounded border transition font-medium"
        >
          Highest EV%
        </button>
        <button
          @click="sortBy = 'confidence'"
          :class="sortBy === 'confidence' ? 'bg-amber-500/20 text-amber-400 border-amber-500/40' : 'text-slate-400 border-slate-700'"
          class="px-2.5 py-1 text-xs rounded border transition font-medium"
        >
          Confidence
        </button>
      </div>
    </div>

    <div class="overflow-x-auto mt-4">
      <table class="w-full text-left text-xs">
        <thead class="text-slate-400 uppercase tracking-wider font-semibold border-b border-slate-800">
          <tr>
            <th class="pb-3 px-3">Sport / Fixture</th>
            <th class="pb-3 px-3">Market</th>
            <th class="pb-3 px-3">Pick</th>
            <th class="pb-3 px-3 text-right">Melbet Odds</th>
            <th class="pb-3 px-3 text-right">Model Fair</th>
            <th class="pb-3 px-3 text-right">Model Prob</th>
            <th class="pb-3 px-3 text-right">Expected Value</th>
            <th class="pb-3 px-3 text-right">Kelly Stake</th>
            <th class="pb-3 px-3 text-center">Action</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-slate-800/60 font-sans">
          <tr
            v-for="bet in sortedBets"
            :key="bet.prediction_id || bet.match_name"
            class="hover:bg-slate-800/40 transition group"
          >
            <!-- Fixture -->
            <td class="py-3 px-3">
              <div class="font-bold text-white text-sm">{{ bet.match_name }}</div>
              <div class="text-[11px] text-slate-400 flex items-center gap-2 mt-0.5">
                <span class="text-amber-400 font-medium">{{ bet.sport_name }}</span>
                <span>•</span>
                <span>{{ bet.league_name }}</span>
              </div>
            </td>

            <!-- Market -->
            <td class="py-3 px-3">
              <span class="px-2 py-0.5 rounded bg-slate-800 text-slate-300 font-mono text-[11px]">
                {{ bet.market_type }}
              </span>
            </td>

            <!-- Pick -->
            <td class="py-3 px-3">
              <span class="font-bold text-amber-400 bg-amber-400/10 px-2.5 py-1 rounded text-xs">
                {{ bet.outcome_name }}
              </span>
            </td>

            <!-- Melbet Odds -->
            <td class="py-3 px-3 text-right font-mono font-bold text-white text-sm">
              {{ bet.bookmaker_odds.toFixed(2) }}
            </td>

            <!-- Model Fair Odds -->
            <td class="py-3 px-3 text-right font-mono text-slate-400">
              {{ bet.fair_odds.toFixed(2) }}
            </td>

            <!-- Model Probability -->
            <td class="py-3 px-3 text-right font-mono">
              <span class="text-emerald-400 font-semibold">{{ (bet.model_probability * 100).toFixed(1) }}%</span>
              <div class="text-[10px] text-slate-500">vs {{ (bet.implied_probability * 100).toFixed(1) }}% imp</div>
            </td>

            <!-- Expected Value -->
            <td class="py-3 px-3 text-right font-mono font-bold">
              <span class="text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded">
                +{{ (bet.expected_value * 100).toFixed(1) }}%
              </span>
              <div class="text-[10px] text-slate-400 mt-0.5">Edge: +{{ (bet.edge * 100).toFixed(1) }}%</div>
            </td>

            <!-- Kelly Stake -->
            <td class="py-3 px-3 text-right font-mono">
              <div class="text-white font-bold">€{{ bet.recommended_stake_amount.toFixed(2) }}</div>
              <div class="text-[10px] text-slate-400">{{ (bet.recommended_stake_pct * 100).toFixed(1) }}% bankroll</div>
            </td>

            <!-- Action -->
            <td class="py-3 px-3 text-center">
              <button
                @click="$emit('place-bet', bet)"
                class="px-3 py-1.5 bg-gradient-to-r from-amber-500 to-amber-600 hover:from-amber-400 hover:to-amber-500 text-slate-950 font-bold rounded-lg shadow-sm transition active:scale-95 text-xs flex items-center justify-center gap-1 mx-auto"
              >
                Paper Bet
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';

const props = defineProps({
  valueBets: {
    type: Array,
    default: () => [],
  },
});

defineEmits(['place-bet']);

const sortBy = ref('ev');

const sortedBets = computed(() => {
  return [...props.valueBets].sort((a, b) => {
    if (sortBy.value === 'ev') {
      return b.expected_value - a.expected_value;
    }
    return b.confidence_score - a.confidence_score;
  });
});
</script>

