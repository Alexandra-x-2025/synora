const queryInput = document.getElementById("queryInput");
const searchCmdBtn = document.getElementById("searchCmdBtn");
const liveSearchBtn = document.getElementById("liveSearchBtn");
const searchCmdOut = document.getElementById("searchCmdOut");
const liveSearchMeta = document.getElementById("liveSearchMeta");
const riskFilter = document.getElementById("riskFilter");
const groupFilter = document.getElementById("groupFilter");
const payloadInput = document.getElementById("payloadInput");
const renderBtn = document.getElementById("renderBtn");
const payloadErr = document.getElementById("payloadErr");
const groupsRoot = document.getElementById("groupsRoot");
const resultMeta = document.getElementById("resultMeta");
const actionCmdOut = document.getElementById("actionCmdOut");
const runActionBtn = document.getElementById("runActionBtn");
const copyActionBtn = document.getElementById("copyActionBtn");
const actionRunMeta = document.getElementById("actionRunMeta");
const historyRoot = document.getElementById("historyRoot");
const langZhBtn = document.getElementById("langZhBtn");
const langEnBtn = document.getElementById("langEnBtn");
const advancedTitle = document.getElementById("advancedTitle");

const HISTORY_KEY = "synora_ui_cmd_history_v1";
const HISTORY_MAX = 12;
const LANG_KEY = "synora_ui_lang_v1";
let currentPayload = null;
let currentLang = localStorage.getItem(LANG_KEY) || "zh";

