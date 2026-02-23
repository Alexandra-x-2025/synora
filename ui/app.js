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
const capabilityRoot = document.getElementById("capabilityRoot");
const capabilityMeta = document.getElementById("capabilityMeta");
const groupsRoot = document.getElementById("groupsRoot");
const emptyState = document.getElementById("emptyState");
const resultMeta = document.getElementById("resultMeta");
const actionRunMeta = document.getElementById("actionRunMeta");
const langZhBtn = document.getElementById("langZhBtn");
const langEnBtn = document.getElementById("langEnBtn");
const advancedTitle = document.getElementById("advancedTitle");
const quickQueryButtons = document.querySelectorAll(".quick-query");

const LANG_KEY = "synora_ui_lang_v1";
let currentPayload = null;
let currentLang = localStorage.getItem(LANG_KEY) || "zh";

const I18N = {
  zh: {
    title: "Synora 中文界面",
    subtitle: "Phase 9：搜索与操作按钮界面",
    queryLabel: "搜索关键词",
    liveSearchBtn: "实时搜索",
    payloadTitle: "粘贴 `ui search --json` 输出",
    renderBtn: "渲染结果",
    capabilityTitle: "功能总览",
    capabilitySubtitle: "当前版本可用能力（按分类展示）",
    groupTitle: "结果分组",
    riskLabel: "风险等级",
    typeLabel: "分组类型",
    advancedTitle: "高级（开发调试）",
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
    capabilityGroups: {
      operations: "核心操作",
      security: "安全与治理",
      ai: "AI 能力",
      jobs: "任务系统",
      product: "产品入口与仓库"
    },
    prompts: {
      enterQuery: "请输入搜索关键词。",
      searching: "正在加载结果...",
      liveHtmlError: "实时搜索接口返回 HTML。请先执行：python scripts/ui_dev_server.py，然后访问 http://127.0.0.1:8787/",
      liveNonJsonError: "实时搜索返回了非 JSON 响应。",
      liveFailed: "实时搜索失败。",
      liveOk: "实时搜索成功：分组数={groups}",
      liveException: "实时搜索异常：{msg}",
      actionRunning: "执行中...",
      actionHtmlError: "动作接口返回 HTML。请先启动 ui_dev_server.py。",
      actionNonJsonError: "动作接口返回了非 JSON 响应。",
      actionFail: "动作失败（exit {code}）",
      actionOk: "动作执行成功。",
      actionNeedConfirm: "该动作是高风险操作，需要确认后执行，是否继续？",
      actionException: "动作执行异常：{msg}",
      unnamed: "（未命名）",
      invalidJson: "JSON 无效：{msg}",
      resultMeta: "关键词=\"{query}\" | 分组={count}",
      resultEmpty: "没有找到匹配结果，换个关键词试试。",
      filters: "筛选",
      capabilityMeta: "总计 {count} 项",
      statusAvailable: "已支持"
    },
    labels: {
      risk: "风险",
      confidence: "置信度",
      execute: "执行"
    }
  },
  en: {
    title: "Synora UI",
    subtitle: "Phase 9: search and action buttons",
    queryLabel: "Search Query",
    liveSearchBtn: "Live Search",
    payloadTitle: "Paste `ui search --json` Payload",
    renderBtn: "Render Results",
    capabilityTitle: "Feature Overview",
    capabilitySubtitle: "Capabilities available in current version",
    groupTitle: "Result Groups",
    riskLabel: "Risk Level",
    typeLabel: "Group Type",
    advancedTitle: "Advanced (Developer)",
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
    capabilityGroups: {
      operations: "Core Operations",
      security: "Security & Governance",
      ai: "AI Capabilities",
      jobs: "Task System",
      product: "Entry & Repositories"
    },
    prompts: {
      enterQuery: "Please enter a query.",
      searching: "Loading results...",
      liveHtmlError: "Live search endpoint returned HTML. Start server with: python scripts/ui_dev_server.py, then open http://127.0.0.1:8787/",
      liveNonJsonError: "Live search returned a non-JSON response.",
      liveFailed: "Live search failed.",
      liveOk: "Live search succeeded: groups={groups}",
      liveException: "Live search exception: {msg}",
      actionRunning: "Running...",
      actionHtmlError: "Action endpoint returned HTML. Start ui_dev_server.py first.",
      actionNonJsonError: "Action endpoint returned a non-JSON response.",
      actionFail: "Action failed (exit {code})",
      actionOk: "Action executed.",
      actionNeedConfirm: "This action is high risk and needs confirmation. Continue?",
      actionException: "Action exception: {msg}",
      unnamed: "(Untitled)",
      invalidJson: "Invalid JSON: {msg}",
      resultMeta: "query=\"{query}\" | groups={count}",
      resultEmpty: "No matching results. Try another query.",
      filters: "Filters",
      capabilityMeta: "{count} features",
      statusAvailable: "Available"
    },
    labels: {
      risk: "risk",
      confidence: "conf",
      execute: "Run"
    }
  }
};

const CAPABILITIES = [
  { group: "operations", key: "install_upgrade_uninstall" },
  { group: "operations", key: "updatable_detection" },
  { group: "security", key: "download_source_checks" },
  { group: "operations", key: "health_cleanup_repair" },
  { group: "operations", key: "software_discovery_library" },
  { group: "ai", key: "ai_analyze" },
  { group: "ai", key: "ai_recommend" },
  { group: "ai", key: "ai_repair_plan" },
  { group: "jobs", key: "local_queue_worker" },
  { group: "jobs", key: "scheduler_basics" },
  { group: "jobs", key: "download_mvp" },
  { group: "product", key: "global_search_ui" },
  { group: "product", key: "repository_mvp" },
  { group: "security", key: "security_gate_audit" },
  { group: "operations", key: "registry_only_discovery" }
];

