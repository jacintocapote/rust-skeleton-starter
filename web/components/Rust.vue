<template>
  <div>
    <p v-if="$fetchState.pending">
      <span class="loading"></span>
    </p>
    <p v-else-if="$fetchState.error">Error while fetching Rust Endpoint 🤬</p>
    <ul v-else>
      <li v-for="product in products">
        {{ product.id }}
        {{ product.price }}
        {{ product.title }}
        <span v-html="product.body"></span>
        {{ product.published }}
      </li>
    </ul>
    <ui-button @click="$fetch">Refresh Data</ui-button>
  </div>
</template>
<script>
  export default {
    data() {
      return {
        products: []
      }
    },
    activated() {
      if (this.$fetchState.timestamp <= Date.now() - 30000) {
        this.$fetch()
      }
    },

    async fetch() {
      this.products = await this.$http.$get('http://localhost:3001/api')
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
