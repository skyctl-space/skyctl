<template>
    <div class="exposure-selector">
      <div class="exposure-button" @click="startEditing">
        <div class="exp-icon">
          <div class="exp-text">EXP</div>
        </div>
  
        <div class="exp-value">
          <template v-if="editing">
            <input
              ref="inputRef"
              v-model="inputValue"
              type="text"
              @keydown.enter.prevent="applyInput"
              @blur="cancelOrApply"
              class="exp-input"
              :class="{ invalid: isInvalid, shake: isInvalid }"
              @animationend="isInvalid = false"
            />
          </template>
          <template v-else>
            {{ modelValue % 1 === 0 ? modelValue : modelValue.toFixed(3) }}s
          </template>
        </div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, nextTick } from 'vue';
  
  const props = defineProps<{
    modelValue: number;
  }>();
  
  const emit = defineEmits<{
    (e: 'update:modelValue', value: number): void;
  }>();
  
  const editing = ref(false);
  const inputValue = ref('');
  const inputRef = ref<HTMLInputElement | null>(null);
  const isInvalid = ref(false);
  
  function startEditing() {
    editing.value = true;
    inputValue.value = props.modelValue.toFixed(3);
    isInvalid.value = false;
    nextTick(() => inputRef.value?.focus());
  }
  
  function applyInput() {
    const num = parseFloat(inputValue.value);
    if (!isNaN(num) && num >= 0.001) {
      emit('update:modelValue', num);
      editing.value = false;
      isInvalid.value = false;
    } else {
      isInvalid.value = true;
    }
  }
  
  function cancelOrApply() {
    applyInput();
  }
  </script>
  
  <style scoped>
  .exposure-selector {
    display: inline-block;
  }
  
  .exposure-button {
    display: flex;
    flex-direction: column;
    align-items: center;
    cursor: pointer;
    user-select: none;
  }
  
  .exp-icon {
    width: 48px;
    height: 32px;
    border: 2px solid currentColor;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .exp-text {
    font-weight: bold;
    font-size: 14px;
    color: currentColor;
  }
  
  .exp-value {
    margin-top: 4px;
    font-size: 13px;
    color: currentColor;
  }
  
  .exp-input {
    width: 50px;
    font-size: 13px;
    text-align: center;
    border: none;
    background: transparent;
    color: currentColor;
    outline: none;
    border-bottom: 1px solid currentColor;
    transition: border-color 0.2s, color 0.2s;
  }
  
  .exp-input.invalid {
    color: #f44336;
    border-color: #f44336;
  }
  
  /* Shake animation */
  @keyframes shake {
    0% { transform: translateX(0); }
    20% { transform: translateX(-3px); }
    40% { transform: translateX(3px); }
    60% { transform: translateX(-3px); }
    80% { transform: translateX(3px); }
    100% { transform: translateX(0); }
  }
  
  .shake {
    animation: shake 0.3s ease;
  }
  </style>
