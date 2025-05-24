<template>
  <v-sheet
    :class="{ 'hidden-menu': !maximized }"
    class="menu-bar d-flex align-center justify-end px-2"
    elevation="2"
  >
    <v-menu
      v-for="(item, index) in menuItems"
      :key="item.name"
      :model-value="menuIndex === index"
      :close-on-content-click="false"
      transition="scale-transition"
    >
      <template #activator="{ props }">
        <v-btn
          v-bind="props"
          :disabled="item.disabled"
          :icon="item.icon"
          variant="text"
          size="small"
          color="white"
          class="mx-1"
          @click.stop="toggleMenu(index)"
        >
          <span v-if="item.svg" v-html="item.svg"></span>
          <v-icon v-else>{{ item.icon }}</v-icon>
        </v-btn>
      </template>

      <!-- Render dynamic component based on menu selection -->
      <component :guid="props.guid" :is="getMenuComponent(item.name)" />
    </v-menu>
  </v-sheet>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps({
  guid: { type: String, required: true },
  maximized: { type: Boolean, default: true }
});

// Import all menu components
import SettingsMenu from './SettingsMenu.vue'
import CameraMenu from './CameraMenu.vue'
import GuideMenu from './GuideMenu.vue'
import MountMenu from './MountMenu.vue'
import EFWMenu from './EFWMenu.vue'
import EAFMenu from './EAFMenu.vue'
import RotatorMenu from './RotatorMenu.vue'
import StorageMenu from './StorageMenu.vue'
import InfoMenu from './InfoMenu.vue'

const menuIndex = ref<number | null>(null)

function toggleMenu(index: number) {
  menuIndex.value = menuIndex.value === index ? null : index
}

