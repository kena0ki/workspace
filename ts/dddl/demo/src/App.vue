<template>
  <Demo msg="Hello Vue 3 + TypeScript + Vite" />
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Demo from './components/Demo.vue';

export default defineComponent({
  name: 'App',
  components: {
    Demo
  }
});
</script>

<style>
</style>
