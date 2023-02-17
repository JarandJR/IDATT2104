<template>
  <p>main.rs</p>
  <form>
    <textarea placeholder='fn main() {
    println!("Hello, world!");
}'/>
  </form>
  <button @click="compile">Compile & Run</button>
</template>

<script setup lang="ts">
import {ref} from 'vue'

const count = ref(0);

function compile() {
  count.value++;
  console.log(`compiling ${count.value}`);
  run();
}

function run() {
  count.value *= 2;
  console.log(`result ${count.value}`);
}
</script>

<style scoped>
textarea {
  resize: none;
  width: 250px;
  height: 10rem;
}
</style>