const queryInput = document.getElementById("queryInput");
const liveSearchBtn = document.getElementById("liveSearchBtn");
const liveSearchMeta = document.getElementById("liveSearchMeta");
const riskFilter = document.getElementById("riskFilter");
const groupFilter = document.getElementById("groupFilter");
const filterBar = document.getElementById("filterBar");
const toggleFilterBtn = document.getElementById("toggleFilterBtn");
const payloadInput = document.getElementById("payloadInput");
const renderBtn = document.getElementById("renderBtn");
const payloadErr = document.getElementById("payloadErr");
const groupsRoot = document.getElementById("groupsRoot");
const emptyState = document.getElementById("emptyState");
const resultMeta = document.getElementById("resultMeta");
const actionRunMeta = document.getElementById("actionRunMeta");
const quickActionMeta = document.getElementById("quickActionMeta");

const langZhBtn = document.getElementById("langZhBtn");
const langEnBtn = document.getElementById("langEnBtn");
const openSettingsBtn = document.getElementById("openSettingsBtn");
const closeSettingsBtn = document.getElementById("closeSettingsBtn");
const settingsPanel = document.getElementById("settingsPanel");
const advancedTitle = document.getElementById("advancedTitle");

const opButtons = document.querySelectorAll(".op-btn");
const quickQueryButtons = document.querySelectorAll(".quick-query");

const settingInputs = {
  auto_update: document.getElementById("settingAutoUpdate"),
  signature_check: document.getElementById("settingSignature"),
  source_allowlist: document.getElementById("settingAllowlist"),
  audit_enabled: document.getElementById("settingAudit"),
  scheduler_enabled: document.getElementById("settingScheduler"),
  worker_enabled: document.getElementById("settingWorker")
};

const LANG_KEY = "synora_ui_lang_v1";
const SETTINGS_KEY = "synora_ui_settings_v1";
let currentPayload = null;
let currentLang = localStorage.getItem(LANG_KEY) || "zh";

