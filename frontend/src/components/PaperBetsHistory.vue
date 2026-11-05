<template>
  <div class="bg-[#131b2e] border border-slate-800 rounded-xl p-5 shadow-lg">
    <div class="flex flex-col sm:flex-row sm:items-center justify-between pb-4 border-b border-slate-800 gap-3">
      <div>
        <h3 class="text-base font-bold text-white flex items-center gap-2">
          <span>Virtual Portfolio & Paper Bets Ledger</span>
        </h3>
        <p class="text-xs text-slate-400">
          Tracking execution quality, Closing Line Value (CLV), and historical P&L
        </p>
      </div>

      <div class="flex items-center gap-2">
        <button
          @click="filter = 'ALL'"
          :class="filter === 'ALL' ? 'bg-slate-700 text-white' : 'text-slate-400 hover:text-white'"
          class="px-2.5 py-1 text-xs rounded transition"
        >
          All
        </button>
        <button
          @click="filter = 'PENDING'"
          :class="filter === 'PENDING' ? 'bg-amber-500/20 text-amber-400' : 'text-slate-400 hover:text-white'"
          class="px-2.5 py-1 text-xs rounded transition"
        >
          Pending
        </button>
        <button
          @click="filter = 'WON'"
          :class="filter === 'WON' ? 'bg-emerald-500/20 text-emerald-400' : 'text-slate-400 hover:text-white'"
          class="px-2.5 py-1 text-xs rounded transition"
        >
          Won
        </button>
        <button
          @click="filter = 'LOST'"
          :class="filter === 'LOST' ? 'bg-rose-500/20 text-rose-400' : 'text-slate-400 hover:text-white'"
          class="px-2.5 py-1 text-xs rounded transition"
        >
          Lost
        </button>
      </div>
    </div>

    <div class="overflow-x-auto mt-4">
      <table class="w-full text-left text-xs">
        <thead class="text-slate-400 uppercase tracking-wider font-semibold border-b border-slate-800">
          <tr>
            <th class="pb-3 px-3">Date</th>
            <th class="pb-3 px-3">Match</th>
            <th class="pb-3 px-3">Outcome</th>
            <th class="pb-3 px-3 text-right">Odds</th>
            <th class="pb-3 px-3 text-right">Stake</th>
            <th class="pb-3 px-3 text-center">Status</th>
            <th class="pb-3 px-3 text-right">CLV</th>
            <th class="pb-3 px-3 text-right">Profit / Loss</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-slate-800/60 font-sans">
          <tr v-if="filteredBets.length === 0">
            <td colspan="8" class="text-center py-8 text-slate-500">
              No paper bets found for the selected filter.
            </td>
          </tr>
          <tr
            v-for="bet in filteredBets"
            :key="bet.id"
            class="hover:bg-slate-800/40 transition"
          >
            <!-- Date -->
            <td class="py-3 px-3 text-slate-400 font-mono text-[11px]">
              {{ formatDate(bet.placed_at) }}
            </td>

            <!-- Match -->
            <td class="py-3 px-3">
              <div class="font-medium text-white">{{ bet.home_team }} vs {{ bet.away_team }}</div>
              <div class="text-[10px] text-slate-400">{{ bet.sport_name }} • {{ bet.league_name }}</div>
            </td>

            <!-- Outcome -->
            <td class="py-3 px-3">
              <span class="font-mono font-bold text-amber-400 bg-amber-400/10 px-2 py-0.5 rounded text-[11px]">
                {{ bet.outcome_name }}
              </span>
            </td>

            <!-- Odds -->
            <td class="py-3 px-3 text-right font-mono font-bold text-slate-200">
              {{ bet.odds_placed?.toFixed(2) }}
            </td>

            <!-- Stake -->
            <td class="py-3 px-3 text-right font-mono text-white font-medium">
              €{{ bet.stake_amount?.toFixed(2) }}
            </td>

            <!-- Status -->
            <td class="py-3 px-3 text-center">
              <span
                :class="{
                  'bg-amber-500/10 text-amber-400 border border-amber-500/20': bet.status === 'PENDING',
                  'bg-emerald-500/10 text-emerald-400 border border-emerald-500/20': bet.status === 'WON',
                  'bg-rose-500/10 text-rose-400 border border-rose-500/20': bet.status === 'LOST',
                  'bg-slate-700/50 text-slate-400': bet.status === 'VOID',
                }"
                class="px-2.5 py-0.5 rounded-full text-[10px] font-bold uppercase tracking-wider"
              >
                {{ bet.status }}
              </span>
              <div v-if="bet.final_score" class="text-[10px] text-slate-400 mt-0.5">
                {{ bet.final_score }}
              </div>
            </td>

            <!-- CLV -->
            <td class="py-3 px-3 text-right font-mono">
              <span
                v-if="bet.closing_line_value !== null && bet.closing_line_value !== undefined"
                :class="bet.closing_line_value >= 0 ? 'text-emerald-400' : 'text-rose-400'"
                class="text-[11px] font-semibold"
              >
                {{ bet.closing_line_value >= 0 ? '+' : '' }}{{ (bet.closing_line_value * 100).toFixed(1) }}%
              </span>
              <span v-else class="text-slate-600">-</span>
            </td>

            <!-- Profit / Loss -->
            <td class="py-3 px-3 text-right font-mono font-bold text-sm">
              <span
                v-if="bet.profit_loss !== null && bet.profit_loss !== undefined"
                :class="bet.profit_loss >= 0 ? 'text-emerald-400' : 'text-rose-400'"
              >
                {{ bet.profit_loss >= 0 ? '+' : '' }}€{{ bet.profit_loss.toFixed(2) }}
              </span>
              <span v-else class="text-slate-500 font-normal text-xs">
                pot. €{{ (bet.stake_amount * bet.odds_placed).toFixed(2) }}
              </span>
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
  bets: {
    type: Array,
    default: () => [],
  },
});

const filter = ref('ALL');

const filteredBets = computed(() => {
  if (filter.value === 'ALL') return props.bets;
  return props.bets.filter((b) => b.status === filter.value);
});

const formatDate = (isoString) => {
  if (!isoString) return '';
  const d = new Date(isoString);
  return d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }) + ' ' + d.toLocaleDateString([], { month: 'short', day: 'numeric' });
};
</script>