const I18N = {
  zh: {
    title: "Synora 中文界面",
    subtitle: "Phase 9：`ui search` / `ui action-run` 可视化入口",
    queryLabel: "搜索关键词",
    searchCmdBtn: "生成搜索命令",
    liveSearchBtn: "实时搜索",
    payloadTitle: "粘贴 `ui search --json` 输出",
    renderBtn: "渲染结果",
    groupTitle: "结果分组",
    riskLabel: "风险等级",
    typeLabel: "分组类型",
    actionTitle: "动作命令",
    actionHint: "点击任意结果卡片生成命令。",
    runActionBtn: "通过 API 执行动作",
    copyActionBtn: "复制命令",
    historyTitle: "最近命令",
    advancedTitle: "高级（开发调试）",
    queryPlaceholder: "输入关键词，如 PowerToys",
    payloadPlaceholder: "粘贴 ui search --json 的完整输出",
    riskOptions: {
      all: "全部",
      low: "低",
      medium: "中",
      high: "高"
    },
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
    prompts: {
      enterQuery: "请输入搜索关键词。",
      searching: "搜索中...",
      liveHtmlError: "实时搜索接口返回 HTML。请先执行：python scripts/ui_dev_server.py，然后访问 http://127.0.0.1:8787/",
      liveNonJsonError: "实时搜索返回了非 JSON 响应。",
      liveFailed: "实时搜索失败。",
      liveOk: "实时搜索成功：分组数={groups}",
      liveException: "实时搜索异常：{msg}",
      pickActionFirst: "请先选择一个动作命令。",
      parseActionIdFail: "无法从命令中解析 action id。",
      actionRunning: "执行中...",
      actionHtmlError: "动作接口返回 HTML。请先启动 ui_dev_server.py。",
      actionNonJsonError: "动作接口返回了非 JSON 响应。",
      actionFail: "动作失败（exit {code}）",
      actionOk: "动作执行成功。",
      actionException: "动作执行异常：{msg}",
      unnamed: "（未命名）",
      noHistory: "暂无命令历史。",
      copied: "已复制。",
      copyFailed: "复制失败。",
      noCopyContent: "没有可复制的命令。",
      invalidJson: "JSON 无效：{msg}",
      resultMeta: "关键词=\"{query}\" | 分组={count}"
    },
    labels: {
      risk: "风险",
      confidence: "置信度",
      copy: "复制",
      execute: "执行"
    }
  },
  en: {
    title: "Synora UI",
    subtitle: "Phase 9 visual entry for `ui search` / `ui action-run`",
    queryLabel: "Search Query",
    searchCmdBtn: "Build Search Command",
    liveSearchBtn: "Live Search",
    payloadTitle: "Paste `ui search --json` Payload",
    renderBtn: "Render Results",
    groupTitle: "Result Groups",
    riskLabel: "Risk Level",
    typeLabel: "Group Type",
    actionTitle: "Action Command",
    actionHint: "Click any result card to generate a command.",
    runActionBtn: "Run Action via API",
    copyActionBtn: "Copy Command",
    historyTitle: "Recent Commands",
    advancedTitle: "Advanced (Developer)",
    queryPlaceholder: "Type keyword, e.g. PowerToys",
    payloadPlaceholder: "Paste full JSON output from ui search --json",
    riskOptions: {
      all: "All",
      low: "Low",
      medium: "Medium",
      high: "High"
    },
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
    prompts: {
      enterQuery: "Please enter a query.",
      searching: "Searching...",
      liveHtmlError: "Live search endpoint returned HTML. Start server with: python scripts/ui_dev_server.py, then open http://127.0.0.1:8787/",
      liveNonJsonError: "Live search returned a non-JSON response.",
      liveFailed: "Live search failed.",
      liveOk: "Live search succeeded: groups={groups}",
      liveException: "Live search exception: {msg}",
      pickActionFirst: "Please select an action command first.",
      parseActionIdFail: "Cannot parse action id from command.",
      actionRunning: "Running...",
      actionHtmlError: "Action endpoint returned HTML. Start ui_dev_server.py first.",
      actionNonJsonError: "Action endpoint returned a non-JSON response.",
      actionFail: "Action failed (exit {code})",
      actionOk: "Action executed.",
      actionNeedConfirm: "This action is high risk and needs confirmation. Continue?",
      actionException: "Action exception: {msg}",
      unnamed: "(Untitled)",
      noHistory: "No command history yet.",
      copied: "Copied.",
      copyFailed: "Copy failed.",
      noCopyContent: "No command to copy.",
      invalidJson: "Invalid JSON: {msg}",
      resultMeta: "query=\"{query}\" | groups={count}"
    },
    labels: {
      risk: "risk",
      confidence: "conf",
      copy: "Copy",
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
    },
    {
      type: "source",
      items: [
        {
          title: "PowerToys (Preview) x64",
          subtitle: "github.com | https://github.com/microsoft/winget-pkgs",
          risk_level: "low",
          confidence: 72,
          action_id: "source.registry:180"
        }
      ]
    },
    {
      type: "ai",
      items: [
        {
          title: "AI Repair Plan PowerToys",
          subtitle: "crash on launch after update",
          risk_level: "high",
          confidence: 78,
          action_id: "ai.repair-plan:repair-plan-1771782518986013100-0-1771782518"
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
  setText("searchCmdBtn", dict.searchCmdBtn);
  setText("liveSearchBtn", dict.liveSearchBtn);
  setText("payloadTitle", dict.payloadTitle);
  setText("renderBtn", dict.renderBtn);
  setText("groupTitle", dict.groupTitle);
  setText("riskLabel", dict.riskLabel);
  setText("typeLabel", dict.typeLabel);
  setText("actionTitle", dict.actionTitle);
  setText("actionHint", dict.actionHint);
  setText("runActionBtn", dict.runActionBtn);
  setText("copyActionBtn", dict.copyActionBtn);
  setText("historyTitle", dict.historyTitle);
  setText("advancedTitle", dict.advancedTitle);

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
  renderHistory();
  if (currentPayload) render(currentPayload);
}

function setLanguage(lang) {
  currentLang = lang === "en" ? "en" : "zh";
  localStorage.setItem(LANG_KEY, currentLang);
  applyLanguage();
}

function loadHistory() {
  try {
    return JSON.parse(localStorage.getItem(HISTORY_KEY) || "[]");
  } catch (_e) {
    return [];
  }
}

function saveHistory(items) {
  localStorage.setItem(HISTORY_KEY, JSON.stringify(items.slice(0, HISTORY_MAX)));
}

function pushHistory(cmd) {
  const clean = String(cmd || "").trim();
  if (!clean) return;
  const existing = loadHistory().filter((x) => x.cmd !== clean);
  existing.unshift({ cmd: clean, ts: Date.now() });
  saveHistory(existing);
  renderHistory();
}

function copyText(text) {
  const clean = String(text || "").trim();
  if (!clean) return Promise.resolve(false);
  if (navigator.clipboard && navigator.clipboard.writeText) {
    return navigator.clipboard.writeText(clean).then(() => true).catch(() => false);
  }
  const ta = document.createElement("textarea");
  ta.value = clean;
  document.body.appendChild(ta);
  ta.select();
  const ok = document.execCommand("copy");
  document.body.removeChild(ta);
  return Promise.resolve(ok);
}

function makeSearchCmd(record = true) {
  const q = queryInput.value.trim();
  if (!q) {
    searchCmdOut.textContent = t().prompts.enterQuery;
    return;
  }
  const cmd = `cargo run -- ui search --q "${q.replaceAll('"', '\\"')}" --json`;
  searchCmdOut.textContent = cmd;
  if (record) pushHistory(cmd);
}

async function runLiveSearch() {
  const q = queryInput.value.trim();
  if (!q) {
    liveSearchMeta.textContent = t().prompts.enterQuery;
    return;
  }
  makeSearchCmd(false);
  liveSearchMeta.textContent = t().prompts.searching;
  try {
    const url = `/api/search?q=${encodeURIComponent(q)}`;
    const res = await fetch(url);
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
    currentPayload = payload;
    render(payload);
    const groups = Array.isArray(payload.groups) ? payload.groups.length : 0;
    liveSearchMeta.textContent = fmt(t().prompts.liveOk, { groups });
  } catch (e) {
    liveSearchMeta.textContent = fmt(t().prompts.liveException, { msg: e.message });
  }
}

function riskChipClass(risk) {
  if (risk === "high") return "text-red-800 border-red-300 bg-red-50";
  if (risk === "medium") return "text-amber-800 border-amber-300 bg-amber-50";
  return "text-green-800 border-green-300 bg-green-50";
}

function makeActionCmd(actionId, risk) {
  if (!actionId) return "";
  if (risk === "high") {
    return `cargo run -- ui action-run --id "${actionId}" --confirm --json`;
  }
  return `cargo run -- ui action-run --id "${actionId}" --json`;
}

async function runActionViaApi() {
  const cmd = actionCmdOut.textContent.trim();
  if (!cmd) {
    actionRunMeta.textContent = t().prompts.pickActionFirst;
    return;
  }
  const match = cmd.match(/--id "([^"]+)"/);
  if (!match) {
    actionRunMeta.textContent = t().prompts.parseActionIdFail;
    return;
  }
  const actionId = match[1];
  const confirm = cmd.includes("--confirm");
  pushHistory(cmd);
  actionRunMeta.textContent = t().prompts.actionRunning;
  try {
    const res = await fetch("/api/action-run", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ id: actionId, confirm })
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

async function runActionWithId(actionId, riskLevel) {
  const confirmNeeded = riskLevel === "high";
  if (confirmNeeded) {
    const ok = window.confirm(
      currentLang === "zh"
        ? "该动作是高风险操作，需要确认后执行，是否继续？"
        : t().prompts.actionNeedConfirm
    );
    if (!ok) return;
  }

  const cmd = makeActionCmd(actionId, riskLevel);
  actionCmdOut.textContent = cmd;
  pushHistory(cmd);
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
  resultMeta.textContent = fmt(t().prompts.resultMeta, {
    query: payload.query || "",
    count: filteredGroups.length
  });

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
      card.className = "rounded-lg border border-stone-300 bg-white px-2.5 py-2.5 shadow-sm transition hover:-translate-y-0.5 hover:border-stone-400";
      const btn = document.createElement("button");
      btn.className = "w-full bg-transparent p-0 text-left text-inherit";
      btn.type = "button";
      btn.addEventListener("click", () => {
        const cmd = makeActionCmd(item.action_id, item.risk_level);
        actionCmdOut.textContent = cmd;
        pushHistory(cmd);
      });

      const tNode = document.createElement("div");
      tNode.className = "mb-1 font-bold";
      tNode.textContent = item.title || t().prompts.unnamed;

      const s = document.createElement("div");
      s.className = "mb-2 text-sm text-stone-500";
      s.textContent = item.subtitle || "";

      const chips = document.createElement("div");
      chips.className = "flex gap-2 text-xs";

      const riskChip = document.createElement("span");
      riskChip.className = `rounded-full border px-2 py-0.5 ${riskChipClass(item.risk_level)}`;
      const riskText = t().riskOptions[item.risk_level] || t().riskOptions.low;
      riskChip.textContent = `${t().labels.risk}: ${riskText}`;

      const confChip = document.createElement("span");
      confChip.className = "rounded-full border border-stone-300 px-2 py-0.5";
      confChip.textContent = `${t().labels.confidence}: ${item.confidence ?? "-"}`;

      chips.appendChild(riskChip);
      chips.appendChild(confChip);

      const actions = document.createElement("div");
      actions.className = "mt-2";
      const runBtn = document.createElement("button");
      runBtn.type = "button";
      runBtn.className = "rounded-md bg-teal-700 px-2.5 py-1 text-xs font-semibold text-white transition hover:brightness-95";
      runBtn.textContent = t().labels.execute;
      runBtn.addEventListener("click", (e) => {
        e.preventDefault();
        e.stopPropagation();
        runActionWithId(item.action_id, item.risk_level || "low");
      });
      actions.appendChild(runBtn);

      btn.appendChild(tNode);
      btn.appendChild(s);
      btn.appendChild(chips);
      btn.appendChild(actions);
      card.appendChild(btn);
      list.appendChild(card);
    });

    wrap.appendChild(head);
    wrap.appendChild(list);
    groupsRoot.appendChild(wrap);
  });
}

