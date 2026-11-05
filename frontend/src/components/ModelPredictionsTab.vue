<template>
  <div class="space-y-6">
    <!-- Header with controls -->
    <div class="bg-[#131b2e] border border-slate-800 rounded-xl p-5 shadow-lg flex flex-col md:flex-row md:items-center justify-between gap-4">
      <div>
        <h3 class="text-base font-bold text-white flex items-center gap-2">
          <span class="px-2 py-0.5 rounded text-xs bg-indigo-500/20 text-indigo-400 font-mono font-bold">IA & Stats</span>
          Pronostics & Modèles Statistiques Avancés
        </h3>
        <p class="text-xs text-slate-400 mt-0.5">
          Calculés par la loi de Poisson bivariée, le modèle de Dixon-Coles et les intensités offensives/défensives réelles
        </p>
      </div>

      <!-- Filters -->
      <div class="flex items-center gap-3">
        <div class="relative">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Rechercher une équipe..."
            class="px-3 py-1.5 rounded-lg bg-slate-900 border border-slate-700 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-amber-400 w-48 font-sans"
          />
        </div>

        <select
          v-model="selectedLeague"
          class="px-3 py-1.5 rounded-lg bg-slate-900 border border-slate-700 text-xs text-slate-200 focus:outline-none focus:border-amber-400 font-sans cursor-pointer"
        >
          <option value="ALL">Toutes les ligues</option>
          <option value="Premier League">Premier League (football-data.org)</option>
          <option value="Bundesliga">Bundesliga (OpenLigaDB)</option>
        </select>
      </div>
    </div>

    <!-- Predictions Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-5">
      <div
        v-for="item in filteredPredictions"
        :key="item.id"
        class="bg-[#131b2e] border border-slate-800 rounded-xl p-5 shadow-lg hover:border-indigo-500/50 transition flex flex-col justify-between group"
      >
        <!-- Card Header -->
        <div>
          <div class="flex items-center justify-between pb-3 border-b border-slate-800 text-xs">
            <div class="flex items-center gap-2">
              <span class="px-2 py-0.5 rounded bg-slate-800 text-amber-400 font-semibold font-mono text-[11px]">
                {{ item.league_name }}
              </span>
              <span class="text-slate-500">•</span>
              <span class="text-slate-400 font-mono text-[11px]">{{ formatDate(item.start_time) }}</span>
            </div>

            <!-- Confidence badge -->
            <div class="flex items-center gap-1.5">
              <span class="text-[11px] text-slate-400">Indice Confiance :</span>
              <span
                class="px-2 py-0.5 rounded-full text-[11px] font-bold font-mono"
                :class="item.confidence_score >= 0.85 ? 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/20' : 'bg-amber-500/10 text-amber-400 border border-amber-500/20'"
              >
                {{ (item.confidence_score * 100).toFixed(0) }}%
              </span>
            </div>
          </div>

          <!-- Fixture & Expected Intensity -->
          <div class="py-4 flex items-center justify-between">
            <!-- Home Team -->
            <div class="flex-1 text-center">
              <div v-if="item.home_crest" class="w-10 h-10 mx-auto mb-2 flex items-center justify-center">
                <img :src="item.home_crest" :alt="item.home_team" class="max-h-full max-w-full object-contain" />
              </div>
              <div v-else class="w-10 h-10 rounded-full bg-slate-800 mx-auto mb-2 flex items-center justify-center text-xs font-bold text-slate-300">
                {{ item.home_team.slice(0, 3) }}
              </div>
              <div class="font-bold text-white text-sm tracking-tight">{{ item.home_team }}</div>
              <div class="text-[11px] text-slate-400 mt-0.5 font-mono">
                Exp. Buts (&lambda;) : <span class="text-amber-400 font-bold">{{ item.lambda.toFixed(2) }}</span>
              </div>
            </div>

            <!-- Score Predictor Box -->
            <div class="px-4 text-center">
              <div class="text-[10px] text-slate-500 uppercase tracking-widest font-mono font-semibold mb-1">
                Score le + probable
              </div>
              <div class="px-3.5 py-1.5 rounded-lg bg-indigo-500/15 border border-indigo-500/30 text-indigo-300 font-black text-xl tracking-wider font-mono">
                {{ item.most_likely_score }}
              </div>
              <div class="text-[10px] text-indigo-400 mt-1 font-mono">
                {{ item.most_likely_score_prob }}% probabilité
              </div>
            </div>

            <!-- Away Team -->
            <div class="flex-1 text-center">
              <div v-if="item.away_crest" class="w-10 h-10 mx-auto mb-2 flex items-center justify-center">
                <img :src="item.away_crest" :alt="item.away_team" class="max-h-full max-w-full object-contain" />
              </div>
              <div v-else class="w-10 h-10 rounded-full bg-slate-800 mx-auto mb-2 flex items-center justify-center text-xs font-bold text-slate-300">
                {{ item.away_team.slice(0, 3) }}
              </div>
              <div class="font-bold text-white text-sm tracking-tight">{{ item.away_team }}</div>
              <div class="text-[11px] text-slate-400 mt-0.5 font-mono">
                Exp. Buts (&mu;) : <span class="text-amber-400 font-bold">{{ item.mu.toFixed(2) }}</span>
              </div>
            </div>
          </div>

          <!-- 1X2 Probabilities Probability Bar -->
          <div class="space-y-1.5 mb-4">
            <div class="flex justify-between text-[11px] font-mono text-slate-400">
              <span class="text-emerald-400 font-bold">1: {{ item.prob_1 }}% (Cote Équit. {{ item.fair_odds_1 }})</span>
              <span class="text-amber-400 font-bold">X: {{ item.prob_x }}% ({{ item.fair_odds_x }})</span>
              <span class="text-cyan-400 font-bold">2: {{ item.prob_2 }}% ({{ item.fair_odds_2 }})</span>
            </div>

            <!-- Stacked Progress Bar -->
            <div class="w-full h-2.5 rounded-full bg-slate-800 overflow-hidden flex">
              <div :style="{ width: item.prob_1 + '%' }" class="bg-emerald-500 h-full" title="Victoire Domicile"></div>
              <div :style="{ width: item.prob_x + '%' }" class="bg-amber-500 h-full" title="Match Nul"></div>
              <div :style="{ width: item.prob_2 + '%' }" class="bg-cyan-500 h-full" title="Victoire Extérieur"></div>
            </div>
          </div>

          <!-- Secondary Statistical Metrics -->
          <div class="grid grid-cols-2 gap-3 text-xs mb-4">
            <!-- Over / Under 2.5 -->
            <div class="p-2.5 rounded-lg bg-slate-900/60 border border-slate-800/80">
              <div class="text-[10px] text-slate-400 uppercase font-mono font-semibold">Total Buts (Over / Under 2.5)</div>
              <div class="mt-1 flex items-center justify-between font-mono">
                <span class="text-slate-200">Plus 2.5 : <strong class="text-emerald-400">{{ item.prob_over_25 }}%</strong></span>
                <span class="text-slate-200">Moins : <strong class="text-rose-400">{{ item.prob_under_25 }}%</strong></span>
              </div>
              <div class="text-[10px] text-slate-500 mt-0.5">Espérance totale : {{ item.expected_total_goals }} buts</div>
            </div>

            <!-- Both Teams to Score (BTTS) -->
            <div class="p-2.5 rounded-lg bg-slate-900/60 border border-slate-800/80">
              <div class="text-[10px] text-slate-400 uppercase font-mono font-semibold">Les 2 Équipes Marquent (BTTS)</div>
              <div class="mt-1 flex items-center justify-between font-mono">
                <span class="text-slate-200">Oui : <strong class="text-emerald-400">{{ item.prob_btts_yes }}%</strong></span>
                <span class="text-slate-200">Non : <strong class="text-rose-400">{{ item.prob_btts_no }}%</strong></span>
              </div>
              <div class="text-[10px] text-slate-500 mt-0.5">Basé sur Dixon-Coles & Poisson</div>
            </div>
          </div>
        </div>

        <!-- Pronostic Recommandé (The Best Pick) -->
        <div class="pt-3 border-t border-slate-800 flex items-center justify-between">
          <div>
            <span class="text-[10px] text-slate-400 uppercase font-mono tracking-wider">Pronostic Recommandé :</span>
            <div class="font-bold text-amber-400 text-sm flex items-center gap-1.5 mt-0.5">
              <span>🎯 {{ item.recommended_pick }}</span>
              <span class="text-xs text-slate-400 font-mono font-normal">(@ cote équit. {{ item.recommended_pick_odds }})</span>
            </div>
          </div>

          <button
            @click="$emit('bet-pick', item)"
            class="px-3 py-1.5 rounded-lg bg-gradient-to-r from-indigo-500 to-indigo-600 hover:from-indigo-400 hover:to-indigo-500 text-white font-bold text-xs shadow-md active:scale-95 transition"
          >
            Parier ce prono
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';

const props = defineProps({
  predictions: {
    type: Array,
    default: () => [],
  },
});

defineEmits(['bet-pick']);

const searchQuery = ref('');
const selectedLeague = ref('ALL');

const filteredPredictions = computed(() => {
  return props.predictions.filter((item) => {
    const matchesSearch =
      !searchQuery.value ||
      item.home_team.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      item.away_team.toLowerCase().includes(searchQuery.value.toLowerCase());

    const matchesLeague =
      selectedLeague.value === 'ALL' || item.league_name === selectedLeague.value;

    return matchesSearch && matchesLeague;
  });
});

const formatDate = (isoString) => {
  if (!isoString) return '';
  const d = new Date(isoString);
  return (
    d.toLocaleDateString([], { weekday: 'short', month: 'short', day: 'numeric' }) +
    ' à ' +
    d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
  );
};
</script>

