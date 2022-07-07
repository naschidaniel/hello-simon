<template>
  <h1>Hallo Simon</h1>
  <button @click="counter++">Klick da mal</button>
  <p>Du hast {{ counter }} mal geklickt.</p>
  <button @click="writeReport">Bericht als PDF speichern</button>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { writeBinaryFile } from "@tauri-apps/api/fs";
import { message } from "@tauri-apps/api/dialog";

import { save } from "@tauri-apps/api/dialog";
import { downloadDir } from "@tauri-apps/api/path";

const counter = ref(0);

async function writeReport() {
  const report = `Du hast ${counter.value} mal geklickt`;
  const reportPdf: Uint8Array = await invoke("gen_pdf_report", {
    title: "Klick Report",
    report,
  });
  try {
    const now = new Date();
    const fileExtension = now
      .toISOString()
      .replace("T", "")
      .replaceAll("-", "")
      .replaceAll(":", "")
      .split(".")[0];
    const filePath = await save({
      defaultPath: `${await downloadDir()}Klick_Report_${fileExtension}.pdf`,
    });
    await writeBinaryFile(filePath, reportPdf, {});
    await message(
      `Der Bericht wurde erfolgreich in ${filePath} gespeichert`,
      "Hello Simon"
    );
  } catch {
    await message("Der Report konnte nicht gespeichert werden", {
      title: "Hello Simon",
      type: "error",
    });
  }
}
</script>

<style scoped>
a {
  color: #42b983;
}

label {
  margin: 0 0.5em;
  font-weight: bold;
}

code {
  background-color: #eee;
  padding: 2px 4px;
  border-radius: 4px;
  color: #304455;
}
</style>
