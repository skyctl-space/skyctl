<template>
    <div class="floating-left-panel">
        <v-btn size="small" icon="mdi-chart-histogram" v-tooltip:end="'Histogram'"
            @click="showHistogram = !showHistogram" />
        <v-btn size="small" icon="mdi-focus-field" disabled v-tooltip:end="'Focuser'"
             />
        <v-btn size="small" icon="mdi-circle" disabled v-tooltip:end="'Guiding'">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="-1.5 -1 24 24"><path fill="currentColor" d="M18.913 2.9L2.632 9.226l4.829 2.006a5.77 5.77 0 0 1 3.118 3.119l2.006 4.828zm1.847.682l-6.328 16.281c-.4 1.03-1.551 1.557-2.571 1.18a1.92 1.92 0 0 1-1.11-1.067l-2.007-4.83a3.85 3.85 0 0 0-2.079-2.078l-4.828-2.006C.833 10.645.375 9.486.814 8.472A2.05 2.05 0 0 1 1.949 7.38L18.23 1.052a1.945 1.945 0 0 1 2.53 2.53"/></svg>
        </v-btn>
        <v-btn size="small" icon="mdi-web" disabled v-tooltip:end="'Solve'"
        />
        <v-btn size="small" icon="mdi-circle" disabled v-tooltip:end="'Detect'">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path fill="currentColor" d="M7.657 6.247c.11-.33.576-.33.686 0l.645 1.937a2.89 2.89 0 0 0 1.829 1.828l1.936.645c.33.11.33.576 0 .686l-1.937.645a2.89 2.89 0 0 0-1.828 1.829l-.645 1.936a.361.361 0 0 1-.686 0l-.645-1.937a2.89 2.89 0 0 0-1.828-1.828l-1.937-.645a.361.361 0 0 1 0-.686l1.937-.645a2.89 2.89 0 0 0 1.828-1.828zM3.794 1.148a.217.217 0 0 1 .412 0l.387 1.162c.173.518.579.924 1.097 1.097l1.162.387a.217.217 0 0 1 0 .412l-1.162.387A1.73 1.73 0 0 0 4.593 5.69l-.387 1.162a.217.217 0 0 1-.412 0L3.407 5.69A1.73 1.73 0 0 0 2.31 4.593l-1.162-.387a.217.217 0 0 1 0-.412l1.162-.387A1.73 1.73 0 0 0 3.407 2.31zM10.863.099a.145.145 0 0 1 .274 0l.258.774c.115.346.386.617.732.732l.774.258a.145.145 0 0 1 0 .274l-.774.258a1.16 1.16 0 0 0-.732.732l-.258.774a.145.145 0 0 1-.274 0l-.258-.774a1.16 1.16 0 0 0-.732-.732L9.1 2.137a.145.145 0 0 1 0-.274l.774-.258c.346-.115.617-.386.732-.732z"/></svg>
        </v-btn>
        <v-btn size="small"  icon="mdi-circle" v-tooltip:end="'Annotate'">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><g fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" color="currentColor"><path d="M17.734 17.734c4.524-4.524 5.624-10.759 2.458-13.926C17.627 1.244 13.05 1.478 9 4.062m-5.192 16.13c2.478 2.478 6.835 2.343 10.78 0M6.266 6.266C2.98 9.552 1.5 13.74 2.15 17"/><path d="M16.915 7.085c-1.9-1.9-5.641-1.24-8.355 1.475s-3.375 6.455-1.475 8.355s5.641 1.24 8.356-1.474C17.09 13.79 17.98 11.764 18 10"/><path d="M13.638 10.362c.634.633.414 1.88-.491 2.785s-2.152 1.125-2.785.491c-.634-.633-.414-1.88.491-2.785s2.152-1.125 2.785-.491"/></g></svg>    
        </v-btn>
        <v-btn size="small" icon="mdi-crosshairs" v-tooltip:end="'Crosshair'"
            @click="showCrosshair = !showCrosshair"
        />

        <v-btn size="small" icon="mdi-circle" v-tooltip:end="'Stellarium'" to="/stellarium">
            <img src="@/assets/images/logo.svg" alt="Logo" style="width: 24px; height: 24px;" />
        </v-btn>
    </div>

</template>

<script setup lang="ts">
const showHistogram = defineModel<boolean>('showHistogram', { required: true });
const showCrosshair = defineModel<boolean>('showCrosshair', { required: true });

</script>

<style scoped>
.floating-left-panel {
    position: absolute;
    top: 50%;
    left: 5px;
    transform: translateY(-50%);
    z-index: 999;
    background-color: rgba(0, 0, 0, 0.1);
    border-radius: 8px;
    padding: 8px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    overflow: hidden;
}

.floating-left-panel v-btn {
    color: white;
}
</style>