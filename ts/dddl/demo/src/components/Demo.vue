<template>
  <div class="header-container dummy">
    <h1 class="header">dummy</h1>
  </div>
  <div class="header-container">
    <h1 class="header">DDDL</h1>
  </div>
  <a href="https://github.com/kena0ki/adima">
    <img class="gh-ribbon" src="../assets/forkme_right_red_aa0000.svg">
  </a>
  <div class="input-area">
    <h2>Input</h2>
    <p class="instruction">Step1. Input a create statement here.</p>
    <div class="input-area-create-statement-container">
      <textarea class="input-area-create-statement" />
    </div>
    <div class="input-area-button-container">
      <button class="pure-button input-area-button">Parse</button>
    </div>
  </div>
  <hr>
  <div class="options-area">
    <div class="options-header">
      <h2>Options</h2>
      <p>Step2. Set options for data to be generated.</p>
    </div>
    <form class="options-form pure-form">
      <h3 class="general-options-header">General options</h3>
      <div class="general-options">
        <label class="options label" for="options-output-format">Output format:</label>
        <select id="options-output-format" class="options select">
          <option>Csv</option>
          <option>Insert statement</option>
        </select>
        <h4>Column option default for each type</h4>
        <div class="column-option-default">
          <template v-for="n in 4" :key="n">
            <label class="options label" for="options-column-option-default-type">Type:</label>
            <select id="options-column-option-default-type" class="options select">
              <option>Numeric type</option>
              <option>String type</option>
            </select>
            <template v-if="type instanceof NumericType">
              <div class="column-option-default-numeric">
                <label class="options label" for="column-option-default-numeric">Step:</label>
                <input id="column-option-default-numeric" class="options input">
              </div>
            </template>
          </template>
          <label class="options label" for="options-column-option-default-type">Type:</label>
          <select id="options-column-option-default-type" class="options select">
            <option>Numeric type</option>
            <option>String type</option>
          </select>
        </div>
        <h4>Column options</h4>
        <div class="column-option-default">
          <label class="options label" for="options-column-option-default-type">Column name:</label>
          <select id="options-column-option-default-type" class="options select">
            <option>COL_1</option>
            <option>COL_2</option>
          </select>
        </div>
      </div>
    </form>
  </div>
  <hr>
  <div class="output-area">
    <div class="output-header">
      <h2>Output</h2>
      <p>Step3. Now, it's time to generate data.</p>
    </div>
    <div class="output-area-button-container">
      <button class="pure-button output-area-button">Generate</button>
    </div>
  </div>
  <h3 class="settings-title">Settings</h3>
  <div class="settings-container">
    <div class="settings settings-height-container">
      <div class="settings-height-sub-container sub-container-settings-label-value">
        <span class="settings-height settings-label">Height: </span>
        <span class="settings-height settings-value">6</span>
      </div>
      <div class="settings-height-sub-container sub-container-settings-input">
        <input class="settings-height settings-input" type="range" min="200" max="500">
      </div>
    </div>
  </div>
  <div class="footer-container dummy">
    <div class="footer">dummy</div>
  </div>
  <div class="footer-container">
    <div class="footer">Made by kena0ki</div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { types } from 'dddl';
export default defineComponent({
  name: 'Demo',
  setup() {
    types.a;
    return {
    };
  },
});
</script>

<style>
hr {
  margin-top: 2em;
}
.header-container {
  position: fixed;
  top: 0;
  left: 0;
  display: flex;
  justify-content: center;
  width: 100%;
  padding: 1rem 0;
  background-color: white;
  box-shadow: inset 0 0 3em rgba(0, 0, 0,.1);
  z-index: 1;
}
.header-container.dummy {
  position: relative;
  z-index: -1
}
.header {
  margin: auto .5rem;
}
.input-area {
  text-align: center;
}
.output-area {
  text-align: center;
}
.input-area-create-statement-container {
  width: 100%;
  height: 40vh;
}
.input-area-create-statement {
  width: 90%;
  height: 90%;
}
.input-area-button,
.output-area-button {
  margin: .3em;
  border-radius: 5px;
}
.options-header {
  text-align: center;
}
.options-form {
  padding: .5em;
}
.general-options-header {
  margin-top: 0;
}
.general-options {
  padding-left: .5em;
}
.column-option-default {
  padding-left: .5em;
}
.column-option-default-numeric {
  padding-left: .5em;
}
.options.select {
  font-size: revert;
  height: revert;
  margin: .5em;
}
.options.input {
  font-size: revert;
  margin: .5em;
}

.footer-container {
  position: fixed;
  left: 0;
  bottom: 0px;
  width: 100%;
  text-align: right;
  background-color: white;
  box-shadow: inset 0 0 3em rgba(0, 0, 0,.1);
  z-index: 1;
}
.footer-container.dummy {
  position: relative;
  z-index: -1;
}
.footer {
  margin: .5rem;
}
.gh-ribbon {
  display: block;
  width: 80px;
  height: 80px;
  position: fixed;
  top: 0;
  right: 0;
  z-index: 1;
}
.settings {
  margin: 1rem 0;
}
.sub-container-settings-input {
  text-align: center;
}
.settings-container {
  max-width: 400px;
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
}
.settings {
  width: 200px;
}
.guide {
  font-size: .7rem;
  color: rgba(0,0,0,.5);
}
.settings-title {
  margin: 2rem auto 0 auto;
}
.rerender-btn-container {
  text-align: center;
  margin: 1rem auto 3rem auto;
  width: 100%;
}
</style>
