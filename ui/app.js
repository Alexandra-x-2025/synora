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

const HISTORY_KEY = "synora_ui_cmd_history_v1";
const HISTORY_MAX = 12;
let currentPayload = null;

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
    searchCmdOut.textContent = "Query is required.";
    return;
  }
  const cmd = `cargo run -- ui search --q "${q.replaceAll('"', '\\"')}" --json`;
  searchCmdOut.textContent = cmd;
  if (record) pushHistory(cmd);
}

async function runLiveSearch() {
  const q = queryInput.value.trim();
  if (!q) {
    liveSearchMeta.textContent = "Query is required.";
    return;
  }
  makeSearchCmd(false);
  liveSearchMeta.textContent = "Searching...";
  try {
    const url = `/api/search?q=${encodeURIComponent(q)}`;
    const res = await fetch(url);
    const raw = await res.text();
    let payload = null;
    try {
      payload = JSON.parse(raw);
    } catch (_e) {
      const looksHtml = raw.trim().startsWith("<!DOCTYPE") || raw.trim().startsWith("<html");
      if (looksHtml) {
        liveSearchMeta.textContent =
          "Live search endpoint returned HTML. Use: python scripts/ui_dev_server.py and open http://127.0.0.1:8787/";
      } else {
        liveSearchMeta.textContent = "Live search returned non-JSON response.";
      }
      return;
    }
    if (!res.ok) {
      liveSearchMeta.textContent = payload.error || "Live search failed.";
      return;
    }
    payloadInput.value = JSON.stringify(payload, null, 2);
    currentPayload = payload;
    render(payload);
    const groups = Array.isArray(payload.groups) ? payload.groups.length : 0;
    liveSearchMeta.textContent = `Live search ok: groups=${groups}`;
  } catch (e) {
    liveSearchMeta.textContent = `Live search error: ${e.message}`;
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
    actionRunMeta.textContent = "No action command selected.";
    return;
  }
  const match = cmd.match(/--id "([^"]+)"/);
  if (!match) {
    actionRunMeta.textContent = "Cannot parse action id from command.";
    return;
  }
  const actionId = match[1];
  const confirm = cmd.includes("--confirm");
  pushHistory(cmd);
  actionRunMeta.textContent = "Running...";
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
      actionRunMeta.textContent = looksHtml
        ? "Action API returned HTML. Start ui_dev_server.py first."
        : "Action API returned non-JSON response.";
      return;
    }
    if (!payload.ok) {
      const code = payload.exit_code ?? "?";
      actionRunMeta.textContent = `Action failed (exit ${code})`;
      return;
    }
    actionRunMeta.textContent = "Action executed.";
  } catch (e) {
    actionRunMeta.textContent = `Action error: ${e.message}`;
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
  resultMeta.textContent = `query="${payload.query || ""}" | groups=${filteredGroups.length}`;

  filteredGroups.forEach((group) => {
    const wrap = document.createElement("section");
    wrap.className = "overflow-hidden rounded-xl border border-stone-300 bg-white";

    const head = document.createElement("div");
    head.className = "border-b border-stone-300 bg-stone-100 px-3 py-2 text-sm font-bold capitalize";
    head.textContent = group.type || "unknown";

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

      const t = document.createElement("div");
      t.className = "mb-1 font-bold";
      t.textContent = item.title || "(untitled)";

      const s = document.createElement("div");
      s.className = "mb-2 text-sm text-stone-500";
      s.textContent = item.subtitle || "";

      const chips = document.createElement("div");
      chips.className = "flex gap-2 text-xs";

      const riskChip = document.createElement("span");
      riskChip.className = `rounded-full border px-2 py-0.5 ${riskChipClass(item.risk_level)}`;
      riskChip.textContent = `risk: ${item.risk_level || "low"}`;

      const confChip = document.createElement("span");
      confChip.className = "rounded-full border border-stone-300 px-2 py-0.5";
      confChip.textContent = `conf: ${item.confidence ?? "-"}`;

      chips.appendChild(riskChip);
      chips.appendChild(confChip);
      btn.appendChild(t);
      btn.appendChild(s);
      btn.appendChild(chips);
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
    p.textContent = "No command history yet.";
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
    btn.textContent = "Copy";
    btn.addEventListener("click", async () => {
      const ok = await copyText(item.cmd);
      actionRunMeta.textContent = ok ? "Copied." : "Copy failed.";
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
    payloadErr.textContent = `Invalid JSON: ${e.message}`;
    return;
  }
  render(parsed);
});
runActionBtn.addEventListener("click", runActionViaApi);
copyActionBtn.addEventListener("click", async () => {
  const ok = await copyText(actionCmdOut.textContent);
  actionRunMeta.textContent = ok ? "Copied." : "Nothing to copy.";
});

payloadInput.value = JSON.stringify(examplePayload, null, 2);
makeSearchCmd();
render(examplePayload);
renderHistory();