const I18N = {
  zh: {
    title: "Synora 中文界面",
    subtitle: "Phase 9：前台操作 + 设置分层",
    queryLabel: "搜索关键词",
    liveSearchBtn: "实时搜索",
    payloadTitle: "粘贴 `ui search --json` 输出",
    renderBtn: "渲染结果",
    groupTitle: "结果分组",
    riskLabel: "风险等级",
    typeLabel: "分组类型",
    advancedTitle: "高级（开发调试）",
    quickActionTitle: "快捷操作",
    quickActionSubtitle: "常用动作按钮，点击后由后端执行",
    settingsBtn: "设置",
    settingsTitle: "设置",
    settingsSubtitle: "策略与系统参数（默认隐藏）",
    closeBtn: "关闭",
    settingAutoUpdateLabel: "轻量定时更新检测",
    settingSignatureLabel: "签名校验",
    settingAllowlistLabel: "来源白名单",
    settingAuditLabel: "审计记录",
    settingSchedulerLabel: "Scheduler 定时入队",
    settingWorkerLabel: "Worker 执行器",
    settingDiscoveryScope: "Discovery 范围：Registry-only（只读）",
    queryPlaceholder: "输入关键词，如 PowerToys",
    payloadPlaceholder: "粘贴 ui search --json 的完整输出",
    riskOptions: { all: "全部", low: "低", medium: "中", high: "高" },
    groupOptions: {
      all: "全部",
      software: "软件",
      source: "来源",
      update: "更新",
      download: "下载",
      ai: "AI"
    },
    groupNames: {
      software: "软件",
      source: "来源",
      update: "更新",
      download: "下载",
      ai: "AI",
      unknown: "未知"
    },
    actionButtons: {
      update_check: "检测可更新软件",
      discover_scan: "软件自动发现",
      repo_sync: "同步软件仓库",
      ai_analyze: "AI 整理分析",
      ai_recommend: "AI 安装建议",
      ai_repair_plan: "AI 修复方案"
    },
    prompts: {
      enterQuery: "请输入搜索关键词。",
      searching: "正在加载结果...",
      liveHtmlError: "实时搜索接口返回 HTML。请先执行：python scripts/ui_dev_server.py，然后访问 http://127.0.0.1:8787/",
      liveNonJsonError: "实时搜索返回了非 JSON 响应。",
      liveFailed: "实时搜索失败。",
      liveOk: "实时搜索成功：分组数={groups}",
      liveException: "实时搜索异常：{msg}",
      filters: "筛选",
      resultMeta: "关键词=\"{query}\" | 分组={count}",
      resultEmpty: "没有找到匹配结果，换个关键词试试。",
      actionRunning: "执行中...",
      actionNeedConfirm: "该动作是高风险操作，需要确认后执行，是否继续？",
      actionOk: "动作执行成功。",
      actionFail: "动作失败（exit {code}）",
      actionException: "动作执行异常：{msg}",
      actionNonJsonError: "动作接口返回了非 JSON 响应。",
      actionHtmlError: "动作接口返回 HTML。请先启动 ui_dev_server.py。",
      opRunning: "正在执行：{name}",
      opOk: "完成：{name}",
      opFail: "失败：{name}（exit {code}）",
      opException: "操作异常：{msg}",
      opNonJsonError: "操作接口返回了非 JSON 响应。",
      opHtmlError: "操作接口返回 HTML。请先启动 ui_dev_server.py。",
      settingsSaved: "设置已保存。"
    },
    labels: {
      risk: "风险",
      confidence: "置信度",
      execute: "执行"
    }
  },
  en: {
    title: "Synora UI",
    subtitle: "Phase 9: actions in front, settings by policy",
    queryLabel: "Search Query",
    liveSearchBtn: "Live Search",
    payloadTitle: "Paste `ui search --json` Payload",
    renderBtn: "Render Results",
    groupTitle: "Result Groups",
    riskLabel: "Risk Level",
    typeLabel: "Group Type",
    advancedTitle: "Advanced (Developer)",
    quickActionTitle: "Quick Actions",
    quickActionSubtitle: "Common actions, executed by backend",
    settingsBtn: "Settings",
    settingsTitle: "Settings",
    settingsSubtitle: "Policy and system parameters",
    closeBtn: "Close",
    settingAutoUpdateLabel: "Lightweight scheduled update detection",
    settingSignatureLabel: "Signature verification",
    settingAllowlistLabel: "Source allowlist",
    settingAuditLabel: "Audit logs",
    settingSchedulerLabel: "Scheduler timed enqueue",
    settingWorkerLabel: "Worker executor",
    settingDiscoveryScope: "Discovery scope: Registry-only (read-only)",
    queryPlaceholder: "Type keyword, e.g. PowerToys",
    payloadPlaceholder: "Paste full JSON output from ui search --json",
    riskOptions: { all: "All", low: "Low", medium: "Medium", high: "High" },
    groupOptions: {
      all: "All",
      software: "Software",
      source: "Source",
      update: "Update",
      download: "Download",
      ai: "AI"
    },
    groupNames: {
      software: "Software",
      source: "Source",
      update: "Update",
      download: "Download",
      ai: "AI",
      unknown: "Unknown"
    },
    actionButtons: {
      update_check: "Check Updates",
      discover_scan: "Run Discovery",
      repo_sync: "Sync Repositories",
      ai_analyze: "AI Analyze",
      ai_recommend: "AI Recommend",
      ai_repair_plan: "AI Repair Plan"
    },
    prompts: {
      enterQuery: "Please enter a query.",
      searching: "Loading results...",
      liveHtmlError: "Live search endpoint returned HTML. Start server with: python scripts/ui_dev_server.py, then open http://127.0.0.1:8787/",
      liveNonJsonError: "Live search returned a non-JSON response.",
      liveFailed: "Live search failed.",
      liveOk: "Live search succeeded: groups={groups}",
      liveException: "Live search exception: {msg}",
      filters: "Filters",
      resultMeta: "query=\"{query}\" | groups={count}",
      resultEmpty: "No matching results. Try another query.",
      actionRunning: "Running...",
      actionNeedConfirm: "This action is high risk and needs confirmation. Continue?",
      actionOk: "Action executed.",
      actionFail: "Action failed (exit {code})",
      actionException: "Action exception: {msg}",
      actionNonJsonError: "Action endpoint returned a non-JSON response.",
      actionHtmlError: "Action endpoint returned HTML. Start ui_dev_server.py first.",
      opRunning: "Running: {name}",
      opOk: "Completed: {name}",
      opFail: "Failed: {name} (exit {code})",
      opException: "Operation exception: {msg}",
      opNonJsonError: "Operation endpoint returned non-JSON.",
      opHtmlError: "Operation endpoint returned HTML. Start ui_dev_server.py first.",
      settingsSaved: "Settings saved."
    },
    labels: {
      risk: "risk",
      confidence: "conf",
      execute: "Run"
    }
  }
};

