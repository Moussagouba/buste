<template>
  <div class="bg-[#131b2e] border border-slate-800 rounded-xl p-5 shadow-lg">
    <div class="flex flex-col sm:flex-row sm:items-center justify-between pb-4 border-b border-slate-800 gap-3">
      <div>
        <h3 class="text-base font-bold text-white flex items-center gap-2">
          <span class="w-2.5 h-2.5 rounded-full bg-emerald-400 animate-pulse"></span>
          Sources de Données Connectées (Multi-Source Feeds)
        </h3>
        <p class="text-xs text-slate-400">
          Agrégation hybride : Cotes en direct, historique CSV, xG StatsBomb, scores officiels & métadonnées
        </p>
      </div>

      <div class="flex items-center gap-2">
        <span class="text-xs text-slate-400 font-mono">
          <span class="text-emerald-400 font-bold">{{ activeSourcesCount }}</span> / {{ sources.length }} Actives
        </span>
      </div>
    </div>

    <!-- Grid of sources -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mt-4">
      <div
        v-for="source in sources"
        :key="source.name"
        class="p-4 rounded-xl border border-slate-800 bg-[#0d1322] hover:border-amber-500/40 transition flex flex-col justify-between group"
      >
        <div>
          <!-- Header -->
          <div class="flex items-center justify-between mb-2">
            <span class="font-bold text-white text-sm group-hover:text-amber-400 transition">
              {{ source.name }}
            </span>
            <span
              class="px-2 py-0.5 rounded-full text-[10px] font-mono font-bold"
              :class="source.is_active ? 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/20' : 'bg-slate-800 text-slate-400'"
            >
              {{ source.is_active ? '● Connecté' : '○ Standby' }}
            </span>
          </div>

          <!-- Role -->
          <p class="text-xs text-slate-300 font-medium mb-3 leading-relaxed">
            {{ source.role }}
          </p>
        </div>

        <!-- Metadata footer -->
        <div class="space-y-1.5 pt-3 border-t border-slate-800/80 text-[11px] font-mono">
          <div class="flex justify-between text-slate-400">
            <span>Accès :</span>
            <span class="text-amber-400">{{ source.access_type }}</span>
          </div>
          <div class="flex justify-between text-slate-400">
            <span>Limite Quota :</span>
            <span class="text-slate-300">{{ source.quota_limit }}</span>
          </div>
          <div class="flex justify-between text-slate-400">
            <span>Enregistrements :</span>
            <span class="text-emerald-400 font-bold">{{ source.records_cached }} éléments</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  sources: {
    type: Array,
    default: () => [],
  },
});

const activeSourcesCount = computed(() => {
  return props.sources.filter((s) => s.is_active).length;
});
</script>

