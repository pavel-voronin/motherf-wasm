import { ref, computed } from "vue";
import { load } from "/js/programs/index.js";

export default {
  template: `
    <div>
        <select v-model="selected" @change="loadProgram()">
            <option v-for="(program, i) in programs" :key="i" :value="program">{{ program.name }}</option>
        </select>
        <div>
            <textarea cols="80" rows="10" v-model="code"></textarea>
        </div>
        <div>
            <textarea cols="80" rows="10" v-model="input"></textarea>
        </div>
        <div>
            <b>Output:</b>
            <pre>{{ result }}</pre>
        </div>
        <div>
            <b>Error:</b>
            <pre>{{ error }}</pre>
        </div>
        <div>
            <button @click="run">Run</button>
            <button @click="terminate">Terminate</button>
        </div>
    </div>`,
  setup() {
    let programs = ref(null);

    let selected = ref(null);
    load().then((data) => (programs.value = data));

    let worker = ref(null);
    let result = ref("");
    let error = ref("");
    let code = ref(
      ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+."
    );
    let input = ref("");
    let clearInput = computed(() => input.value.trim().replaceAll("\\0", "\0"));

    const initWorker = () => {
      worker.value = new Worker(new URL("/js/worker.js", import.meta.url), {
        type: "module",
      });

      worker.value.onmessage = function ({ data }) {
        result.value = "";
        error.value = "";

        if (data.result !== null) {
          result.value = data.result;
        } else if (data.error !== null) {
          error.value = data.error;
        }
      };
    };

    initWorker();

    const run = async () => {
      result.value = "...";
      error.value = "";

      worker.value.postMessage({ code: code.value, input: clearInput.value });
    };

    const terminate = () => {
      worker.value.terminate();

      result.value = "";
      error.value = "terminated";

      initWorker();
    };

    const loadProgram = () => {
      code.value = selected.value.code;
      input.value = selected.value.input;
    };

    return {
      run,
      terminate,
      code,
      input,
      result,
      error,
      programs,
      selected,
      loadProgram,
    };
  },
};