function renderHistory() {
  historyRoot.innerHTML = "";
  const items = loadHistory();
  if (!items.length) {
    const p = document.createElement("p");
    p.className = "text-sm text-stone-500";
    p.textContent = t().prompts.noHistory;
    historyRoot.appendChild(p);
    return;
  }
  items.slice(0, 8).forEach((item) => {
    const row = document.createElement("div");
    row.className = "flex flex-wrap items-center gap-2 rounded-lg border border-stone-300 bg-white px-2 py-1.5";
    const code = document.createElement("code");
    code.className = "min-w-[280px] flex-1 font-mono text-xs";
    code.textContent = item.cmd;
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "rounded-md bg-stone-800 px-2 py-1 text-xs font-semibold text-white";
    btn.textContent = t().labels.copy;
    btn.addEventListener("click", async () => {
      const ok = await copyText(item.cmd);
      actionRunMeta.textContent = ok ? t().prompts.copied : t().prompts.copyFailed;
    });
    row.appendChild(code);
    row.appendChild(btn);
    historyRoot.appendChild(row);
  });
}

searchCmdBtn.addEventListener("click", makeSearchCmd);
liveSearchBtn.addEventListener("click", runLiveSearch);
riskFilter.addEventListener("change", () => currentPayload && render(currentPayload));
groupFilter.addEventListener("change", () => currentPayload && render(currentPayload));
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
runActionBtn.addEventListener("click", runActionViaApi);
copyActionBtn.addEventListener("click", async () => {
  const cmd = actionCmdOut.textContent;
  const ok = await copyText(cmd);
  actionRunMeta.textContent = ok ? t().prompts.copied : t().prompts.noCopyContent;
});
langZhBtn.addEventListener("click", () => setLanguage("zh"));
langEnBtn.addEventListener("click", () => setLanguage("en"));

payloadInput.value = JSON.stringify(examplePayload, null, 2);
makeSearchCmd();
render(examplePayload);
applyLanguage();
