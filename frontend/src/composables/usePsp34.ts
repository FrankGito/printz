// Import reactive and ref from Vue
import { ref, reactive } from "vue";

export default function usePsp34() {
  // Define a reactive variable
  const reactiveVariable = reactive({ value: "Initial value" });

  // Define a ref variable
  const simpleRef = ref(0);

  // Define a function to update the reactive variable
  const updateReactiveVariable = (newValue: string) => {
    reactiveVariable.value = newValue;
  };

  // Define function to increment simpleRef value.
  const incrementSimpleRef = () => {
    simpleRef.value += 1;
  };

  // Expose the variable and function
  return {
    reactiveVariable,
    updateReactiveVariable,
    simpleRef,
    incrementSimpleRef,
  };
}
