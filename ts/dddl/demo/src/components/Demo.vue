<template>
  <div class="header-container">
    <h1 class="header">DDDL</h1>
  </div>
  <a href="https://github.com/kena0ki/adima">
    <img class="gh-ribbon" src="../assets/forkme_right_red_aa0000.svg">
  </a>
  <div class="input-area">
    <h2>Input</h2>
    <p>Input a create statement here.</p>
    <div class="input-area-create-statement-container">
      <textarea class="input-area-create-statement" />
    </div>
    <div class="input-area-button-container">
      <button class="pure-button input-area-button">Parse</button>
      <button class="pure-button input-area-button">Generate</button>
    </div>
  </div>
  <hr>
  <div class="options-area">
    <div class="options-header">
      <h2>Options</h2>
    </div>
    <form class="options-form pure-form pure-form-stacked">
      <h3>General options</h3>
      <div class="general-options">
        <label class="options label" for="options-output-format">Output format:</label>
        <select id="options-output-format" class="options select">
          <option>Csv</option>
          <option>Insert statement</option>
        </select>
        <h5>Column option default for each type</h5>
        <div class="column-option-default">
          <label class="options label" for="options-column-option-default-type">Type:</label>
          <select id="options-column-option-default-type" class="options select">
            <option>Numeric type</option>
            <option>String type</option>
          </select>
          <div class="column-option-default-numeric">
            <label class="options label" for="column-option-default-numeric">Step:</label>
            <input id="column-option-default-numeric" class="options input">
          </div>
        </div>
        <h5>Column options</h5>
      </div>
    </form>
  </div>
  <div class="result">
    result
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
  <div class="footer-container">
    <div class="footer">Made by kena0ki</div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
export default defineComponent({
  name: 'Demo',
});
</script>

<style>
.header-container {
  display: flex;
  justify-content: center;
  width: 100%;
  padding: 1rem 0;
}
.header {
  margin: auto .5rem;
}
.input-area {
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
.input-area-button {
  margin: .3em;
  border-radius: 5px;
}
.options-header {
  text-align: center;
}
.options-form {
  padding: .5em;
}
.options.label {
  display: inline-block;
  margin-right: .7em;
}
.options.select {
  display: inline-block!important;
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
.options.input {
  display: inline-block!important;
}

.footer-container {
  position: fixed;
  left: 0;
  bottom: 0px;
  width: 100%;
  text-align: right;
}
.footer {
  margin: .5rem;
}
.gh-ribbon {
  display: block;
  width: 80px;
  height: 80px;
  position: absolute;
  top: 0;
  right: 0;
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
