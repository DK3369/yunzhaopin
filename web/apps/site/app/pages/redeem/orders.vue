<script setup lang="ts">
const { me } = useSiteChrome()
if (!me.value) {
  await navigateTo({ path: '/login', query: { next: '/redeem/orders' } })
} else {
  await navigateTo(Number(me.value.usertype) === 2 ? '/com/rewards' : '/user/rewards')
}
</script>
