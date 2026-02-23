const queryInput = document.getElementById("queryInput");
const searchCmdBtn = document.getElementById("searchCmdBtn");
const liveSearchBtn = document.getElementById("liveSearchBtn");
const searchCmdOut = document.getElementById("searchCmdOut");
const liveSearchMeta = document.getElementById("liveSearchMeta");
const payloadInput = document.getElementById("payloadInput");
const renderBtn = document.getElementById("renderBtn");
const payloadErr = document.getElementById("payloadErr");
const groupsRoot = document.getElementById("groupsRoot");
const resultMeta = document.getElementById("resultMeta");
const actionCmdOut = document.getElementById("actionCmdOut");
const runActionBtn = document.getElementById("runActionBtn");
const actionRunMeta = document.getElementById("actionRunMeta");

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

function makeSearchCmd() {
  const q = queryInput.value.trim();
  if (!q) {
    searchCmdOut.textContent = "Query is required.";
    return;
  }
  searchCmdOut.textContent = `cargo run -- ui search --q "${q.replaceAll('"', '\\"')}" --json`;
}

async function runLiveSearch() {
  const q = queryInput.value.trim();
  if (!q) {
    liveSearchMeta.textContent = "Query is required.";
    return;
  }
  liveSearchMeta.textContent = "Searching...";
  try {
    const url = `/api/search?q=${encodeURIComponent(q)}`;
    const res = await fetch(url);
    const payload = await res.json();
    if (!res.ok) {
      liveSearchMeta.textContent = payload.error || "Live search failed.";
      return;
    }
    payloadInput.value = JSON.stringify(payload, null, 2);
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
  actionRunMeta.textContent = "Running...";
  try {
    const res = await fetch("/api/action-run", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ id: actionId, confirm })
    });
    const payload = await res.json();
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
  const groups = Array.isArray(payload.groups) ? payload.groups : [];
  groupsRoot.innerHTML = "";
  resultMeta.textContent = `query="${payload.query || ""}" | groups=${groups.length}`;

  groups.forEach((group) => {
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
        actionCmdOut.textContent = makeActionCmd(item.action_id, item.risk_level);
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

searchCmdBtn.addEventListener("click", makeSearchCmd);
liveSearchBtn.addEventListener("click", runLiveSearch);
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

payloadInput.value = JSON.stringify(examplePayload, null, 2);
makeSearchCmd();
render(examplePayload);
