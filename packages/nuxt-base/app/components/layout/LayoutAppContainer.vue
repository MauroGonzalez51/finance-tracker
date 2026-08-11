<script setup lang="ts">
    const modal = useModal();
</script>

<template>
    <IonPage>
        <template v-if="modal.state.value.kind === 'ion-action-sheet'">
            <IonActionSheet
                v-bind="modal.state.value.props"
                :is-open="modal.state.value.open"
                @did-dismiss="() => modal.dispatch.setOpen(false)"
            />
        </template>

        <template v-if="modal.state.value.kind === 'ion-alert'">
            <IonAlert
                v-bind="modal.state.value.props"
                :is-open="modal.state.value.open"
                @did-dismiss="() => modal.dispatch.setOpen(false)"
            />
        </template>

        <template v-if="modal.state.value.kind === 'ion-loading'">
            <IonLoading
                v-bind="modal.state.value.props"
                :is-open="modal.state.value.open"
                @did-dismiss="() => modal.dispatch.setOpen(false)"
            />
        </template>

        <template v-if="modal.state.value.kind === 'ion-modal'">
            <component
                :is="modal.component.value"
                :is-open="modal.state.value.open"
                @did-dismiss="() => modal.dispatch.setOpen(false)"
            />
        </template>

        <slot />
    </IonPage>
</template>
