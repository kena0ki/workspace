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
      <div class="spacer-1em" />
      <div class="input-area-button-container">
        <button type="button" class="pure-button output-area-button" @click="onClickParse">Parse</button>
      </div>
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
          <h3 class="general-options-header accordion" @click="onClickAccordion($event)">> General options</h3>
        </button>
        <div class="accordion-panel">
          <div class="general-options indent-05 panel">
            <label class="options label" for="options-output-format">Output format:</label>
            <select id="options-output-format" class="options select">
              <option v-for="opt in OutputFormatOption" :key="opt.label">{{ opt.label }}</option>
            </select>
            <div class="accordion-container">
              <button type="button" class="accordion-button" @click="onClickAccordion($event)">
                <h4 class="accordion">> Column option by type</h4>
              </button>
              <div class="accordion-panel">
                <div class="column-option-default indent-05 panel">
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
                      <input id="column-option-default-datetime-initial-value" v-model="option.columnOptionsDefault.date.initialValue" class="options input">
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
              </div>
            </div>
            <div class="accordion-container">
              <button type="button" class="accordion-button" @click="onClickAccordion($event)">
                <h4 class="accordion">
                  > Column option
                </h4>
              </button>
              <button type="button" class="pure-button input-area-button column-options-add" @click="onClickAddColOpt">+</button>
              <div class="accordion-panel">
                <div class="column-options indent-05">
                  <div v-for="(name,nameIdx) in colNameInUse" :key="nameIdx" :set="type = columnDefs.find(def => def.colName === name)?.type">
                    <h5 style="display: inline-block" class="options label" for="options-column-options-column-name">Column name:</h5>
                    <select id="options-column-options-column-name" :value="colNameInUse[nameIdx]" class="options select" @change="onChangeColName(nameIdx, $event)">
                      <option v-for="(opt) in columnDefs.filter(def => (!colNameInUse.includes(def.colName) || def.colName === name))" :key="opt">{{ opt.colName }}</option>
                    </select>
                    <template v-if="type instanceof dataTypes.NumericType">
                      <div class="column-option-default-numeric indent-05">
                        <div>
                          <label class="options label" for="column-option-default-numeric-step">Step:</label>
                          <input id="column-option-default-numeric-step" v-model="option.columnOptions[name].stepBy" class="options input">
                        </div>
                        <div>
                          <label class="options label" for="column-option-default-numeric-initial-value">Initial value:</label>
                          <input id="column-option-default-numeric-initial-value" v-model="option.columnOptions[name].initialValue" class="options input">
                        </div>
                        <div>
                          <label class="options label" for="column-option-default-numeric-limit">Limit:</label>
                          <input id="column-option-default-numeric-limit" v-model="option.columnOptions[name].limit" class="options input">
                        </div>
                        <div>
                          <label class="options label" for="column-option-default-numeric-loop">Loop:</label>
                          <select id="column-option-default-numeric-loop" v-model="option.columnOptions[name].loop" class="options select">
                            <option v-for="opt in NUM_LOOP_OPTS" :key="opt">{{ opt }}</option>
                          </select>
                        </div>
                      </div>
                    </template>
                    <template v-if="type instanceof dataTypes.StringType">
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
                    </template>
                    <template v-if="type instanceof dataTypes.DatetimeType">
                      <div class="column-option-default-datetime indent-05">
                        <div>
                          <label class="options label" for="column-option-default-datetime-initial-value">Initial value:</label>
                          <input id="column-option-default-datetime-initial-value" v-model="option.columnOptions[name].initialValue" class="options input">
                        </div>
                      </div>
                    </template>
                    <template v-if="type instanceof dataTypes.BooleanType">
                      <div class="column-option-default-boolean indent-05">
                        <div>
                          <label class="options label" for="column-option-default-boolean-initial-value">Initial value:</label>
                          <select id="column-option-default-boolean-loop" v-model="option.columnOptions[name].initialValue" class="options select">
                            <option v-for="opt in [false,true]" :key="opt">{{ opt }}</option>
                          </select>
                        </div>
                        <div>
                          <label class="options label" for="column-option-default-boolean-random">Random:</label>
                          <select id="column-option-default-boolean-loop" v-model="option.columnOptions[name].random" class="options select">
                            <option v-for="opt in [false,true]" :key="opt">{{ opt }}</option>
                          </select>
                        </div>
                        <div>
                          <label class="options label" for="column-option-default-boolean-use-null">Use null:</label>
                          <select id="column-option-default-boolean-use-null" v-model="option.columnOptions[name].useNull" class="options select">
                            <option v-for="opt in [false,true]" :key="opt">{{ opt }}</option>
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
      <textarea :value="generatedData" class="generated-data" wrap="off" disabled />
    </div>
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
  NumericColumnOption,
  StringColumnOption,
  DatetimeColumnOption,
  BooleanColumnOption,
  NUM_LOOP_OPTS,
  STR_LOOP_OPTS,
  LENGTH_IN_OPTS,
  dataTypes,
  parse,
  CreateTableStatement,
  CsvFormat,
  InsertStatementFormat,
} from '../../../dist';
import { getOptionByLabel } from '../util';
// type SetUp = {
//   option: Ref<UnwrapRef<GeneratorOption>>,
// };
const DDL=`create table "ITEM" (
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
export default defineComponent({
  name: 'Demo',
  setup() {
    const ddl = ref(DDL);
    const option = ref(new GeneratorOption);
    const colNameInUse = ref<(string|undefined)[]>([]);
    type ColDefType = ({colName: string, type: dataTypes.DataType})[];
    const columnDefs = ref<ColDefType>([]);
    let parseDone = ref(false);
    let stmt: CreateTableStatement|undefined;
    const onClickParse = () => {
      const [statements, error] = parse(ddl.value);
      if (!statements) throw error;
      if (statements.length<=0) return;
      stmt = statements[0];
      parseDone.value = true;
      columnDefs.value = statements[0].columns.reduce<ColDefType>((prev, curr) => prev.concat({ colName: curr.name.value, type: curr.dataType }), []);
      console.log(columnDefs.value);
    };
    const onClickAddColOpt = () => {
      colNameInUse.value.push(undefined);
    };
    const generatedData = ref('');
    const onClickGenerate = async () => {
      if (!stmt) return;
      generatedData.value = '';
      for await (const [result, errors]  of generate(stmt, new GeneratorOption(option.value))) {
        if (errors.length > 0) throw errors;
        generatedData.value += result.row + '\n';
      }
    };
    const onChangeColName = (idx: number, event: Event) => {
      const oldName = colNameInUse.value[idx];
      if (oldName) option.value.columnOptions[oldName] = undefined;
      const newName = (event.target as HTMLInputElement).value;
      colNameInUse.value[idx] = newName;
      const type = columnDefs.value.find(def => def.colName === newName)?.type;
      if (type instanceof dataTypes.NumericType) {
        option.value.columnOptions[newName] = new NumericColumnOption();
      } else if (type instanceof dataTypes.StringType) {
        option.value.columnOptions[newName] = new StringColumnOption();
      } else if (type instanceof dataTypes.DatetimeType) {
        option.value.columnOptions[newName] = new DatetimeColumnOption();
      } else if (type instanceof dataTypes.BooleanType) {
        option.value.columnOptions[newName] = new BooleanColumnOption();
      }
    };
    const OutputFormatOption = [
      { label: 'Csv',              constructor: () => new CsvFormat(option.value.outputFormat) },
      { label: 'Insert statement', constructor: () => new InsertStatementFormat },
    ];
    return {
      ddl,
      option,
      parseDone,
      NUM_LOOP_OPTS,
      STR_LOOP_OPTS,
      LENGTH_IN_OPTS,
      columnDefs,
      dataTypes,
      colNameInUse,
      generatedData,
      onClickParse,
      onClickAddColOpt,
      onClickGenerate,
      onChangeColName,
    };
  },
  methods: {
    onClickAccordion(event: Event) {
      const el = event.currentTarget as HTMLElement;
      const container = el.parentElement as HTMLElement;
      container.classList.toggle("active");
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
  width: 98%;
  height: 100%;
}
.spacer-1em {
  height: 1em;
}
.spacer-2em {
  height: 2em;
}
.column-options-add {
  margin-left: 1em;
  font-size: 60%;
}
.accordion-button {
  background-color: #fff;
  color: #444;
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
</style>
