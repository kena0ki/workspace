<template>
  <div class="header-container dummy">
    <h1 class="header-title">&nbsp;</h1>
    <div class="header-subtitle"><span>generates Data from DDL, </span><span>i.e. create table statements.</span></div>
  </div>
  <div class="header-container">
    <h1 class="header-title">DDDL</h1>
    <div class="header-subtitle"><span>generates Data from DDL, </span><span>i.e. create table statements.</span></div>
  </div>
  <a href="https://github.com/kena0ki/adima">
    <img class="gh-ribbon" src="../assets/forkme_right_red_aa0000.svg">
  </a>
  <div class="input-area">
    <h2>Input</h2>
    <p class="instruction">Step1. Input a create statement here.</p>
    <form class="options-form pure-form">
      <div class="input-area-create-statement-container">
        <textarea v-model="ddl" class="input-area-create-statement" :placeholder="PLACEHOLDER_DDL" />
      </div>
      <div class="spacer-1em" />
      <div class="input-area-button-container">
        <button type="button" class="pure-button output-area-button" @click="onClickParse">Parse</button>
      </div>
      <div v-if="parseMessageType" :class="{ [parseMessageType]: true }">{{ parseMessage }}</div>
    </form>
  </div>
  <div class="spacer-1em" />
  <hr>
  <div class="options-area">
    <div class="options-header">
      <h2>Options</h2>
      <p>Step2. Set options for data to be generated.</p>
    </div>
    <form class="options-form pure-form">
      <div class="accordion-container">
        <button type="button" class="accordion-button" :disabled="!parseDone" @click="onClickAccordion($event)">
          <h3 class="general-options-header accordion">
            <span class="accordion-arrow">ᐅ</span> Generator options
          </h3>
        </button>
        <div class="accordion-panel">
          <div class="general-options indent-05 panel">
            <div class="options-size">
              <label class="options label" for="options-size">Data size:</label>
              <input id="options-size" v-model="genOpt.size" class="options input" type="number" step="any" max="100">
            </div>
            <div class="options-output-format">
              <label class="options label" for="options-output-format">Output format:</label>
              <select id="options-output-format" class="options select" @change="onChangeOutputFormat($event.currentTarget.value)">
                <option v-for="opt in OUTPUT_FORMAT_OPTS" :key="opt.label">{{ opt.label }}</option>
              </select>
              <template v-if="genOpt.outputFormat.__tag === 'CsvFormat'">
                <div class="options-output-format-csv indent-05">
                  <div>
                    <label class="options label" for="options-output-format-csv-delimiter">Step:</label>
                    <input id="options-output-format-csv-delimiter" v-model="genOpt.outputFormat.delimiter" class="options input">
                  </div>
                  <div>
                    <label class="options label" for="options-output-format-csv-quote">Quote:</label>
                    <input id="options-output-format-csv-quote" v-model="genOpt.outputFormat.quote" class="options input">
                  </div>
                  <div>
                    <label class="options label" for="options-output-format-csv-escapeSequence">Escape sequence:</label>
                    <input id="options-output-format-csv-escapeSequence" v-model="genOpt.outputFormat.escapeSequence" class="options input">
                  </div>
                  <div>
                    <label class="options label" for="options-output-format-csv-header">Needs header:</label>
                    <select id="options-output-format-csv-header" v-model="genOpt.outputFormat.header" class="options select">
                      <option v-for="opt in [false,true]" :key="opt">{{ opt }}</option>
                    </select>
                  </div>
                </div>
              </template>
            </div>
            <div class="accordion-container">
              <button type="button" class="accordion-button" @click="onClickAccordion($event)">
                <h4 class="accordion"><span class="accordion-arrow">ᐅ</span> Column options by type</h4>
              </button>
              <div class="accordion-panel">
                <div class="column-option-default indent-05 panel">
                  <h5>Numeric type</h5>
                  <div class="column-option-default-numeric indent-05">
                    <div>
                      <label class="options label" for="column-option-default-numeric-step">Step:</label>
                      <input id="column-option-default-numeric-step" v-model="genOpt.columnOptionsDefault.num.stepBy" type="number" step="any" class="options input">
                    </div>
                    <div>
                      <label class="options label" for="column-option-default-numeric-initial-value">Initial value:</label>
                      <input id="column-option-default-numeric-initial-value" v-model="genOpt.columnOptionsDefault.num.initialValue" type="number" step="any" class="options input">
                    </div>
                    <div>
                      <label class="options label" for="column-option-default-numeric-limit">Limit:</label>
                      <input id="column-option-default-numeric-limit" v-model="genOpt.columnOptionsDefault.num.limit" type="number" step="any" class="options input">
                    </div>
                    <div>
                      <label class="options label" for="column-option-default-numeric-loop">Loop:</label>
                      <select id="column-option-default-numeric-loop" v-model="genOpt.columnOptionsDefault.num.loop" class="options select">
                        <option v-for="opt in NUM_LOOP_OPTS" :key="opt">{{ opt }}</option>
                      </select>
                    </div>
                  </div>
                  <h5>String type</h5>
                  <div class="column-option-default-string indent-05">
                    <div>
                      <label class="options label" for="column-option-default-string-max-length">Max length:</label>
                      <input id="column-option-default-string-max-length" v-model="genOpt.columnOptionsDefault.str.maxLength" type="number" class="options input">
                    </div>
                    <div>
                      <label class="options label" for="column-option-default-string-length-in">Unit of length:</label>
                      <select id="column-option-default-string-length-in" v-model="genOpt.columnOptionsDefault.str.lengthIn" class="options select">
                        <option v-for="opt in LENGTH_IN_OPTS" :key="opt">{{ opt }}</option>
                      </select>
                    </div>
                    <div>
                      <label class="options label" for="column-option-default-string-prefix">Prefix:</label>
                      <input id="column-option-default-string-prefix" v-model="genOpt.columnOptionsDefault.str.prefix" class="options input">
                    </div>
                    <div>
                      <label class="options label" for="column-option-default-string-loop">Loop:</label>
                      <select id="column-option-default-string-loop" v-model="genOpt.columnOptionsDefault.str.loop" class="options select">
                        <option v-for="opt in STR_LOOP_OPTS" :key="opt">{{ opt }}</option>
                      </select>
                    </div>
                  </div>
                  <h5>Datetime type</h5>
                  <div class="column-option-default-datetime indent-05">
                    <div>
                      <label class="options label" for="column-option-default-datetime-initial-value-date">Initial value:</label>
                      <input id="column-option-default-datetime-initial-value-date" :value="genOpt.columnOptionsDefault.date.initialValue?.toISOString().slice(0,10)"
                             type="date" class="options input" @change="onChangeDefaultOptionsInitialDateValue($event.currentTarget.value)"
                      >
                      <input id="column-option-default-datetime-initial-value-time" :value="genOpt.columnOptionsDefault.date.initialValue?.toISOString().slice(11,19)"
                             type="time" step="1" class="options input" @change="onChangeDefaultOptionsInitialTimeValue($event.currentTarget.value)"
                      >
                    </div>
                  </div>
                  <h5>Boolean type</h5>
                  <div class="column-option-default-boolean indent-05">
                    <div>
                      <label class="options label" for="column-option-default-boolean-initial-value">Initial value:</label>
                      <select id="column-option-default-boolean-loop" v-model="genOpt.columnOptionsDefault.bool.initialValue" class="options select">
                        <option v-for="opt in [false,true]" :key="opt" :value="opt">{{ opt }}</option>
                      </select>
                    </div>
                    <div>
                      <label class="options label" for="column-option-default-boolean-random">Random:</label>
                      <select id="column-option-default-boolean-loop" v-model="genOpt.columnOptionsDefault.bool.random" class="options select">
                        <option v-for="opt in [false,true]" :key="opt" :value="opt">{{ opt }}</option>
                      </select>
                    </div>
                    <div>
                      <label class="options label" for="column-option-default-boolean-use-null">Use null:</label>
                      <select id="column-option-default-boolean-use-null" v-model="genOpt.columnOptionsDefault.bool.useNull" class="options select">
                        <option v-for="opt in [false,true]" :key="opt" :value="opt">{{ opt }}</option>
                      </select>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            <div class="accordion-container">
              <button type="button" class="accordion-button" @click="onClickAccordion($event)">
                <h4 class="accordion"><span class="accordion-arrow">ᐅ</span> Column options by name</h4>
              </button>
              <button type="button" class="pure-button input-area-button column-options-add" @click="onClickAddColOpt">✚</button>
              <div class="accordion-panel">
                <div class="column-options indent-05">
                  <div v-for="(name,nameIdx) in colNameInUse" :key="nameIdx" :set="type = columnDefs.find(def => def.colName === name)?.type">
                    <h5 style="display: inline-block" class="options label" for="options-column-options-column-name">Column name:</h5>
                    <select id="options-column-options-column-name" :value="colNameInUse[nameIdx]" class="options select" @change="onChangeColName(nameIdx, $event)">
                      <option v-for="(opt) in columnDefs.filter(def => (!colNameInUse.includes(def.colName) || def.colName === name))" :key="opt">{{ opt.colName }}</option>
                    </select>
                    <button type="button" class="pure-button input-area-button column-options-del" @click="onClickDelColOpt(name, nameIdx)">✖</button>
                    <template v-if="type instanceof dataTypes.NumericType">
                      <div class="column-option-numeric indent-05">
                        <div>
                          <label class="options label" for="column-option-numeric-step">Step:</label>
                          <input id="column-option-numeric-step" v-model="genOpt.columnOptions[name].stepBy" class="options input">
                        </div>
                        <div>
                          <label class="options label" for="column-option-numeric-initial-value">Initial value:</label>
                          <input id="column-option-numeric-initial-value" v-model="genOpt.columnOptions[name].initialValue" class="options input">
                        </div>
                        <div>
                          <label class="options label" for="column-option-numeric-limit">Limit:</label>
                          <input id="column-option-numeric-limit" v-model="genOpt.columnOptions[name].limit" class="options input">
                        </div>
                        <div>
                          <label class="options label" for="column-option-numeric-loop">Loop:</label>
                          <select id="column-option-numeric-loop" v-model="genOpt.columnOptions[name].loop" class="options select">
                            <option v-for="opt in NUM_LOOP_OPTS" :key="opt">{{ opt }}</option>
                          </select>
                        </div>
                      </div>
                    </template>
                    <template v-if="type instanceof dataTypes.StringType">
                      <div class="column-option-string indent-05">
                        <div>
                          <label class="options label" for="column-option-string-max-length">Max length:</label>
                          <input id="column-option-string-max-length" v-model="genOpt.columnOptions[name].maxLength" class="options input">
                        </div>
                        <div>
                          <label class="options label" for="column-option-string-length-in">Unit of length:</label>
                          <select id="column-option-string-length-in" v-model="genOpt.columnOptions[name].lengthIn" class="options select">
                            <option v-for="opt in LENGTH_IN_OPTS" :key="opt">{{ opt }}</option>
                          </select>
                        </div>
                        <div>
                          <label class="options label" for="column-option-string-prefix">Prefix:</label>
                          <input id="column-option-string-prefix" v-model="genOpt.columnOptions[name].prefix" class="options input">
                        </div>
                        <div>
                          <label class="options label" for="column-option-string-loop">Loop:</label>
                          <select id="column-option-string-loop" v-model="genOpt.columnOptions[name].loop" class="options select">
                            <option v-for="opt in STR_LOOP_OPTS" :key="opt">{{ opt }}</option>
                          </select>
                        </div>
                      </div>
                    </template>
                    <template v-if="type instanceof dataTypes.DatetimeType">
                      <div class="column-option-datetime indent-05">
                        <div>
                          <label class="options label" for="column-option-datetime-initial-value">Initial value:</label>
                          <input id="column-option-datetime-initial-value-date" :value="genOpt.columnOptions[name].initialValue?.toISOString().slice(0,10)"
                                 type="date" class="options input" @change="onChangeOptionsInitialDateValue($event.currentTarget.value, name)"
                          >
                          <input id="column-option-datetime-initial-value-time" :value="genOpt.columnOptions[name].initialValue?.toISOString().slice(11,19)"
                                 type="time" step="1" class="options input" @change="onChangeOptionsInitialTimeValue($event.currentTarget.value, name)"
                          >
                        </div>
                      </div>
                    </template>
                    <template v-if="type instanceof dataTypes.BooleanType">
                      <div class="column-option-boolean indent-05">
                        <div>
                          <label class="options label" for="column-option-boolean-initial-value">Initial value:</label>
                          <select id="column-option-boolean-loop" v-model="genOpt.columnOptions[name].initialValue" class="options select">
                            <option v-for="opt in [false,true]" :key="opt" :value="opt">{{ opt }}</option>
                          </select>
                        </div>
                        <div>
                          <label class="options label" for="column-option-boolean-random">Random:</label>
                          <select id="column-option-boolean-loop" v-model="genOpt.columnOptions[name].random" class="options select">
                            <option v-for="opt in [false,true]" :key="opt" :value="opt">{{ opt }}</option>
                          </select>
                        </div>
                        <div>
                          <label class="options label" for="column-option-boolean-use-null">Use null:</label>
                          <select id="column-option-boolean-use-null" v-model="genOpt.columnOptions[name].useNull" class="options select">
                            <option v-for="opt in [false,true]" :key="opt" :value="opt">{{ opt }}</option>
                          </select>
                        </div>
                      </div>
                    </template>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </form>
  </div>
  <div class="spacer-1em" />
  <hr>
  <div class="output-area">
    <div class="output-header">
      <h2>Output</h2>
      <p>Step3. Click the generate button.</p>
    </div>
    <div class="output-area-button-container">
      <button type="button" class="pure-button output-area-button" :disabled="!parseDone" @click="onClickGenerate">Generate</button>
    </div>
    <div class="spacer-1em" />
    <div class="generated-data-container">
      <div class="generated-data">{{ generatedData }}</div>
    </div>
    <div class="spacer-1em" />
    <div v-if="generateMessageType" :class="{ [generateMessageType]: true }">{{ generateMessage }}</div>
  </div>
  <div class="spacer-2em" />
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
  generate,
  GeneratorOption,
  newGeneratorOption,
  newNumericColumnOption,
  newStringColumnOption,
  newDatetimeColumnOption,
  newBooleanColumnOption,
  NUM_LOOP_OPTS,
  STR_LOOP_OPTS,
  LENGTH_IN_OPTS,
  dataTypes,
  parse,
  CreateTableStatement,
  ParseError,
  newCsvFormat,
  newInsertStatementFormat,
} from '../../../dist';
const PLACEHOLDER_DDL=`create table ITEM (
  id char(10),
  price decimal(10,3),
  qty INTEGER,
  name binary(20),
  type char(2),
  rate decimal(1,0),
  sold_out boolean,
  updatedAt timestamp,
  updateDate date,
  updateTime time,
);`;
const OUTPUT_FORMAT_OPTS = [
  { label: 'Csv',              constructor: newCsvFormat },
  { label: 'Insert statement', constructor: newInsertStatementFormat },
];
export default defineComponent({
  name: 'Demo',
  setup() {
    const ddl = ref(PLACEHOLDER_DDL);
    const genOpt = ref<GeneratorOption>(newGeneratorOption());
    const colNameInUse = ref<(string|undefined)[]>([]);
    type ColDefType = ({colName: string, type: dataTypes.DataType})[];
    const columnDefs = ref<ColDefType>([]);
    const parseMessage = ref('');
    const parseMessageType = ref('');
    let parseDone = ref(false);
    let stmt: CreateTableStatement|undefined;
    const onClickParse = () => {
      const result = parse(ddl.value);
      if (result instanceof ParseError) {
        parseMessage.value = 'Parse error!!\n' + result.message;
        parseMessageType.value = 'isa_error';
        parseDone.value = false;
        return;
      }
      const statements = result;
      if (statements.length<=0) return;
      stmt = statements[0];
      parseDone.value = true;
      columnDefs.value = statements[0].columns.reduce<ColDefType>((prev, curr) => prev.concat({ colName: curr.name.value, type: curr.dataType }), []);
      console.log(columnDefs.value);
      parseMessage.value = 'Parse done!';
      parseMessageType.value = 'isa_success';
    };
    const onClickAddColOpt = () => {
      colNameInUse.value.push(undefined);
    };
    const onClickDelColOpt = (name: string, idx: number) => {
      genOpt.value.columnOptions[name] = undefined;
      colNameInUse.value.splice(idx,1);
    };
    const generatedData = ref('');
    const generateMessage = ref('');
    const generateMessageType = ref('');
    const onClickGenerate = async () => {
      if (!stmt) return;
      generatedData.value = '';
      try {
        for await (const [result, errors] of generate(stmt, genOpt.value)) {
          if (errors.length > 0) {
            generateMessage.value = 'Generate error!!\n' + errors.join('\n');
            generateMessageType.value = 'isa_error';
            generatedData.value = '';
            return;
          }
          generatedData.value += result.row + '\n';
        }
      } catch (err) {
        generateMessage.value = 'Generate error!!';
        generateMessageType.value = 'isa_error';
        generatedData.value = '';
        return;
      }
      generateMessage.value = '';
      generateMessageType.value = '';
    };
    const onChangeColName = (idx: number, event: Event): void => {
      const oldName = colNameInUse.value[idx];
      if (oldName) genOpt.value.columnOptions[oldName] = undefined;
      const newName = (event.target as HTMLInputElement).value;
      colNameInUse.value[idx] = newName;
      const type = columnDefs.value.find(def => def.colName === newName)?.type;
      if (type instanceof dataTypes.NumericType) {
        genOpt.value.columnOptions[newName] = newNumericColumnOption();
      } else if (type instanceof dataTypes.StringType) {
        genOpt.value.columnOptions[newName] = newStringColumnOption();
      } else if (type instanceof dataTypes.DatetimeType) {
        genOpt.value.columnOptions[newName] = newDatetimeColumnOption();
      } else if (type instanceof dataTypes.BooleanType) {
        genOpt.value.columnOptions[newName] = newBooleanColumnOption();
      }
    };
    const onChangeOutputFormat = (label: string): void => {
      genOpt.value.outputFormat = OUTPUT_FORMAT_OPTS.find(o => o.label===label)!.constructor();
    };
    const onChangeDefaultOptionsInitialDateValue = (date: string): void => {
      genOpt.value.columnOptionsDefault.date.initialValue.setUTCFullYear(+date.slice(0,4), +date.slice(5,7)-1, +date.slice(8,10));
    };
    const onChangeDefaultOptionsInitialTimeValue = (time: string): void => {
      genOpt.value.columnOptionsDefault.date.initialValue.setUTCHours(+time.slice(0,2), +time.slice(3,5), +time.slice(6,8));
    };
    const onChangeOptionsInitialDateValue = (date: string, name: string): void => {
      genOpt.value.columnOptions[name] = genOpt.value.columnOptions[name] || newDatetimeColumnOption();
      const opt = genOpt.value.columnOptions[name];
      if (opt?.__tag !== 'DatetimeColumnOption') return;
      opt.initialValue.setUTCFullYear(+date.slice(0,4), +date.slice(5,7)-1, +date.slice(8,10));
    };
    const onChangeOptionsInitialTimeValue = (time: string, name: string): void => {
      genOpt.value.columnOptions[name] = genOpt.value.columnOptions[name] || newDatetimeColumnOption();
      const opt = genOpt.value.columnOptions[name];
      if (opt?.__tag !== 'DatetimeColumnOption') return;
      opt.initialValue.setUTCHours(+time.slice(0,2), +time.slice(3,5), +time.slice(6,8));
    };
    return {
      PLACEHOLDER_DDL,
      ddl,
      genOpt,
      parseDone,
      parseMessage,
      parseMessageType,
      NUM_LOOP_OPTS,
      STR_LOOP_OPTS,
      LENGTH_IN_OPTS,
      OUTPUT_FORMAT_OPTS,
      columnDefs,
      dataTypes,
      colNameInUse,
      generatedData,
      generateMessage,
      generateMessageType,
      onClickParse,
      onClickAddColOpt,
      onClickDelColOpt,
      onClickGenerate,
      onChangeColName,
      onChangeOutputFormat,
      onChangeDefaultOptionsInitialDateValue,
      onChangeDefaultOptionsInitialTimeValue,
      onChangeOptionsInitialDateValue,
      onChangeOptionsInitialTimeValue,
    };
  },
  methods: {
    onClickAccordion(event: Event) {
      const el = event.currentTarget as HTMLElement;
      const container = el.parentElement as HTMLElement;
      container.classList.toggle("active");
      const arrow = el.querySelector('.accordion-arrow') as HTMLElement;
      arrow.textContent = arrow.textContent === 'ᐅ' ? '▼' : 'ᐅ';
    },
  },
});
</script>

