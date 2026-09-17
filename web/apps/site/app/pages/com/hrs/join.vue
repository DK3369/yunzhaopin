<script setup lang="ts">
const route = useRoute()
await navigateTo({ path: '/com/hrs', query: { ...route.query } })
</script>
