import { createI18n } from 'vue-i18n';

const messages = {
  en: {
    title: 'Adobe Network Blocker',
    sourceLabel: 'Block List Source:',
    sourceUpdateDate: 'Source Update Date:',
    statusLabel: 'Block Status:',
    hostsUpdateDate: 'Hosts Update Date:',
    statusBlocked: 'Blocked',
    statusNotBlocked: 'Not Blocked',
    updateButton: 'Update Hosts',
    updating: 'Updating...',
    success: 'Hosts updated successfully!',
    error: 'Error: {msg}',
    permissionError: 'Permission denied. Please run as administrator/sudo.',
    fetchError: 'Failed to fetch source data.',
    sourceDescription: 'Source Description:',
    sourceLink: 'github ignaciocastro/a-dove-is-dumb',
    sourceOriginal: 'Original',
    sourceFastly: 'Fastly Mirror',
    sourceGcore: 'Gcore Mirror',
    sourceQuantil: 'Quantil Mirror',
    sourceGhproxy: 'Ghproxy'
  },
  zh: {
    title: '屏蔽Adobe联网',
    sourceLabel: '屏蔽信息来源:',
    sourceUpdateDate: '信息源更新日期:',
    statusLabel: '屏蔽状态:',
    hostsUpdateDate: '更新日期:',
    statusBlocked: '已屏蔽',
    statusNotBlocked: '未屏蔽',
    updateButton: '更新Hosts',
    updating: '更新中...',
    success: 'Hosts 更新成功！',
    error: '错误: {msg}',
    permissionError: '权限不足，请以管理员身份运行。',
    fetchError: '获取信息源失败。',
    sourceDescription: '来源说明:',
    sourceLink: 'github ignaciocastro/a-dove-is-dumb',
    sourceOriginal: '原地址',
    sourceFastly: 'Fastly镜像',
    sourceGcore: 'Gcore镜像',
    sourceQuantil: 'Quantil镜像',
    sourceGhproxy: 'Ghproxy'
  }
};

const i18n = createI18n({
  legacy: false,
  locale: navigator.language.startsWith('zh') ? 'zh' : 'en',
  fallbackLocale: 'en',
  messages,
});

export default i18n;