<style>
.header-container {
  position: fixed;
  top: 0;
  left: 0;
  display: flex;
  align-items: center;
  flex-direction: column;
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
.header-title {
  margin: auto .5rem;
}
.header-subtitle {
  display: flex;
  justify-content: center;
  flex-wrap: wrap;
  white-space: pre-wrap;
  width: 90%;
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
  height: 100%;
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
.generated-data-container {
  width: 100%;
  height: 40vh;
}
.generated-data {
  height: 100%;
  width: 94%;
  overflow-x: scroll;
}
.spacer-1em {
  height: 1em;
}
.spacer-2em {
  height: 2em;
}
.column-options-add,
.column-options-del {
  margin-left: 1em;
  font-size: 60%;
}
.accordion-button {
  background-color: #fff;
  cursor: pointer;
  padding: 0;
  border: none;
  outline: none;
  transition: 0.4s;
}
.accordion-panel {
  max-height: 0px;
  display: none;
  transition: max-height 0.2s ease-out;
}
.accordion-container.active>.accordion-panel {
  max-height: none;
  display: block;
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
  padding: .5em;
}
.footer-container.dummy {
  position: relative;
  z-index: -1;
}
.footer {
  margin: 0 1em;
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
.isa_info, .isa_success, .isa_warning, .isa_error {
    margin: 0.5em 0px;
    padding: 0.6em;
    border-radius: .3em;
    white-space: pre-wrap;
}
.isa_info {
    color: #00529B;
    background-color: #BDE5F8;
    box-shadow: 0 0 3px #00529B;
}
.isa_success {
    color: #4F8A10;
    background-color: #DFF2BF;
    box-shadow: 0 0 3px #00529B;
}
.isa_warning {
    color: #9F6000;
    background-color: #FEEFB3;
    box-shadow: 0 0 3px #9F6000;
}
.isa_error {
    color: #D8000C;
    background-color: #FFD2D2;
    box-shadow: 0 0 3px #D8000C;
}
</style>