const examplePayload = {
  query: "PowerToys",
  groups: [
    {
      type: "software",
      items: [
        {
          title: "PowerToys (Preview) x64",
          subtitle: "0.97.2 | Microsoft Corporation | active",
          risk_level: "low",
          confidence: 80,
          action_id: "software.show:111"
        }
      ]
    }
  ]
};

function t() {
  return I18N[currentLang] || I18N.zh;
}

function fmt(template, vars = {}) {
  return String(template || "").replace(/\{(\w+)\}/g, (_m, k) =>
    Object.prototype.hasOwnProperty.call(vars, k) ? String(vars[k]) : ""
  );
}

function setText(id, value) {
  const el = document.getElementById(id);
  if (el) el.textContent = value;
}

function setLangButtons() {
  const zhActive = currentLang === "zh";
  langZhBtn.className = zhActive
    ? "rounded-md bg-stone-800 px-2 py-1 text-xs font-semibold text-white"
    : "rounded-md border border-stone-400 bg-white px-2 py-1 text-xs font-semibold text-stone-700";
  langEnBtn.className = !zhActive
    ? "rounded-md bg-stone-800 px-2 py-1 text-xs font-semibold text-white"
    : "rounded-md border border-stone-400 bg-white px-2 py-1 text-xs font-semibold text-stone-700";
}

function applyLanguage() {
  const dict = t();
  document.documentElement.lang = currentLang === "zh" ? "zh-CN" : "en";
  document.title = dict.title;

  setText("titleText", dict.title);
  setText("subtitleText", dict.subtitle);
  setText("queryLabel", dict.queryLabel);
  setText("liveSearchBtn", dict.liveSearchBtn);
  setText("quickActionTitle", dict.quickActionTitle);
  setText("quickActionSubtitle", dict.quickActionSubtitle);
  setText("openSettingsBtn", dict.settingsBtn);
  setText("settingsTitle", dict.settingsTitle);
  setText("settingsSubtitle", dict.settingsSubtitle);
  setText("closeSettingsBtn", dict.closeBtn);

  setText("settingAutoUpdateLabel", dict.settingAutoUpdateLabel);
  setText("settingSignatureLabel", dict.settingSignatureLabel);
  setText("settingAllowlistLabel", dict.settingAllowlistLabel);
  setText("settingAuditLabel", dict.settingAuditLabel);
  setText("settingSchedulerLabel", dict.settingSchedulerLabel);
  setText("settingWorkerLabel", dict.settingWorkerLabel);
  setText("settingDiscoveryScope", dict.settingDiscoveryScope);

  setText("payloadTitle", dict.payloadTitle);
  setText("renderBtn", dict.renderBtn);
  setText("groupTitle", dict.groupTitle);
  setText("riskLabel", dict.riskLabel);
  setText("typeLabel", dict.typeLabel);
  setText("advancedTitle", dict.advancedTitle);
  setText("toggleFilterBtn", dict.prompts.filters);

  opButtons.forEach((btn) => {
    const op = btn.dataset.op;
    btn.textContent = dict.actionButtons[op] || op;
  });

  queryInput.placeholder = dict.queryPlaceholder;
  payloadInput.placeholder = dict.payloadPlaceholder;

  for (const option of riskFilter.options) {
    const label = dict.riskOptions[option.value];
    if (label) option.textContent = label;
  }
  for (const option of groupFilter.options) {
    const label = dict.groupOptions[option.value];
    if (label) option.textContent = label;
  }

  setLangButtons();
  if (currentPayload) render(currentPayload);
}

function setLanguage(lang) {
  currentLang = lang === "en" ? "en" : "zh";
  localStorage.setItem(LANG_KEY, currentLang);
  applyLanguage();
}

function loadSettings() {
  try {
    return JSON.parse(localStorage.getItem(SETTINGS_KEY) || "{}");
  } catch (_e) {
    return {};
  }
}

function saveSettings() {
  const data = {};
  Object.entries(settingInputs).forEach(([key, el]) => {
    data[key] = Boolean(el?.checked);
  });
  localStorage.setItem(SETTINGS_KEY, JSON.stringify(data));
  quickActionMeta.textContent = t().prompts.settingsSaved;
}

function applySettingsToUi() {
  const data = loadSettings();
  Object.entries(settingInputs).forEach(([key, el]) => {
    if (el && Object.prototype.hasOwnProperty.call(data, key)) {
      el.checked = Boolean(data[key]);
    }
  });
}

function setSearchLoading(isLoading) {
  liveSearchBtn.disabled = isLoading;
  liveSearchBtn.classList.toggle("opacity-60", isLoading);
  liveSearchBtn.classList.toggle("cursor-not-allowed", isLoading);
}