const EFWIcon = `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><rect width="24" height="24" fill="none"/><g fill="none" fill-rule="evenodd"><path d="m12.593 23.258l-.011.002l-.071.035l-.02.004l-.014-.004l-.071-.035q-.016-.005-.024.005l-.004.01l-.017.428l.005.02l.01.013l.104.074l.015.004l.012-.004l.104-.074l.012-.016l.004-.017l-.017-.427q-.004-.016-.017-.018m.265-.113l-.013.002l-.185.093l-.01.01l-.003.011l.018.43l.005.012l.008.007l.201.093q.019.005.029-.008l.004-.014l-.034-.614q-.005-.018-.02-.022m-.715.002a.02.02 0 0 0-.027.006l-.006.014l-.034.614q.001.018.017.024l.015-.002l.201-.093l.01-.008l.004-.011l.017-.43l-.003-.012l-.01-.01z"/><path fill="currentColor" d="M12 2c5.523 0 10 4.477 10 10s-4.477 10-10 10S2 17.523 2 12S6.477 2 12 2m-1.47 13.197l-.747 1.12c-.317.476-.502 1.364.196 1.974c.415.363 1.075.71 2.021.71s1.606-.347 2.021-.71c.698-.61.513-1.498.196-1.973l-.747-1.121a1.767 1.767 0 0 0-2.94 0m6.262-3.971l-1.297.364a1.767 1.767 0 0 0-.909 2.796l.835 1.057c.354.448 1.142.9 1.938.424c.473-.283 1.007-.803 1.3-1.703c.292-.9.166-1.635-.051-2.142c-.364-.853-1.267-.951-1.816-.796m-9.584 0c-.55-.155-1.452-.057-1.816.796c-.217.507-.343 1.241-.05 2.142c.292.9.825 1.42 1.3 1.703c.748.447 1.49.074 1.87-.345l.067-.079l.835-1.057a1.767 1.767 0 0 0-.772-2.752l-.137-.044l-1.297-.365Zm.677-4.89c-.766.556-1.095 1.224-1.218 1.763c-.206.903.466 1.513 1.001 1.711l1.264.468A1.767 1.767 0 0 0 11.31 8.55l-.054-1.347c-.023-.57-.395-1.398-1.318-1.48c-.55-.05-1.288.057-2.053.613m8.23 0c-.765-.556-1.503-.663-2.052-.614c-.87.078-1.25.816-1.311 1.378l-.008.103l-.054 1.347a1.767 1.767 0 0 0 2.247 1.77l.131-.042l1.264-.468c.535-.198 1.207-.808 1.001-1.712c-.122-.538-.452-1.206-1.218-1.762"/></g></svg>`
const EAFIcon = `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path fill="currentColor" d="M8 20H5a1 1 0 0 1-1-1v-3a1 1 0 0 0-2 0v3a3 3 0 0 0 3 3h3a1 1 0 0 0 0-2M3 9a1 1 0 0 0 1-1V5a1 1 0 0 1 1-1h3a1 1 0 0 0 0-2H5a3 3 0 0 0-3 3v3a1 1 0 0 0 1 1m16-7h-3a1 1 0 0 0 0 2h3a1 1 0 0 1 1 1v3a1 1 0 0 0 2 0V5a3 3 0 0 0-3-3m-3 10a1 1 0 0 0-1-1h-2V9a1 1 0 0 0-2 0v2H9a1 1 0 0 0 0 2h2v2a1 1 0 0 0 2 0v-2h2a1 1 0 0 0 1-1m5 3a1 1 0 0 0-1 1v3a1 1 0 0 1-1 1h-3a1 1 0 0 0 0 2h3a3 3 0 0 0 3-3v-3a1 1 0 0 0-1-1"/></svg>`
const RotatorIcon =`<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path fill="currentColor" d="M12.039 23q-2.139 0-4.053-.76q-1.915-.761-3.402-2.104t-2.436-3.173T1 13h1q.2 1.896 1.04 3.526q.839 1.63 2.157 2.83t3.019 1.899t3.615.734l-1.935-1.935l.708-.708l3.462 3.462q-.504.12-1.011.157q-.507.035-1.016.035M15 18v-2H9.616q-.672 0-1.144-.472T8 14.385V9H6V8h2V6h1v8.385q0 .23.192.423t.423.192H19v1h-3v2zm0-4V9.616q0-.231-.192-.424T14.385 9H10V8h4.385q.67 0 1.143.472q.472.472.472 1.144V14zm7-3q-.194-1.896-1.036-3.526t-2.161-2.83t-3.018-1.899t-3.616-.734l1.935 1.935l-.708.708l-3.462-3.462q.505-.12 1.011-.156Q11.452 1 11.962 1q2.138 0 4.053.76q1.914.761 3.401 2.105t2.436 3.173T23 11z"/></svg>`

const menuItems = [
  { name: 'Settings', icon: 'mdi-cog', disabled: true },
  { name: 'Camera', icon: 'mdi-camera-outline', disabled: false },
  { name: 'Guide', icon: 'mdi-crosshairs-gps', disabled: true },
  { name: 'Mount', icon: 'mdi-telescope', disabled: true },
  { name: 'EFW', svg: EFWIcon, disabled: true },
  { name: 'EAF', svg: EAFIcon, disabled: true },
  { name: 'Rotator', svg: RotatorIcon, disabled: true },
  { name: 'Storage', icon: 'mdi-sd', disabled: true },
  { name: 'Info', icon: 'mdi-information-outline', disabled: true }
]

// Function to get the component based on menu name
function getMenuComponent(menuName: string) {
  switch (menuName) {
    case 'Settings':
      return SettingsMenu
    case 'Camera':
      return CameraMenu
    case 'Guide':
      return GuideMenu
    case 'Mount':
      return MountMenu
    case 'EFW':
      return EFWMenu
    case 'EAF':
      return EAFMenu
    case 'Rotator':
      return RotatorMenu
    case 'Storage':
      return StorageMenu
    case 'Info':
      return InfoMenu
    default:
      return null
  }
}
</script>

<style scoped>
.menu-bar {
  background-color: rgba(33, 33, 33, 0.6); /* dark semi-transparent */
  color: white;
  border-radius: 8px;
  position: absolute;
  top: 0;
  width: auto;
  right: 0;
  z-index: 10;
  pointer-events: auto;
}
.hidden-menu {
  display: none !important;
}
</style>
