<template>
  <div>
    <p v-if="$fetchState.pending">
      <span class="loading"></span>
    </p>
    <p v-else-if="$fetchState.error">Error while fetching mountains 🤬</p>
    <ul v-else>
      <li v-for="mountain in mountains">
          {{ mountain.value }}
      </li>
    </ul>
    <ui-button @click="$fetch">Refresh Data</ui-button>
  </div>
</template>
<script>
  export default {
    data() {
      return {
        mountains: []
      }
    },
    activated() {
      if (this.$fetchState.timestamp <= Date.now() - 30000) {
        this.$fetch()
      }
    },

    async fetch() {
      this.mountains = await this.$http.$get('http://localhost:8080')
    }
  }
</script>
<style scoped>
  li {
    margin-bottom: 0.5rem;
  }
  li:first-letter {
    text-transform: uppercase;
  }
  .loading {
    display: inline-block;
    width: 1.5rem;
    height: 1.5rem;
    border: 4px solid rgba(9, 133, 81, 0.705);
    border-radius: 50%;
    border-top-color: #158876;
    animation: spin 1s ease-in-out infinite;
  }
  @keyframes spin {
    to {
      -webkit-transform: rotate(360deg);
    }
  }
</style>