function riskChipClass(risk) {
  if (risk === "high") return "text-red-800 border-red-300 bg-red-50";
  if (risk === "medium") return "text-amber-800 border-amber-300 bg-amber-50";
  return "text-green-800 border-green-300 bg-green-50";
}

async function runActionWithId(actionId, riskLevel) {
  const confirmNeeded = riskLevel === "high";
  if (confirmNeeded && !window.confirm(t().prompts.actionNeedConfirm)) return;

  actionRunMeta.textContent = t().prompts.actionRunning;

  try {
    const res = await fetch("/api/action-run", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ id: actionId, confirm: confirmNeeded })
    });
    const raw = await res.text();
    let payload = null;
    try {
      payload = JSON.parse(raw);
    } catch (_e) {
      const looksHtml = raw.trim().startsWith("<!DOCTYPE") || raw.trim().startsWith("<html");
      actionRunMeta.textContent = looksHtml ? t().prompts.actionHtmlError : t().prompts.actionNonJsonError;
      return;
    }
    if (!payload.ok) {
      const code = payload.exit_code ?? "?";
      actionRunMeta.textContent = fmt(t().prompts.actionFail, { code });
      return;
    }
    actionRunMeta.textContent = t().prompts.actionOk;
  } catch (e) {
    actionRunMeta.textContent = fmt(t().prompts.actionException, { msg: e.message });
  }
}

function render(payload) {
  currentPayload = payload;
  const groups = Array.isArray(payload.groups) ? payload.groups : [];
  const risk = riskFilter.value;
  const groupOnly = groupFilter.value;
  const filteredGroups = groups
    .filter((g) => groupOnly === "all" || g.type === groupOnly)
    .map((g) => ({
      ...g,
      items: (Array.isArray(g.items) ? g.items : []).filter(
        (it) => risk === "all" || (it.risk_level || "low") === risk
      )
    }))
    .filter((g) => g.items.length > 0);

  groupsRoot.innerHTML = "";
  emptyState.classList.add("hidden");
  emptyState.textContent = "";
  resultMeta.textContent = fmt(t().prompts.resultMeta, {
    query: payload.query || "",
    count: filteredGroups.length
  });

  if (!filteredGroups.length) {
    emptyState.textContent = t().prompts.resultEmpty;
    emptyState.classList.remove("hidden");
    return;
  }

  filteredGroups.forEach((group) => {
    const wrap = document.createElement("section");
    wrap.className = "overflow-hidden rounded-xl border border-stone-300 bg-white";

    const head = document.createElement("div");
    head.className = "border-b border-stone-300 bg-stone-100 px-3 py-2 text-sm font-bold";
    head.textContent = t().groupNames[group.type] || t().groupNames.unknown;

    const list = document.createElement("div");
    list.className = "grid gap-2 p-2.5";
    const items = Array.isArray(group.items) ? group.items : [];

    items.forEach((item) => {
      const card = document.createElement("article");
      card.className = "rounded-lg border border-stone-300 bg-white px-2.5 py-2.5 shadow-sm";

      const titleNode = document.createElement("div");
      titleNode.className = "mb-1 font-bold";
      titleNode.textContent = item.title || t().prompts.unnamed;

      const subtitleNode = document.createElement("div");
      subtitleNode.className = "mb-2 text-sm text-stone-500";
      subtitleNode.textContent = item.subtitle || "";

      const chips = document.createElement("div");
      chips.className = "mb-2 flex gap-2 text-xs";

      const riskChip = document.createElement("span");
      riskChip.className = `rounded-full border px-2 py-0.5 ${riskChipClass(item.risk_level)}`;
      const riskText = t().riskOptions[item.risk_level] || t().riskOptions.low;
      riskChip.textContent = `${t().labels.risk}: ${riskText}`;

      const confChip = document.createElement("span");
      confChip.className = "rounded-full border border-stone-300 px-2 py-0.5";
      confChip.textContent = `${t().labels.confidence}: ${item.confidence ?? "-"}`;

      const runBtn = document.createElement("button");
      runBtn.type = "button";
      runBtn.className = "rounded-md bg-teal-700 px-2.5 py-1 text-xs font-semibold text-white transition hover:brightness-95";
      runBtn.textContent = t().labels.execute;
      runBtn.addEventListener("click", () => runActionWithId(item.action_id, item.risk_level || "low"));

      chips.appendChild(riskChip);
      chips.appendChild(confChip);
      card.appendChild(titleNode);
      card.appendChild(subtitleNode);
      card.appendChild(chips);
      card.appendChild(runBtn);
      list.appendChild(card);
    });

    wrap.appendChild(head);
    wrap.appendChild(list);
    groupsRoot.appendChild(wrap);
  });
}

