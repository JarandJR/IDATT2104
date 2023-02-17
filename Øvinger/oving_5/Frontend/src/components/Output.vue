<template>
  <p>Output:</p>
  <div>
    <p id="output" >{{result}}</p>
  </div>
</template>

<script setup>
import {ref} from 'vue'

let result = ref("result...");

function printOutput(event) {
  result = "ur gat"
}
</script>

<style scoped>
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