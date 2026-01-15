<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

const sources = [
  { name: 'sourceOriginal', url: 'https://a.dove.isdumb.one/list.txt' },
  { name: 'sourceFastly', url: 'https://fastly.jsdelivr.net/gh/ignaciocastro/a-dove-is-dumb@latest/list.txt' },
  { name: 'sourceGcore', url: 'https://gcore.jsdelivr.net/gh/ignaciocastro/a-dove-is-dumb@latest/list.txt' },
  { name: 'sourceQuantil', url: 'https://quantil.jsdelivr.net/gh/ignaciocastro/a-dove-is-dumb@latest/list.txt' },
  { name: 'sourceGhproxy', url: 'https://gh-proxy.com/https://raw.githubusercontent.com/ignaciocastro/a-dove-is-dumb/main/list.txt' }
];

const sourceUrl = ref(sources[0].url);

async function openExternalLink(url: string) {
  if (!(window as any).__TAURI_INTERNALS__) return;
  try {
    await invoke("plugin:opener|open", { path: url });
  } catch (err) {
    console.error("Failed to open link:", err);
  }
}
const sourceUpdateDate = ref("-");
const hostsStatus = ref(false);
const hostsUpdateDate = ref("-");
const isUpdating = ref(false);
const message = ref("");
const isError = ref(false);

async function checkHostsStatus() {
  if (!(window as any).__TAURI_INTERNALS__) {
    console.warn("Not running in Tauri environment.");
    return;
  }
  try {
    const [isBlocked, updateDate]: [boolean, string] = await invoke("get_hosts_status");
    hostsStatus.value = isBlocked;
    hostsUpdateDate.value = updateDate || "-";
  } catch (err) {
    console.error("Failed to check hosts status:", err);
    message.value = t('error', { msg: err });
    isError.value = true;
  }
}

async function updateHosts() {
  if (!(window as any).__TAURI_INTERNALS__) {
    message.value = "Tauri context not found.";
    isError.value = true;
    return;
  }
  isUpdating.value = true;
  message.value = t('updating');
  isError.value = false;
  try {
    await invoke("update_hosts", { url: sourceUrl.value });
    message.value = t('success');
    await checkHostsStatus();
  } catch (err) {
    console.error("Update failed:", err);
    message.value = t('error', { msg: err });
    isError.value = true;
  } finally {
    isUpdating.value = false;
  }
}

async function fetchSourceUpdateDate() {
  if (!(window as any).__TAURI_INTERNALS__) return;
  try {
    const date: string = await invoke("get_source_update_date", { url: sourceUrl.value });
    sourceUpdateDate.value = date;
  } catch (err) {
    console.error("Failed to fetch source:", err);
    sourceUpdateDate.value = t('fetchError');
  }
}

onMounted(() => {
  fetchSourceUpdateDate();
  checkHostsStatus();
});
</script>

<template>
  <main class="container">
    <h1>{{ t('title') }}</h1>

    <div class="form-container">
      <div class="form-row source-info-row">
        <label>{{ t('sourceLabel') }}</label>
        <div class="source-info-content">
          <div class="source-description">
            <span class="external-link" @click="openExternalLink('https://github.com/ignaciocastro/a-dove-is-dumb')">
              {{ t('sourceLink') }}
            </span>
          </div>
          <div class="source-selectors">
          <label v-for="source in sources" :key="source.url" class="radio-item">
            <input type="radio" v-model="sourceUrl" :value="source.url" @change="fetchSourceUpdateDate" />{{ t(source.name) }}
          </label>
        </div>
        </div>
      </div>

      <div class="form-row">
        <label>{{ t('sourceUpdateDate') }}</label>
        <span>{{ sourceUpdateDate }}</span>
      </div>

      <div class="form-row">
        <label>{{ t('statusLabel') }}</label>
        <span :class="{ 'status-blocked': hostsStatus, 'status-not-blocked': !hostsStatus }">
          {{ hostsStatus ? t('statusBlocked') : t('statusNotBlocked') }}
        </span>
      </div>

      <div class="form-row">
        <label>{{ t('hostsUpdateDate') }}</label>
        <span>{{ hostsUpdateDate }}</span>
      </div>

      <div class="button-row">
        <button @click="updateHosts" :disabled="isUpdating">
          {{ isUpdating ? t('updating') : t('updateButton') }}
        </button>
      </div>

      <div v-if="message" :class="['message', { 'error': isError }]">
        {{ message }}
      </div>
    </div>
  </main>
</template>

<style scoped>
.form-container {
  max-width: 540px;
  margin: 0.4rem auto;
  padding: 1rem;
  background: white;
  border-radius: 8px;
  box-shadow: 0 4px 6px rgba(0,0,0,0.1);
  text-align: left;
}

.form-row {
  margin-bottom: 1rem;
  display: flex;
  align-items: center;
}

.form-row label {
  width: 130px;
  font-weight: bold;
  color: #333;
}

.source-info-row {
  align-items: flex-start;
}

.source-info-content {
  flex: 1;
}

.source-description {
  margin-bottom: 0.5rem;
}

.external-link {
  color: #1a73e8;
  text-decoration: underline;
  cursor: pointer;
  font-size: 0.9rem;
}

.source-selectors {
  display: block;
  padding: 0;
  margin: 0;
  text-align: left;
}

.radio-item {
  display: inline-block;
  width: 33.33%;
  min-width: 120px;
  cursor: pointer;
  font-size: 0.85rem;
  color: #555;
  padding: 0.3rem 0;
  margin: 0;
  box-sizing: border-box;
  white-space: nowrap;
  text-align: left;
  vertical-align: middle;
}

.radio-item input {
  margin: 0 2px 0 0;
  padding: 0;
  width: auto;
  vertical-align: middle;
}

.form-row input {
  flex: 1;
  padding: 0.4rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 0.9rem;
}

.form-row span {
  flex: 1;
  color: #666;
}

.status-blocked {
  color: #4caf50 !important;
  font-weight: bold;
}

.status-not-blocked {
  color: #f44336 !important;
  font-weight: bold;
}

.button-row {
  display: flex;
  justify-content: center;
  margin-top: 1rem;
}

button {
  padding: 0.6rem 1.8rem;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.95rem;
  transition: background-color 0.2s;
}

button:hover:not(:disabled) {
  background-color: #0056b3;
}

button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

.message {
  margin-top: 1rem;
  padding: 0.8rem;
  border-radius: 4px;
  background-color: #e8f5e9;
  color: #2e7d32;
  text-align: center;
  font-size: 0.9rem;
}

.message.error {
  background-color: #ffebee;
  color: #c62828;
}
</style>

<style>
:root {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  background-color: #f0f2f5;
  color: #1c1e21;
}

.container {
  padding: 0.5rem;
  text-align: center;
}

h1 {
  color: #1a73e8;
  margin-bottom: 1rem;
  font-size: 1.5rem;
}
</style>