async function runLiveSearch() {
  const q = queryInput.value.trim();
  if (!q) {
    liveSearchMeta.textContent = t().prompts.enterQuery;
    return;
  }

  setSearchLoading(true);
  liveSearchMeta.textContent = t().prompts.searching;

  try {
    const res = await fetch(`/api/search?q=${encodeURIComponent(q)}`);
    const raw = await res.text();
    let payload = null;
    try {
      payload = JSON.parse(raw);
    } catch (_e) {
      const looksHtml = raw.trim().startsWith("<!DOCTYPE") || raw.trim().startsWith("<html");
      liveSearchMeta.textContent = looksHtml ? t().prompts.liveHtmlError : t().prompts.liveNonJsonError;
      return;
    }
    if (!res.ok) {
      liveSearchMeta.textContent = payload.error || t().prompts.liveFailed;
      return;
    }

    payloadInput.value = JSON.stringify(payload, null, 2);
    render(payload);
    const groups = Array.isArray(payload.groups) ? payload.groups.length : 0;
    liveSearchMeta.textContent = fmt(t().prompts.liveOk, { groups });
  } catch (e) {
    liveSearchMeta.textContent = fmt(t().prompts.liveException, { msg: e.message });
  } finally {
    setSearchLoading(false);
  }
}

function opPayload(op) {
  const q = queryInput.value.trim();
  if (op === "ai_recommend") {
    return { goal: q || "Rust development workstation" };
  }
  if (op === "ai_repair_plan") {
    return { software: q || "PowerToys", issue: "crash on launch after update" };
  }
  return {};
}

async function runQuickOp(op) {
  const name = t().actionButtons[op] || op;
  quickActionMeta.textContent = fmt(t().prompts.opRunning, { name });
  try {
    const res = await fetch("/api/op", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ op, payload: opPayload(op) })
    });
    const raw = await res.text();
    let data = null;
    try {
      data = JSON.parse(raw);
    } catch (_e) {
      const looksHtml = raw.trim().startsWith("<!DOCTYPE") || raw.trim().startsWith("<html");
      quickActionMeta.textContent = looksHtml ? t().prompts.opHtmlError : t().prompts.opNonJsonError;
      return;
    }

    if (!data.ok) {
      quickActionMeta.textContent = fmt(t().prompts.opFail, { name, code: data.exit_code ?? "?" });
      return;
    }

    quickActionMeta.textContent = fmt(t().prompts.opOk, { name });

    if (op === "ai_recommend" || op === "ai_analyze" || op === "ai_repair_plan") {
      // Keep a visible echo of backend json in advanced panel
      if (data.result) payloadInput.value = JSON.stringify(data.result, null, 2);
    }
  } catch (e) {
    quickActionMeta.textContent = fmt(t().prompts.opException, { msg: e.message });
  }
}

liveSearchBtn.addEventListener("click", runLiveSearch);
riskFilter.addEventListener("change", () => currentPayload && render(currentPayload));
groupFilter.addEventListener("change", () => currentPayload && render(currentPayload));
toggleFilterBtn.addEventListener("click", () => filterBar.classList.toggle("hidden"));
queryInput.addEventListener("keydown", (e) => {
  if (e.key === "Enter") runLiveSearch();
});
quickQueryButtons.forEach((btn) => {
  btn.addEventListener("click", () => {
    queryInput.value = btn.dataset.query || "";
    runLiveSearch();
  });
});
opButtons.forEach((btn) => {
  btn.addEventListener("click", () => runQuickOp(btn.dataset.op));
});

openSettingsBtn.addEventListener("click", () => settingsPanel.classList.remove("hidden"));
closeSettingsBtn.addEventListener("click", () => settingsPanel.classList.add("hidden"));
Object.values(settingInputs).forEach((el) => {
  el.addEventListener("change", saveSettings);
});

renderBtn.addEventListener("click", () => {
  payloadErr.textContent = "";
  let parsed;
  try {
    parsed = JSON.parse(payloadInput.value);
  } catch (e) {
    payloadErr.textContent = fmt(t().prompts.invalidJson, { msg: e.message });
    return;
  }
  render(parsed);
});

langZhBtn.addEventListener("click", () => setLanguage("zh"));
langEnBtn.addEventListener("click", () => setLanguage("en"));

payloadInput.value = JSON.stringify(examplePayload, null, 2);
applySettingsToUi();
render(examplePayload);
applyLanguage();