const CAPABILITY_TEXT = {
  zh: {
    install_upgrade_uninstall: "一键安装/升级/卸载能力",
    updatable_detection: "可更新软件检测（手动触发或轻量定时）",
    download_source_checks: "下载源基础安全校验（签名/来源/白名单）",
    health_cleanup_repair: "软件体检、清理、修复最小安全子集",
    software_discovery_library: "软件自动发现并生成个人软件库（Software Discovery）",
    ai_analyze: "AI 软件整理分析（Analyze）",
    ai_recommend: "AI 场景化安装建议（Recommend）",
    ai_repair_plan: "AI 修复方案（Repair Plan，计划模式）",
    local_queue_worker: "本地任务队列（SQLite）与 Worker 基础框架",
    scheduler_basics: "Scheduler 基础能力（定时入队）",
    download_mvp: "下载模块 MVP（受控下载 + 校验 + 审计）",
    global_search_ui: "全局搜索 UI 主入口（Raycast 风格）",
    repository_mvp: "软件仓库系统 MVP（公共仓库只读 + 个人仓库管理）",
    security_gate_audit: "安全门禁与审计链路可用",
    registry_only_discovery: "Discovery 范围：Registry-only（已决策）"
  },
  en: {
    install_upgrade_uninstall: "One-click install / upgrade / uninstall",
    updatable_detection: "Updatable software detection (manual or lightweight schedule)",
    download_source_checks: "Basic download source checks (signature/source/allowlist)",
    health_cleanup_repair: "Software health check, cleanup, and minimal safe repair set",
    software_discovery_library: "Auto software discovery and personal software inventory generation",
    ai_analyze: "AI software analysis (Analyze)",
    ai_recommend: "AI scenario-based install recommendations (Recommend)",
    ai_repair_plan: "AI repair plan (plan-only mode)",
    local_queue_worker: "Local task queue (SQLite) and worker foundation",
    scheduler_basics: "Scheduler baseline capability (timed enqueue)",
    download_mvp: "Download module MVP (controlled download + verification + audit)",
    global_search_ui: "Global search UI entry (Raycast style)",
    repository_mvp: "Repository system MVP (public read-only + personal management)",
    security_gate_audit: "Security gate and audit chain available",
    registry_only_discovery: "Discovery scope: Registry-only (decided)"
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
  setText("payloadTitle", dict.payloadTitle);
  setText("renderBtn", dict.renderBtn);
  setText("capabilityTitle", dict.capabilityTitle);
  setText("capabilitySubtitle", dict.capabilitySubtitle);
  setText("groupTitle", dict.groupTitle);
  setText("riskLabel", dict.riskLabel);
  setText("typeLabel", dict.typeLabel);
  setText("advancedTitle", dict.advancedTitle);
  setText("toggleFilterBtn", dict.prompts.filters);

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
  renderCapabilities();
  if (currentPayload) render(currentPayload);
}

function setLanguage(lang) {
  currentLang = lang === "en" ? "en" : "zh";
  localStorage.setItem(LANG_KEY, currentLang);
  applyLanguage();
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

function renderCapabilities() {
  const dict = t();
  capabilityRoot.innerHTML = "";
  capabilityMeta.textContent = fmt(dict.prompts.capabilityMeta, { count: CAPABILITIES.length });

  const grouped = {};
  for (const item of CAPABILITIES) {
    if (!grouped[item.group]) grouped[item.group] = [];
    grouped[item.group].push(item);
  }

  Object.keys(grouped).forEach((groupKey) => {
    const wrap = document.createElement("section");
    wrap.className = "rounded-lg border border-stone-300 bg-white";

    const head = document.createElement("div");
    head.className = "border-b border-stone-300 bg-stone-100 px-3 py-2 text-sm font-semibold";
    head.textContent = dict.capabilityGroups[groupKey] || groupKey;

    const list = document.createElement("div");
    list.className = "grid gap-2 p-2.5";

    grouped[groupKey].forEach((item) => {
      const row = document.createElement("div");
      row.className = "flex items-start justify-between gap-2 rounded-md border border-stone-200 bg-white px-2 py-2";

      const text = document.createElement("div");
      text.className = "text-sm text-stone-700";
      text.textContent = CAPABILITY_TEXT[currentLang]?.[item.key] || item.key;

      const badge = document.createElement("span");
      badge.className = "whitespace-nowrap rounded-full border border-emerald-300 bg-emerald-50 px-2 py-0.5 text-xs font-semibold text-emerald-800";
      badge.textContent = dict.prompts.statusAvailable;

      row.appendChild(text);
      row.appendChild(badge);
      list.appendChild(row);
    });

    wrap.appendChild(head);
    wrap.appendChild(list);
    capabilityRoot.appendChild(wrap);
  });
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

liveSearchBtn.addEventListener("click", runLiveSearch);
riskFilter.addEventListener("change", () => currentPayload && render(currentPayload));
groupFilter.addEventListener("change", () => currentPayload && render(currentPayload));
toggleFilterBtn.addEventListener("click", () => {
  filterBar.classList.toggle("hidden");
});
queryInput.addEventListener("keydown", (e) => {
  if (e.key === "Enter") runLiveSearch();
});
quickQueryButtons.forEach((btn) => {
  btn.addEventListener("click", () => {
    queryInput.value = btn.dataset.query || "";
    runLiveSearch();
  });
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
render(examplePayload);
applyLanguage();
