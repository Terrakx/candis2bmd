const { invoke } = window.__TAURI__.core;
const { open, save } = window.__TAURI__.dialog;

let invoices = [];

const logArea = document.getElementById("logArea");
const invoiceTable = document.getElementById("invoiceTable");
const scanBtn = document.getElementById("scanBtn");
const exportBtn = document.getElementById("exportBtn");
const browseSource = document.getElementById("browseSource");
const browseZip = document.getElementById("browseZip");
const browseTarget = document.getElementById("browseTarget");
const sourceDirInput = document.getElementById("sourceDir");
const targetFileInput = document.getElementById("targetFile");
const nextBelegNrInput = document.getElementById("nextBelegNr");
const belegSymbolInput = document.getElementById("belegSymbol");
const selectAllCheckbox = document.getElementById("selectAll");

function log(message, type = "info") {
  const time = new Date().toLocaleTimeString();
  const entry = document.createElement("div");
  entry.className = "log-entry";
  
  const spanTime = document.createElement("span");
  spanTime.className = "log-time";
  spanTime.textContent = `[${time}]`;
  
  const spanMsg = document.createElement("span");
  spanMsg.className = `log-message ${type === "success" ? "log-success" : type === "error" ? "log-error" : ""}`;
  spanMsg.textContent = message;
  
  entry.appendChild(spanTime);
  entry.appendChild(spanMsg);
  logArea.appendChild(entry);
  logArea.scrollTop = logArea.scrollHeight;
}

async function scan() {
  const folder = sourceDirInput.value;
  if (!folder) return;
  
  log(`Scanne nach Belegen in ${folder}...`);
  try {
    invoices = await invoke("scan_folder", { folderPath: folder });
    renderTable();
    log(`Erfolgreich ${invoices.length} Belege gefunden.`, "success");
  } catch (err) {
    log(`Fehler beim Scannen: ${err}`, "error");
  }
}

function renderTable() {
  invoiceTable.innerHTML = "";
  invoices.forEach((inv, index) => {
    const hasPdf = inv.pdf_filename && inv.pdf_filename.toLowerCase().endsWith(".pdf");
    const row = document.createElement("tr");
    row.innerHTML = `
      <td><input type="checkbox" ${inv.selected ? 'checked' : ''} id="check-${index}"></td>
      <td>${inv.consolidated_invoice_id}</td>
      <td>${inv.entries[0]?.supplier_name || 'Unbekannt'}</td>
      <td>${inv.consolidated_amount.toFixed(2).replace('.', ',')} €</td>
      <td style="text-align: center;">${hasPdf ? '<span style="color: #4CAF50; font-weight: bold; font-size: 1.2em;">✓</span>' : '-'}</td>
      <td>${inv.status}</td>
    `;
    invoiceTable.appendChild(row);
    
    document.getElementById(`check-${index}`).addEventListener("change", (e) => {
      invoices[index].selected = e.target.checked;
    });
  });
}

async function exportCSV() {
  const target = targetFileInput.value;
  if (!target) {
    log("Bitte Ziel-Datei wählen", "error");
    return;
  }
  
  const startNr = parseInt(nextBelegNrInput.value) || 1;
  const symbol = belegSymbolInput.value || "ER";
  
  log(`Generiere CSV (Symbol: ${symbol}, ab BelegNr ${startNr})...`);
  
  try {
    const result = await invoke("generate_csv", { 
      invoices, 
      targetFile: target, 
      startNr,
      symbol
    });
    log(result, "success");
    
    // Update next number locally for convenience
    const selectedCount = invoices.filter(i => i.selected).length;
    nextBelegNrInput.value = startNr + selectedCount;
  } catch (err) {
    log(`Export-Fehler: ${err}`, "error");
  }
}

browseSource.addEventListener("click", async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Quellverzeichnis wählen",
      defaultPath: sourceDirInput.value || undefined
    });
    if (selected) {
      sourceDirInput.value = selected;
      targetFileInput.value = `${selected}\\bmd_import.csv`;
      log(`Quellpfad gesetzt: ${selected}`);
      // Auto scan
      scan();
    }
  } catch (e) {
    log("Dialog-Fehler (Source): " + e, "error");
  }
});

browseZip.addEventListener("click", async () => {
  try {
    const selected = await open({
      directory: false,
      multiple: false,
      title: "Zip-Datei wählen",
      filters: [{ name: "Zip Archiv", extensions: ["zip"] }]
    });
    if (selected) {
      log(`Entpacke Zip-Datei: ${selected}...`);
      try {
        const extractedDir = await invoke("extract_zip", { zipPath: selected });
        sourceDirInput.value = extractedDir;
        targetFileInput.value = `${extractedDir}\\bmd_import.csv`;
        log(`Erfolgreich entpackt nach: ${extractedDir}`, "success");
        scan();
      } catch (err) {
        log(`Fehler beim Entpacken: ${err}`, "error");
      }
    }
  } catch (e) {
    log("Dialog-Fehler (Zip): " + e, "error");
  }
});

browseTarget.addEventListener("click", async () => {
  try {
    const selected = await save({
      filters: [{ name: "BMD CSV", extensions: ["csv"] }],
      title: "Export-Datei speichern",
      defaultPath: targetFileInput.value || undefined
    });
    if (selected) {
      targetFileInput.value = selected;
      log(`Zielpfad gesetzt: ${selected}`);
    }
  } catch (e) {
    log("Dialog-Fehler (Target): " + e, "error");
  }
});

scanBtn.addEventListener("click", scan);
exportBtn.addEventListener("click", exportCSV);

selectAllCheckbox.addEventListener("change", (e) => {
  invoices.forEach(inv => inv.selected = e.target.checked);
  renderTable();
});

// Init
(async () => {
  try {
    const lastNr = await invoke("get_last_beleg_nr");
    const baseDir = await invoke("get_base_dir");
    
    nextBelegNrInput.value = lastNr + 1;
    sourceDirInput.value = baseDir;
    targetFileInput.value = `${baseDir}\\bmd_import.csv`;
    
    log(`Backend-Anbindung bereit.`);
    // Auto scan on start
    scan();
  } catch (e) {
    log("Initialisierungsfehler.", "error");
    console.error(e);
  }
})();
