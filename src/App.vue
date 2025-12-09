<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

const files = ref<string[]>([]);

async function test(){
    invoke('process_file_array', { filePaths: files.value })
      .then((message) => {
        console.log("Response from Rust:", message);
      })
      .catch((error) => {
        console.error("Error invoking Rust command:", error);
      });
}

async function selectImages() {
  await open({
    multiple: true,
    filters: [{ name: "Images", extensions: ['avif', 'bmp', 'dds', 'exr', 'ff', 'gif', 'hdr', 'ico', 'jpeg', 'png', 'pnm', 'qoi', 'tga', 'tiff', 'webp'] }],
  })
    .then((selected) => {
      if (Array.isArray(selected)) {
        console.log("Selected files:", selected);
        files.value = selected;
      } else if (selected === null) {
        console.log("User cancelled the selection");
        files.value = [];
      } else {
        console.log("Selected file:", selected);
        files.value = [selected];
      }
    })
    .catch((error) => {
      console.error("Error selecting files:", error);
      return [];
    });
}
</script>

<template>
  <main>
    <h1>JXL File Compressor - jpegxl-rs</h1>
    <button @click="selectImages">Select Images</button>
    <button @click="test" :disabled="files.length == 0? true: false">Process Files</button>
    <ul>
      <li v-for="file in files" :key="file">{{ file }}</li>
    </ul>
  </main>
</template>

<style scoped>
h1 {
  background-color: lightblue;
}
</style>
