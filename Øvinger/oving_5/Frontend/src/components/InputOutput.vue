<template>
  <p>main.rs</p>
  <form>
    <textarea v-model = "input" />
  </form>
  <button @click="compile">Compile & Run</button>
  <p>Output:</p>
  <div>
    <p id="output" >{{result}}</p>
  </div>
</template>

<script setup lang="ts">
import {ref} from 'vue'
import axios from "axios";

const input = ref(`fn main() {println!('Hello, world!');}`);
const result = ref("");

async function compile() {
  result.value = "No connection with service";
  console.log(input.value);
  result.value = await (await axios.post("http://127.0.0.1:7878/post_code", input.value)).data;
}
</script>

<style scoped>
textarea {
  resize: none;
  width: 250px;
  height: 10rem;
}

div {
  height: 20rem;
  width: 100%;
  background-color: lightskyblue;
}

#output {
  /*Does nothing???*/
  border-width: 10px;
  border-color: #2c3e50;
}
</style>