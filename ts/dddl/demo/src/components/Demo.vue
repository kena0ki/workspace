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
    <form class="options-form pure-form">
      <div class="input-area-create-statement-container">
        <textarea v-model="ddl" class="input-area-create-statement" />
      </div>
      <div class="input-area-button-container">
        <input type="button" value="Parse" class="pure-button output-area-button" @click="onParse">
      </div>
    </form>
  </div>
  <hr>
  <div class="options-area">
    <div class="options-header">
      <h2>Options</h2>
      <p>Step2. Set options for data to be generated.</p>
    </div>
    <form class="options-form pure-form">
      <h3 class="general-options-header">General options</h3>
      <div class="general-options indent-05">
        <label class="options label" for="options-output-format">Output format:</label>
        <select id="options-output-format" class="options select">
          <option>Csv</option>
          <option>Insert statement</option>
        </select>
        <h4>Column option default for each type</h4>
        <div class="column-option-default indent-05">
          <h5>Numeric type</h5>
          <div class="column-option-default-numeric indent-05">
            <div>
              <label class="options label" for="column-option-default-numeric-step">Step:</label>
              <input id="column-option-default-numeric-step" v-model="option.columnOptionsDefault.num.stepBy" class="options input">
            </div>
            <div>
              <label class="options label" for="column-option-default-numeric-initial-value">Initial value:</label>
              <input id="column-option-default-numeric-initial-value" v-model="option.columnOptionsDefault.num.initialValue" class="options input">
            </div>
            <div>
              <label class="options label" for="column-option-default-numeric-limit">Limit:</label>
              <input id="column-option-default-numeric-limit" v-model="option.columnOptionsDefault.num.limit" class="options input">
            </div>
            <div>
              <label class="options label" for="column-option-default-numeric-loop">Loop:</label>
              <select id="column-option-default-numeric-loop" v-model="option.columnOptionsDefault.num.loop" class="options select">
                <option v-for="opt in NUM_LOOP_OPTS" :key="opt">{{ opt }}</option>
              </select>
            </div>
          </div>
          <h5>String type</h5>
          <div class="column-option-default-string indent-05">
            <div>
              <label class="options label" for="column-option-default-string-max-length">Max length:</label>
              <input id="column-option-default-string-max-length" v-model="option.columnOptionsDefault.str.maxLength" class="options input">
            </div>
            <div>
              <label class="options label" for="column-option-default-string-length-in">Unit of length:</label>
              <select id="column-option-default-string-length-in" v-model="option.columnOptionsDefault.str.lengthIn" class="options select">
                <option v-for="opt in LENGTH_IN_OPTS" :key="opt">{{ opt }}</option>
              </select>
            </div>
            <div>
              <label class="options label" for="column-option-default-string-prefix">Prefix:</label>
              <input id="column-option-default-string-prefix" class="options input">
            </div>
            <div>
              <label class="options label" for="column-option-default-string-loop">Loop:</label>
              <select id="column-option-default-string-loop" v-model="option.columnOptionsDefault.str.loop" class="options select">
                <option v-for="opt in STR_LOOP_OPTS" :key="opt">{{ opt }}</option>
              </select>
            </div>
          </div>
          <h5>Datetime type</h5>
          <div class="column-option-default-datetime indent-05">
            <div>
              <label class="options label" for="column-option-default-datetime-initial-value">Initial value:</label>
              <input id="column-option-default-datetime-initial-value" class="options input">
            </div>
          </div>
          <h5>Boolean type</h5>
          <div class="column-option-default-boolean indent-05">
            <div>
              <label class="options label" for="column-option-default-boolean-initial-value">Initial value:</label>
              <select id="column-option-default-boolean-loop" v-model="option.columnOptionsDefault.bool.initialValue" class="options select">
                <option v-for="opt in [false,true]" :key="opt">{{ opt }}</option>
              </select>
            </div>
            <div>
              <label class="options label" for="column-option-default-boolean-random">Random:</label>
              <select id="column-option-default-boolean-loop" v-model="option.columnOptionsDefault.bool.random" class="options select">
                <option v-for="opt in [false,true]" :key="opt">{{ opt }}</option>
              </select>
            </div>
            <div>
              <label class="options label" for="column-option-default-boolean-use-null">Use null:</label>
              <select id="column-option-default-boolean-use-null" v-model="option.columnOptionsDefault.bool.useNull" class="options select">
                <option v-for="opt in [false,true]" :key="opt">{{ opt }}</option>
              </select>
            </div>
          </div>
        </div>
        <h4>Column options</h4>
        <template v-for="(usedIdx,usedIdxIdx) in colIdxInUse" :key="usedIdxIdx">
          <div class="column-options">
            <label class="options label" for="options-column-options-column-name">Column name:</label>
            <select id="options-column-options-column-name" v-model="colIdxInUse[usedIdxIdx]" class="options select">
              <option v-for="(opt,idx) in columnDefs.filter((_,i) => !colIdxInUse.includes(i))" :key="idx" :value="idx">{{ opt.colName }}</option>
            </select>
            <template v-if="columnDefs[usedIdx] instanceof dataTypes.NumericType">
              num
            </template>
            <template v-if="columnDefs[usedIdx] instanceof dataTypes.StringType">
              string
            </template>
            <template v-if="columnDefs[usedIdx] instanceof dataTypes.DatetimeType">
              datetime
            </template>
            <template v-if="columnDefs[usedIdx] instanceof dataTypes.BooleanType">
              boolean
            </template>
          </div>
        </template>
      </div>
      <div>
        <input type="button" value="Add" class="pure-button input-area-button column-options-add" @click="onAddColOpt">
      </div>
    </form>
  </div>
  <hr>
  <div class="output-area">
    <div class="output-header">
      <h2>Output</h2>
      <p>Step3. Now, it's time to generate data. Click generate button.</p>
    </div>
    <div class="output-area-button-container">
      <input type="button" value="Generate" class="pure-button output-area-button">
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
import { ref, defineComponent } from 'vue';
import {
  GeneratorOption,
  NumericColumnOption,
  StringColumnOption,
  DatetimeColumnOption,
  BooleanColumnOption,
  NUM_LOOP_OPTS,
  STR_LOOP_OPTS,
  LENGTH_IN_OPTS,
  dataTypes,
  parser,
} from '../../../dist';
// type SetUp = {
//   option: Ref<UnwrapRef<GeneratorOption>>,
// };
const COLUMN_OPTIONS_TYPES = [
  { clazz: NumericColumnOption, label: 'Numeric type' },
  { clazz: StringColumnOption, label: 'String type' },
  { clazz: DatetimeColumnOption, label: 'Datetime type' },
  { clazz: BooleanColumnOption, label: 'Boolean type' },
];
export default defineComponent({
  name: 'Demo',
  setup() {
    const ddl = ref('');
    const option = ref(new GeneratorOption);
    const colIdxInUse = ref<(number|undefined)[]>([]);
    type ColDefType = ({colName: string, type: dataTypes.DataType})[];
    const columnDefs = ref<ColDefType>([]);
    const onParse = () => {
      const [statements, error] = parser.parse(ddl.value);
      if (!statements) throw error;
      if (statements.length<=0) return;
      columnDefs.value = statements[0].columns.reduce<ColDefType>((prev, curr) => prev.concat({ colName: curr.name.value, type: curr.dataType }), []);
      console.log(columnDefs.value);
    };
    const onAddColOpt = () => {
      colIdxInUse.value.push(undefined);
    };
    return {
      ddl,
      option,
      COLUMN_OPTIONS_TYPES,
      NUM_LOOP_OPTS,
      STR_LOOP_OPTS,
      LENGTH_IN_OPTS,
      columnDefs,
      dataTypes,
      colIdxInUse,
      onParse,
      onAddColOpt,
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
.indent-05 {
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
